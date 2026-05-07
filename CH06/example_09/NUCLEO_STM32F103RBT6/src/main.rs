#![no_std]
#![no_main]

use cortex_m::peripheral::{syst::SystClkSource, SYST};
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

const SYSTICK_RELOAD: u32 = 0x00FF_FFFF; // 24-bit maximum

fn do_work() {
    for _ in 0..10_000 {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting SysTick profiling...");

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;

    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(SYSTICK_RELOAD);
    syst.clear_current();
    syst.enable_counter();

    let start = SYST::get_current();
    do_work();
    let end = SYST::get_current();

    // SysTick counts down, so start >= end for non-wrapping intervals.
    // The mask keeps the result within the 24-bit counter range.
    let elapsed = start.wrapping_sub(end) & SYSTICK_RELOAD;
    rprintln!("do_work() took {} cycles", elapsed);

    loop {}
}
