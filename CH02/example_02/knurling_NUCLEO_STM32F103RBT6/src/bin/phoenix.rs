#![no_main]
#![no_std]

use knurling_nucleo_stm32f103_rb as _; // library crate
use stm32f1xx_hal as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Rise like the phoenix!");

    knurling_nucleo_stm32f103_rb::exit()
}