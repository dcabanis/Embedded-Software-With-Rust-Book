#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_alloc::LlffHeap as Heap;
use panic_probe as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn record_event(log: &mut Vec<String>, source: &str, code: u32) {
    log.push(format!("{}: code 0x{:08x}", source, code));
}

#[entry]
fn main() -> ! {
    const HEAP_SIZE: usize = 4096;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE); }

    let mut log: Vec<String> = Vec::new();

    record_event(&mut log, "uart", 0x0001);
    record_event(&mut log, "i2c",  0x002a);
    record_event(&mut log, "gpio", 0x00ff);

    for entry in &log {
        defmt::info!("{}", entry.as_str());
    }

    loop {}
}
