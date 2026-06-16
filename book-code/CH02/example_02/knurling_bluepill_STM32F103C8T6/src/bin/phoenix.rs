#![no_main]
#![no_std]

use knurling_bluepill as _; // library crate
use stm32f1xx_hal as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Rise like the Phoenix!");

    knurling_bluepill::exit()
}