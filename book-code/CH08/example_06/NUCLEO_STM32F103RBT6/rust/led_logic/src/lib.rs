#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Return 1 for LED on, or 0 for LED off.
#[unsafe(no_mangle)]
pub extern "C" fn led_pattern(step: u32) -> u32 {
    match step % 10 {
        0 | 2 => 1,
        _ => 0,
    }
}
