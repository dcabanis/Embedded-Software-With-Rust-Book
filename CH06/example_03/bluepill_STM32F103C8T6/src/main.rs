#![no_std]
#![no_main]

use cortex_m::{iprintln, Peripherals};
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut cp = Peripherals::take().unwrap();
    iprintln!(&mut cp.ITM.stim[0], "ITM log message");
    loop {}
}
