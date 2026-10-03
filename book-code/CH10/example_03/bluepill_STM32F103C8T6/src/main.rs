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

bind_interrupts!(struct Irqs {
    EXTI3 => embassy_stm32::exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI3>;
});

#[embassy_executor::task]
async fn stop_task(pin: Peri<'static, peripherals::PB3>, exti: Peri<'static, peripherals::EXTI3>) {
    let mut stop = ExtiInput::new(pin, exti, Pull::Up, Irqs);
    // Also recognize a button already held down when the task starts.
    stop.wait_for_low().await;

    HEARTBEAT.lock(|cell| {
        let mut state = cell.borrow_mut();
        let heartbeat = state.as_mut().unwrap();
        heartbeat.stopped = true;
        heartbeat.led.set_high(); // PB8 high means LED off (active-low).
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
                heartbeat.led.set_low(); // Begin the lit interval.
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
            cell.borrow_mut().as_mut().unwrap().led.set_high();
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

    // PB3 defaults to a JTAG signal on this chip (JTDO), so it must be
    // released before it behaves as plain GPIO. This also frees PA15 and
    // PB4 (JTDI and JTRST) as a side effect of the same call, even though
    // this example does not use them. SWD debugging (what probe-rs uses
    // here) keeps working afterward. embassy-stm32 has no higher-level API
    // for this, so the AFIO register is written directly through
    // stm32-metapac, the PAC crate embassy-stm32 itself is built on.
    stm32_metapac::AFIO
        .mapr()
        .modify(|w| w.set_swj_cfg(stm32_metapac::afio::vals::SwjCfg::JTAG_DISABLE));

    hprintln!("heartbeat demo ready");

    HEARTBEAT.lock(|cell| {
        cell.replace(Some(Heartbeat {
            led: Output::new(p.PB8, Level::High, Speed::Low),
            stopped: false,
        }));
    });

    interrupt::USART2.set_priority(Priority::P6);
    let high_spawner = EXECUTOR_HIGH.start(interrupt::USART2);
    high_spawner.spawn(stop_task(p.PB3, p.EXTI3).unwrap());

    let executor = EXECUTOR_LOW.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(heartbeat_task().unwrap());
    });
}
