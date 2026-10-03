#![no_std]
#![no_main]

use cortex_m_semihosting::hprintln;
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::{bind_interrupts, peripherals, Peri};
use embassy_time::Timer;
use panic_halt as _;

use app::{Event, Fan, FanState};

// EXTI3 and EXTI4 each have their own IRQ vector on this chip (unlike the
// shared EXTI9_5/EXTI15_10 vectors used by the other lines), so they're
// bound individually here.
bind_interrupts!(struct Irqs {
    EXTI3 => embassy_stm32::exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI3>;
    EXTI4 => embassy_stm32::exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI4>;
});

#[embassy_executor::task]
async fn fan_task(
    button_pin: Peri<'static, peripherals::PB3>,
    tach_pin: Peri<'static, peripherals::PB4>,
    button_exti: Peri<'static, peripherals::EXTI3>,
    tach_exti: Peri<'static, peripherals::EXTI4>,
    led_pin: Peri<'static, peripherals::PB8>,
) {
    // Both buttons on this board read active-low with an internal pull-up
    // (idle high, pressed low) — the opposite polarity from the pull-down
    // circuit in section 8.3.2's text, so presses are falling edges here.
    let mut button = ExtiInput::new(button_pin, button_exti, Pull::Up, Irqs);
    // Stands in for a real tachometer: press repeatedly within 200 ms to
    // simulate a running motor's pulses, or stop pressing to simulate a
    // stall. The LED is active-low, so Level::High is "off" at startup.
    let mut tach = ExtiInput::new(tach_pin, tach_exti, Pull::Up, Irqs);
    let mut led = Output::new(led_pin, Level::High, Speed::Low);
    let mut fan = Fan::new();

    // Establish the initial Off output before waiting for an event.
    apply_state(fan.state(), &mut led);

    loop {
        let event = match fan.state() {
            FanState::Off | FanState::Stalled => {
                button.wait_for_falling_edge().await;
                Event::ButtonPress
            }
            FanState::Low | FanState::Medium | FanState::High => {
                // Wait for either a button press or a detected stall.
                match select(button.wait_for_falling_edge(), watch_for_stall(&mut tach)).await {
                    Either::First(_) => Event::ButtonPress,
                    Either::Second(_) => Event::StallDetected,
                }
            }
        };

        fan.handle(event);
        apply_state(fan.state(), &mut led);
    }
}

async fn watch_for_stall(tach: &mut ExtiInput<'static, Async>) {
    loop {
        // A pulse restarts the timeout; a timeout ends the monitor.
        match select(tach.wait_for_falling_edge(), Timer::after_millis(200)).await {
            Either::First(_) => continue,
            Either::Second(_) => return,
        }
    }
}

fn apply_state(state: FanState, led: &mut Output<'static>) {
    match state {
        FanState::Off => set_outputs(0, false, led),
        FanState::Low => set_outputs(1, false, led),
        FanState::Medium => set_outputs(2, false, led),
        FanState::High => set_outputs(3, false, led),
        FanState::Stalled => set_outputs(0, true, led),
    }
}

fn set_outputs(speed: u8, fault: bool, led: &mut Output<'static>) {
    hprintln!("fan speed = {}", speed);
    // Active-low LED: LOW = on, HIGH = off.
    if fault {
        led.set_low();
    } else {
        led.set_high();
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // PB3 and PB4 default to JTAG signals on this chip (JTDO and JTRST),
    // so they must be released before they behave as plain GPIO. SWD
    // debugging (what probe-rs uses here) keeps working afterward — only
    // the extra JTAG-only pins go away. embassy-stm32 has no higher-level
    // API for this, so the AFIO register is written directly through
    // stm32-metapac, the PAC crate embassy-stm32 itself builds on.
    stm32_metapac::AFIO
        .mapr()
        .modify(|w| w.set_swj_cfg(stm32_metapac::afio::vals::SwjCfg::JTAG_DISABLE));

    hprintln!("fan controller ready");

    spawner.spawn(fan_task(p.PB3, p.PB4, p.EXTI3, p.EXTI4, p.PB8).unwrap());
}
