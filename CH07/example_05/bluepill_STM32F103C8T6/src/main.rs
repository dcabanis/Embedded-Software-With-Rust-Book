#![no_std]
#![no_main]

use arrayvec::{ArrayString, ArrayVec};
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    // --- ArrayVec: panicking push (safe for code paths that cannot overflow) ---
    let mut buffer: ArrayVec<u8, 16> = ArrayVec::new();
    for i in 0u8..16 {
        buffer.push(i); // panics if capacity is exceeded
    }
    defmt::info!("ArrayVec: {} elements, last = {}", buffer.len(), buffer[15]);

    // --- ArrayVec: fallible try_push (safe for input-driven paths) ---
    match buffer.try_push(0x55) {
        Ok(_) => defmt::info!("try_push succeeded (unexpected)"),
        Err(e) => defmt::info!(
            "try_push rejected: buffer full, value = {}",
            e.element()
        ),
    }

    // --- ArrayString: panicking push_str ---
    let mut name: ArrayString<32> = ArrayString::new();
    name.push_str("sensor-01"); // panics if the result would exceed 32 bytes
    defmt::info!("ArrayString: \"{}\" (len={})", name.as_str(), name.len());

    // --- ArrayString: fallible try_push_str ---
    match name.try_push_str("_this_suffix_would_overflow_the_32_byte_limit") {
        Ok(_) => defmt::info!("try_push_str succeeded (unexpected)"),
        Err(_) => defmt::info!("try_push_str rejected: would exceed 32-byte capacity"),
    }
    // The string is unchanged after a failed try_push_str.
    defmt::info!("ArrayString unchanged: \"{}\"", name.as_str());

    loop {}
}
