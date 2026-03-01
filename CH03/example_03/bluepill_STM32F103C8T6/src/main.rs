#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception};
use panic_halt as _;
// Required by the cortex-m-rt crate
use stm32f1xx_hal as _;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;

    syst.set_reload(8_000_000); // 1 second delay (assuming 8 MHz clock)
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    loop {}
}

#[exception]
fn SysTick() {
    // This function is called on each SysTick interrupt
}
