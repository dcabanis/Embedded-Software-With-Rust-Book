#![no_std]
#![no_main]

extern crate alloc;

use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_alloc::LlffHeap as Heap;
use panic_probe as _;
use smallvec::SmallVec;
use tinyvec::TinyVec;

// Both TinyVec and SmallVec spill to the heap when their inline capacity
// is exceeded.  A global allocator must be installed before that happens.
#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    const HEAP_SIZE: usize = 4096;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe {
        HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE);
    }

    // --- tinyvec::TinyVec ---
    // TinyVec<[T; N]> stores the first N elements inline (no heap).
    // The element type must implement Default (u16 does, trivially).
    let mut tv: TinyVec<[u16; 8]> = TinyVec::new();
    for value in [10u16, 20, 30, 40, 50, 60, 70, 80] {
        tv.push(value); // stays inline for the first 8 items
    }
    defmt::info!("TinyVec: len={} is_heap={}", tv.len(), tv.is_heap());

    tv.push(90); // 9th item triggers heap spill: inline array is copied to a Vec<u16>
    defmt::info!("TinyVec after spill: len={} is_heap={}", tv.len(), tv.is_heap());

    // --- smallvec::SmallVec ---
    // SmallVec<[T; N]> behaves the same but uses unsafe internally, which
    // removes the T: Default requirement.  Use when T has no sensible default.
    let mut sv: SmallVec<[u16; 8]> = SmallVec::new();
    for value in [512u16, 1024, 2048, 4096, 8192, 16384, 32768, 1] {
        sv.push(value); // stays inline for the first 8 items
    }
    defmt::info!("SmallVec: len={} spilled={}", sv.len(), sv.spilled());

    sv.push(1986); // 9th item triggers heap spill
    defmt::info!("SmallVec after spill: len={} spilled={}", sv.len(), sv.spilled());

    loop {}
}
