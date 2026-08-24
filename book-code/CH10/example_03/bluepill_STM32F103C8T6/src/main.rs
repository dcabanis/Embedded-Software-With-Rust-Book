#![no_std]
#![no_main]
#![deny(improper_ctypes)]
#![deny(improper_ctypes_definitions)]

use core::ffi::{c_int, c_void, CStr};
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
    use core::ffi::{c_char, c_int, c_void};

    unsafe extern "C" {
        pub fn sum_array(arr: *const i32, len: usize) -> i32;
        pub fn c_string_len(s: *const c_char) -> usize;
        pub fn register_callback_ctx(
            cb: unsafe extern "C" fn(c_int, *mut c_void),
            ctx: *mut c_void,
        );
        pub fn trigger_callback_ctx(value: c_int);
    }
}

// Rust's slice guarantees safety on this side; `as_ptr()` /
// `len()` are all C needs to walk the same memory.
pub fn safe_sum_array(slice: &[i32]) -> i32 {
    unsafe { ffi::sum_array(slice.as_ptr(), slice.len()) }
}

// Taking `&CStr` rather than `&str` forces the caller to hand over
// something already NUL-terminated.
pub fn safe_c_string_len(s: &CStr) -> usize {
    unsafe { ffi::c_string_len(s.as_ptr()) }
}

static EVENT_COUNT: AtomicU32 = AtomicU32::new(0);

/// The trampoline casts the generic `void*` context back into
/// the concrete type it actually is, then does the real work.
///
/// SAFETY: `ctx` must be exactly the pointer registered in
/// `install_callback` below: a valid, live `*const AtomicU32` for as long
/// as the callback might fire. Nothing in the signature proves that, so
/// the function itself is `unsafe extern "C" fn`, not just the cast
/// inside it.
unsafe extern "C" fn count_events(value: c_int, ctx: *mut c_void) {
    // SAFETY: `ctx` is exactly the pointer registered in `install_callback`,
    // and `EVENT_COUNT` lives for the whole program.
    let counter = unsafe { &*(ctx as *const AtomicU32) };
    let event_number = counter.fetch_add(1, Ordering::Relaxed) + 1;
    defmt::info!("callback fired: value={} (event #{})", value, event_number);
}

fn install_callback() {
    unsafe {
        ffi::register_callback_ctx(count_events, &EVENT_COUNT as *const _ as *mut c_void);
    }
}

#[entry]
fn main() -> ! {
    let values = [10, 20, 30, 40, 50];
    let total = safe_sum_array(&values);
    defmt::info!("sum_array({}) = {}", values, total);

    // A `c"..."` literal is a &CStr with the terminating zero byte added
    // by the compiler, no risk of handing C an unterminated Rust &str.
    let banner = c"Hello from Rust";
    let len = safe_c_string_len(banner);
    defmt::info!("c_string_len(\"Hello from Rust\") = {}", len);

    install_callback();

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000 - 1); // ~1 s at the 8 MHz HSI reset clock
    syst.clear_current();
    syst.enable_counter();

    let mut tick: i32 = 0;
    loop {
        if syst.has_wrapped() {
            tick += 1;
            // C decides to call back into Rust here; Rust only supplied
            // the function to call and the context to hand it.
            unsafe { ffi::trigger_callback_ctx(tick) };
        }
    }
}
