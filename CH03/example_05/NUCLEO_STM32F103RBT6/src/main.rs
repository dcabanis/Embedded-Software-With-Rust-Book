#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use stm32f1xx_hal::{
    pac,
    prelude::*,
};

// MCU-agnostic driver logic: only depends on embedded-hal traits.
fn blink_led<P, D>(led: &mut P, delay: &mut D) -> !
where
    P: OutputPin,
    D: DelayNs,
{
    loop {
        let _ = led.set_high();
        delay.delay_ms(500);

        let _ = led.set_low();
        delay.delay_ms(500);
    }
}

#[entry]
fn main() -> ! {
    // --- Take peripherals ---------------------------------------------------
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // --- Clocks -------------------------------------------------------------
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Typical STM32F103 clocking (8 MHz HSE -> 72 MHz SYSCLK).
    rcc = rcc.freeze(
        stm32f1xx_hal::rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz()),
        &mut flash.acr,
    );
    let clocks = rcc.clocks;

    // --- GPIO: NUCLEO-F103RB user LED (LD2 on PA5) -------------------------
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    // --- Delay provider -----------------------------------------------------
    let mut delay = cp.SYST.delay(&clocks);

    // --- Use the generic function ------------------------------------------
    blink_led(&mut led, &mut delay);
}
