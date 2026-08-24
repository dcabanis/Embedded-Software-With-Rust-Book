#![no_std]
#![deny(improper_ctypes_definitions)]

use core::panic::PanicInfo;

// Every no_std program needs one: core has no built-in idea of what a
// panic should do on this hardware. A function exported to C
// should never panic to report an error, so this only exists to satisfy
// the language requirement, not as an error-reporting path.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn square(value: i32) -> i32 {
    value.wrapping_mul(value)
}

// Idiomatic Rust error handling (`Result`) cannot cross
// the FFI boundary, so the C caller gets an integer status code instead.
// The output parameter is a raw pointer, not `&mut i32`, because a Rust
// reference must never be null but nothing stops the C caller from
// passing NULL.
//
// SAFETY: `result`, if non-null, must be valid for writes and properly
// aligned for an `i32`, for the duration of this call. A null check
// narrows the failure mode but doesn't prove the rest of that contract,
// so the function can't claim to be fully safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn divide(a: i32, b: i32, result: *mut i32) -> i32 {
    if result.is_null() {
        return -2; // error: null output pointer
    }
    match a.checked_div(b) {
        Some(quotient) => {
            // SAFETY: pointer checked non-null; the rest of the contract
            // is documented above and is the caller's responsibility.
            unsafe { *result = quotient };
            0 // success
        }
        None => -1, // error: division by zero, or i32::MIN / -1 overflow
    }
}
