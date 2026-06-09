#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;
use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

// The const generic parameter is the ORDER: this heap supports blocks
// from 2^0 to 2^31 bytes.  Allocations are rounded up to the next
// power of two (internal fragmentation), in exchange for bounded
// worst-case timing and predictable fragmentation behaviour.
#[global_allocator]
static HEAP: LockedHeap<32> = LockedHeap::empty();

#[entry]
fn main() -> ! {
    const HEAP_SIZE: usize = 4096;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

    unsafe {
        HEAP.lock().init(&raw mut HEAP_MEM as *mut u8 as usize, HEAP_SIZE);
    }

    let x = Box::new(1986u32);
    defmt::info!("Buddy heap: Box value = {}", *x);
    drop(x);

    let mut v: Vec<u32> = Vec::new();
    v.push(10);
    v.push(20);
    defmt::info!("Buddy heap: Vec len={}, values=[{}, {}]", v.len(), v[0], v[1]);

    loop {}
}
