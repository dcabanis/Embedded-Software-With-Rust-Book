#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use panic_halt as _;
use stm32f1xx_hal as _;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;

    // Default reset clock on STM32F103 is HSI = 8 MHz.
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000 - 1);
    syst.clear_current();
    syst.enable_interrupt();
    syst.enable_counter();

    loop {
        asm::wfi();
    }
}

#[exception]
fn SysTick() {}
