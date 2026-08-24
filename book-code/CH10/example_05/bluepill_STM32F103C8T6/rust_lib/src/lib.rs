#![no_std]
#![deny(improper_ctypes_definitions)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// A self-contained, side-effect-free checksum. Adding up every byte and
// detecting a mismatch on the receiving end is a simple way to catch
// data corrupted in transit.
//
// SAFETY: `data` must be null, or point to `len` initialized, readable
// bytes with no concurrent mutation for the duration of the call. A
// null check narrows the failure mode but doesn't prove the rest of
// that contract, so the function can't claim to be fully safe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn checksum(data: *const u8, len: usize) -> u8 {
    if data.is_null() {
        return 0;
    }
    // SAFETY: pointer checked non-null; the rest of the contract is
    // documented above and is the caller's responsibility to uphold.
    let bytes = unsafe { core::slice::from_raw_parts(data, len) };
    bytes.iter().fold(0u8, |acc, b| acc.wrapping_add(*b))
}
