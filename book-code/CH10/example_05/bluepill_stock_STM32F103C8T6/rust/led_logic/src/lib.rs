#![no_std]

use panic_halt as _;

/// Return 1 for LED on, or 0 for LED off.
#[unsafe(no_mangle)]
pub extern "C" fn led_pattern(step: u32) -> u32 {
    match step % 10 {
        0 | 2 => 1,
        _ => 0,
    }
}
