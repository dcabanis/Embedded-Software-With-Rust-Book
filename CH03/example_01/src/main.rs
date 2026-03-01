#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f1xx_hal as _; // kept intentionally for upcoming HAL-based examples

#[entry]
fn main() -> ! {
    let _ = hprintln!("Hello, world!");

    loop {
        cortex_m::asm::nop();
    }
}
