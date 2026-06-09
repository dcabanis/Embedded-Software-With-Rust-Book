#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_alloc::LlffHeap as Heap;
use panic_probe as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Reserve 4 KiB of RAM for the heap.  MaybeUninit avoids zeroing
    // the region at startup — the allocator overwrites it on first use.
    const HEAP_SIZE: usize = 4096;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

    // Hand the backing store to the allocator.  Must happen before any
    // code that calls Box::new, Vec::push, String::from, etc.
    unsafe {
        HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE);
    }

    // From this point on the full alloc API is available.
    let x = Box::new(1986u32);
    defmt::info!("Box::new succeeded, value = {}", *x);
    drop(x); // returned to the heap

    let mut v: Vec<u32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    defmt::info!("Vec len={}, values=[{}, {}, {}]", v.len(), v[0], v[1], v[2]);

    loop {}
}
