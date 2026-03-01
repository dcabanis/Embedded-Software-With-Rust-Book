#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{ErrorType, OutputPin};

use stm32f1xx_hal::{
    pac,
    prelude::*,
};

/// MCU-agnostic driver logic: only depends on embedded-hal traits.
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

    // Typical Blue Pill clocking (8 MHz HSE -> 72 MHz SYSCLK).
    rcc = rcc.freeze(
        stm32f1xx_hal::rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz()),
        &mut flash.acr,
    );
    let clocks = rcc.clocks;

    // --- GPIO: PC13 LED -----------------------------------------------------
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // Blue Pill LED on PC13 is commonly wired active-low.
    // That means "set_low" turns it ON, and "set_high" turns it OFF.
    // To keep blink_led() generic (HIGH=on, LOW=off), we wrap the pin
    // in a tiny adapter that inverts the polarity.
    let raw_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut led = ActiveLow(raw_led);

    // --- Delay provider -----------------------------------------------------
    let mut delay = cp.SYST.delay(&clocks);

    // --- Use the generic function ------------------------------------------
    blink_led(&mut led, &mut delay);
}

/// Small adapter that makes an active-low output pin look active-high.
/// This keeps the platform-agnostic blink_led() logic readable.
struct ActiveLow<P>(P);

impl<P> OutputPin for ActiveLow<P>
where
    P: OutputPin,
{
    fn set_high(&mut self) -> Result<(), Self::Error> {
        // "High" for the logical LED means drive the pin low electrically.
        self.0.set_low()
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        // "Low" for the logical LED means drive the pin high electrically.
        self.0.set_high()
    }
}

impl<P> ErrorType for ActiveLow<P>
where
    P: OutputPin,
{
    type Error = P::Error;
}
