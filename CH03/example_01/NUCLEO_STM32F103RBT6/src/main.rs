#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f1xx_hal as _; 
use cortex_m_semihosting::hprintln;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let _ = hprintln!("Hello, world!");

    loop {
        cortex_m::asm::nop();
    }
}
