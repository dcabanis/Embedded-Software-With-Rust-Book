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

// EXTI4 has its own IRQ vector on this chip, but EXTI13 shares the
// EXTI15_10 vector with every other line in that range — see the
// bluepill_STM32F103C8T6 port's comment for why EXTI3/EXTI4 didn't need
// this there.
bind_interrupts!(struct Irqs {
    EXTI15_10 => embassy_stm32::exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI15_10>;
    EXTI4 => embassy_stm32::exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI4>;
});

#[embassy_executor::task]
async fn fan_task(
    button_pin: Peri<'static, peripherals::PC13>,
    tach_pin: Peri<'static, peripherals::PB4>,
    button_exti: Peri<'static, peripherals::EXTI13>,
    tach_exti: Peri<'static, peripherals::EXTI4>,
    led_pin: Peri<'static, peripherals::PA5>,
) {
    // B1, the onboard USER button, is already pulled up on the Nucleo
    // board and pulls PC13 to ground when pressed, so no internal pull is
    // requested here — matching example_01's NUCLEO port.
    let mut button = ExtiInput::new(button_pin, button_exti, Pull::None, Irqs);
    // Stands in for a real tachometer, on the same pin used for this role
    // on the bluepill_STM32F103C8T6 port: press repeatedly within 200 ms
    // to simulate a running motor's pulses, or stop pressing to simulate
    // a stall. External wiring required — see README.
    let mut tach = ExtiInput::new(tach_pin, tach_exti, Pull::Up, Irqs);
    // LD2 on the NUCLEO-F103RB: active-high, opposite polarity from the
    // bluepill port's PB8.
    let mut led = Output::new(led_pin, Level::Low, Speed::Low);
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
    // Active-high LED: HIGH = on, LOW = off.
    if fault {
        led.set_high();
    } else {
        led.set_low();
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // PB4 defaults to a JTAG signal on this chip (JTRST), so it must be
    // released before it behaves as plain GPIO. This also frees PA15 and
    // PB3 (JTDI and JTDO) as a side effect of the same call, even though
    // this example does not use them. SWD debugging (what probe-rs uses
    // here) keeps working afterward. embassy-stm32 has no higher-level API
    // for this, so the AFIO register is written directly through
    // stm32-metapac, the PAC crate embassy-stm32 itself is built on.
    stm32_metapac::AFIO
        .mapr()
        .modify(|w| w.set_swj_cfg(stm32_metapac::afio::vals::SwjCfg::JTAG_DISABLE));

    hprintln!("fan controller ready");

    spawner.spawn(fan_task(p.PC13, p.PB4, p.EXTI13, p.EXTI4, p.PA5).unwrap());
}
