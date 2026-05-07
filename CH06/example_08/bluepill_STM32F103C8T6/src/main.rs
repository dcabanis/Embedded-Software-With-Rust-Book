#![no_std]
#![no_main]

use cortex_m::peripheral::DWT;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

fn do_task_a() {
    for _ in 0..500_000 {
        cortex_m::asm::nop();
    }
}

fn do_task_b() {
    for _ in 0..1_000_000 {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting DWT profiling...");

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut dcb = cp.DCB;
    let mut dwt = cp.DWT;

    // On some Cortex-M7 implementations the DWT is locked at reset.
    // DWT::unlock() is a no-op on parts where it is not required (e.g. M3).
    DWT::unlock();
    dcb.enable_trace();
    dwt.enable_cycle_counter();

    let start_a = DWT::cycle_count();
    do_task_a();
    let elapsed_a = DWT::cycle_count().wrapping_sub(start_a);
    rprintln!("Task A took {} cycles", elapsed_a);

    let start_b = DWT::cycle_count();
    do_task_b();
    let elapsed_b = DWT::cycle_count().wrapping_sub(start_b);
    rprintln!("Task B took {} cycles", elapsed_b);

    loop {}
}
