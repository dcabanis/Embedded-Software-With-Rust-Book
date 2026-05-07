#![no_std]
#![no_main]

use cortex_m as _;
use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    loop {
        info!("Structured log from defmt");
        // ~1 second pause at the 8 MHz HSI default clock.
        cortex_m::asm::delay(8_000_000);
    }
}
