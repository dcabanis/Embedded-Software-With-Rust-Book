#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[inline(never)]
fn slow_increment(start: u32, iterations: u32) -> u32 {
    let mut counter = start;
    for _ in 0..iterations {
        counter = counter.wrapping_add(1);
        cortex_m::asm::nop();
    }
    counter
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("Starting slow_increment from 0, 10 iterations...");
    let result = slow_increment(0, 10);
    rprintln!("Result: {}", result);

    loop {
        cortex_m::asm::nop();
    }
}
