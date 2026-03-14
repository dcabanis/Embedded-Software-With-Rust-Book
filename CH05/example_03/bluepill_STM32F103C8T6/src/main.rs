#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc::Config};

#[entry]
fn main() -> ! {
    // Take ownership of the singleton core and device peripherals
    // `unwrap()` is safe here because `#[entry]` guarantees single invocation
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain the RCC and FLASH peripherals, then freeze the default clock configuration
    // freeze() consumes `Rcc` and returns it with `clocks` populated
    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = rcc.freeze(Config::DEFAULT, &mut flash.acr);

    // Copy the frozen clock frequencies for the SysTick delay provider
    let clocks = rcc.clocks;

    // Build a HAL delay provider backed by SysTick, configured for the frozen clocks
    let mut delay = cp.SYST.delay(&clocks);

    // GPIOC / PC13 (typical Blue Pill LED, active-low)
    // split() takes &mut Rcc which derefs to the PAC RCC to enable the GPIO clock
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // Configure PC13 as push-pull output
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    loop {
        // Drive PC13 high (LED OFF on many Blue Pill boards as the LED is wired active-low)
        led.set_high();
        delay.delay_ms(1000_u32);

        // Drive PC13 low (LED ON)
        led.set_low();
        delay.delay_ms(1000_u32);
    }
}
