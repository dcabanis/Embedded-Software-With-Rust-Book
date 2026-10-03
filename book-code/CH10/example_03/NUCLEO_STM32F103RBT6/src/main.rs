#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m_semihosting::hprintln;
use embassy_executor::{Executor, InterruptExecutor};
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::interrupt::{InterruptExt, Priority};
use embassy_stm32::{bind_interrupts, interrupt, peripherals, Peri};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Instant, Timer};
use panic_halt as _;
use static_cell::StaticCell;

struct Heartbeat {
    led: Output<'static>,
    stopped: bool,
}

static HEARTBEAT: Mutex<CriticalSectionRawMutex, RefCell<Option<Heartbeat>>> =
    Mutex::new(RefCell::new(None));

// Executors with different priorities.
static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR_LOW: StaticCell<Executor> = StaticCell::new();

// EXTI13 shares the EXTI15_10 vector with every other line in that range
// on this chip — see the example_02 NUCLEO port's comment for details.
bind_interrupts!(struct Irqs {
    EXTI15_10 => embassy_stm32::exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI15_10>;
});

#[embassy_executor::task]
async fn stop_task(
    pin: Peri<'static, peripherals::PC13>,
    exti: Peri<'static, peripherals::EXTI13>,
) {
    // B1, the onboard USER button, is already pulled up on the Nucleo
    // board, so no internal pull is requested here — matching examples
    // 01/02's NUCLEO ports.
    let mut stop = ExtiInput::new(pin, exti, Pull::None, Irqs);
    // Also recognize a button already held down when the task starts.
    stop.wait_for_low().await;

    HEARTBEAT.lock(|cell| {
        let mut state = cell.borrow_mut();
        let heartbeat = state.as_mut().unwrap();
        heartbeat.stopped = true;
        heartbeat.led.set_low(); // LD2 low means LED off (active-high).
    });
    hprintln!("stop button pressed, heartbeat halted");
    // This one-shot task ends; the shared flag retains the stop condition.
}

#[embassy_executor::task]
async fn heartbeat_task() {
    loop {
        let running = HEARTBEAT.lock(|cell| {
            let mut state = cell.borrow_mut();
            let heartbeat = state.as_mut().unwrap();
            if heartbeat.stopped {
                false
            } else {
                heartbeat.led.set_high(); // Begin the lit interval.
                true
            }
        });

        if !running {
            hprintln!("heartbeat task ending");
            return; // Resetting the board is the only restart in this demo.
        }

        // Deliberately occupy this executor without yielding.
        // Interrupts remain enabled throughout this loop.
        let end = Instant::now() + Duration::from_millis(500);
        while Instant::now() < end {
            core::hint::spin_loop();
        }

        HEARTBEAT.lock(|cell| {
            cell.borrow_mut().as_mut().unwrap().led.set_low();
        });
        Timer::after_millis(500).await; // The dark interval can yield.
    }
}

// USART2 is reserved for the high-priority executor, not UART traffic.
#[interrupt]
unsafe fn USART2() {
    unsafe { EXECUTOR_HIGH.on_interrupt() }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());

    hprintln!("heartbeat demo ready");

    HEARTBEAT.lock(|cell| {
        cell.replace(Some(Heartbeat {
            led: Output::new(p.PA5, Level::Low, Speed::Low),
            stopped: false,
        }));
    });

    interrupt::USART2.set_priority(Priority::P6);
    let high_spawner = EXECUTOR_HIGH.start(interrupt::USART2);
    high_spawner.spawn(stop_task(p.PC13, p.EXTI13).unwrap());

    let executor = EXECUTOR_LOW.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(heartbeat_task().unwrap());
    });
}
