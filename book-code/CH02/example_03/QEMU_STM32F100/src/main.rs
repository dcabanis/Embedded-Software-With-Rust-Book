#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f1xx_hal as _;

#[entry]
fn main() -> ! {
    hprintln!("Fly like a bird!");
    loop {}
}
