#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*, rcc::Config};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut rcc = rcc.freeze(Config::DEFAULT, &mut flash.acr);
    let clocks = rcc.clocks;

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut delay = cp.SYST.delay(&clocks);

    hprintln!("Application starting...");

    loop {
        // On Bluepill boards the PC13 LED is active-low.
        led.set_low();
        delay.delay_ms(500_u16);
        led.set_high();
        delay.delay_ms(500_u16);
    }
}
