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

    // GPIOA / PA5 (NUCLEO LD2 user LED, active-high)
    // split() takes &mut Rcc which derefs to the PAC RCC to enable the GPIO clock
    let mut gpioa = dp.GPIOA.split(&mut rcc);

    // Configure PA5 as push-pull output (PA5 is in the CRL register, pins 0-7)
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    loop {
        // Drive PA5 high (LED ON on NUCLEO as the LED is wired active-high)
        led.set_high();
        delay.delay_ms(1000_u32);

        // Drive PA5 low (LED OFF)
        led.set_low();
        delay.delay_ms(1000_u32);
    }
}
