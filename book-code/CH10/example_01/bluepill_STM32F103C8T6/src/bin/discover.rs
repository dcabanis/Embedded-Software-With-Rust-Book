#![no_std]
#![no_main]

//! One-off hardware discovery tool — not part of the book example.
//!
//! Configures the four candidate button pins (PB3, PB4, PB5, PA15) as
//! internal-pull-up inputs and prints their live state whenever it changes,
//! and cycles the four candidate LED pins (PB8, PB9, PA9, PA10) through
//! HIGH and LOW, one at a time, printing which pin/level is active. Watch
//! the board while this runs and note, for each LED, which level (HIGH or
//! LOW) turns it on, and for each button, whether pressing it reads as 0
//! (matches the pull-up assumption) or does nothing (needs a different
//! pull configuration instead).
//!
//! PB3, PB4, and PA15 default to JTAG pin functions on this chip, so they
//! are released via `AFIO.mapr.disable_jtag()` before use — otherwise they
//! would not behave as ordinary GPIO at all.

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f1xx_hal::{
    gpio::{gpioa::PA15, gpiob::{PB3, PB4, PB5}, Input, PullUp},
    pac,
    prelude::*,
};

const SYSCLK_HZ: u32 = 8_000_000;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    dp.DBGMCU.cr().modify(|_, w| w.dbg_sleep().set_bit());

    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc);
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut gpiob = dp.GPIOB.split(&mut rcc);

    // PB3, PB4, PA15 start up as JTAG signals; free them for plain GPIO use.
    let (pa15, pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    let btn_pb3 = pb3.into_pull_up_input(&mut gpiob.crl);
    let btn_pb4 = pb4.into_pull_up_input(&mut gpiob.crl);
    let btn_pb5 = gpiob.pb5.into_pull_up_input(&mut gpiob.crl);
    let btn_pa15 = pa15.into_pull_up_input(&mut gpioa.crh);

    let mut led_pb8 = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);
    let mut led_pb9 = gpiob.pb9.into_push_pull_output(&mut gpiob.crh);
    let mut led_pa9 = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
    let mut led_pa10 = gpioa.pa10.into_push_pull_output(&mut gpioa.crh);

    hprintln!("=== GPIO discovery tool ===");
    hprintln!("Buttons use an internal pull-up: idle = 1, a normal active-low");
    hprintln!("press should read 0. Report any pin that behaves differently.");
    hprintln!("Watching the board, note which level (HIGH/LOW) lights each LED.");

    let mut last = [true; 4];

    loop {
        led_pb8.set_high();
        hprintln!("--- PB8 = HIGH ---");
        poll_buttons(&btn_pb3, &btn_pb4, &btn_pb5, &btn_pa15, &mut last);
        led_pb8.set_low();
        hprintln!("--- PB8 = LOW ---");
        poll_buttons(&btn_pb3, &btn_pb4, &btn_pb5, &btn_pa15, &mut last);

        led_pb9.set_high();
        hprintln!("--- PB9 = HIGH ---");
        poll_buttons(&btn_pb3, &btn_pb4, &btn_pb5, &btn_pa15, &mut last);
        led_pb9.set_low();
        hprintln!("--- PB9 = LOW ---");
        poll_buttons(&btn_pb3, &btn_pb4, &btn_pb5, &btn_pa15, &mut last);

        led_pa9.set_high();
        hprintln!("--- PA9 = HIGH ---");
        poll_buttons(&btn_pb3, &btn_pb4, &btn_pb5, &btn_pa15, &mut last);
        led_pa9.set_low();
        hprintln!("--- PA9 = LOW ---");
        poll_buttons(&btn_pb3, &btn_pb4, &btn_pb5, &btn_pa15, &mut last);

        led_pa10.set_high();
        hprintln!("--- PA10 = HIGH ---");
        poll_buttons(&btn_pb3, &btn_pb4, &btn_pb5, &btn_pa15, &mut last);
        led_pa10.set_low();
        hprintln!("--- PA10 = LOW ---");
        poll_buttons(&btn_pb3, &btn_pb4, &btn_pb5, &btn_pa15, &mut last);
    }
}

// Samples all four button pins for about one second, printing immediately
// whenever the combined state changes so a press is visible right away.
fn poll_buttons(
    pb3: &PB3<Input<PullUp>>,
    pb4: &PB4<Input<PullUp>>,
    pb5: &PB5<Input<PullUp>>,
    pa15: &PA15<Input<PullUp>>,
    last: &mut [bool; 4],
) {
    for _ in 0..50 {
        let now = [pb3.is_high(), pb4.is_high(), pb5.is_high(), pa15.is_high()];
        if now != *last {
            hprintln!(
                "buttons: pb3={} pb4={} pb5={} pa15={}",
                now[0] as u8,
                now[1] as u8,
                now[2] as u8,
                now[3] as u8
            );
            *last = now;
        }
        delay(SYSCLK_HZ / 50);
    }
}
