#![no_std]
#![no_main]
#![deny(improper_ctypes)]
#![deny(improper_ctypes_definitions)]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

// C declarations and the lint suppressions they need stay quarantined
// here, the rest of the crate stays fully convention-checked.
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
    use core::ffi::c_int;

    // A `DeviceState` byte written by C is not proven to be one of the
    // three valid variants: C only ever sees a `uint8_t`, and
    // nothing stops it from writing `3` or `255`. Reading an arbitrary
    // byte back as a real `#[repr(u8)] enum` would be undefined
    // behavior the moment that byte isn't one of the declared variants.
    // Wrapping it as a plain, `#[repr(transparent)]` integer first
    // matched against the C `typedef uint8_t DeviceState` in
    // c/device_status.h, defers that risk until `checked()` actually
    // validates the value.
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
    pub struct DeviceState(pub u8);

    #[allow(dead_code)]
    pub const DEVICE_STATE_IDLE: DeviceState = DeviceState(0);
    #[allow(dead_code)]
    pub const DEVICE_STATE_ACTIVE: DeviceState = DeviceState(1);
    #[allow(dead_code)]
    pub const DEVICE_STATE_ERROR: DeviceState = DeviceState(2);

    // The real Rust enum, with real exhaustiveness checking. It only
    // ever gets created through `DeviceState::checked()`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
    pub enum CheckedDeviceState {
        Idle,
        Active,
        Error,
    }

    impl DeviceState {
        pub fn checked(self) -> Option<CheckedDeviceState> {
            match self.0 {
                0 => Some(CheckedDeviceState::Idle),
                1 => Some(CheckedDeviceState::Active),
                2 => Some(CheckedDeviceState::Error),
                _ => None,
            }
        }
    }

    // #[repr(C)] pins field order, alignment, and padding to C's rules
    // so this layout matches `DeviceStatus` in device_status.h exactly.
    // `#[repr(transparent)]` on `DeviceState` means swapping
    // it in for a bare `u8` here changes nothing about that layout.
    #[repr(C)]
    #[derive(Debug, Clone, Copy, defmt::Format)]
    pub struct DeviceStatus {
        pub id: c_int,
        pub state: DeviceState,
        pub temperature: f32,
    }

    // Compile-time layout check mirroring the C-side _Static_assert.
    const _: () = assert!(core::mem::size_of::<DeviceStatus>() == 12);

    unsafe extern "C" {
        pub fn add(a: c_int, b: c_int) -> c_int;
        pub fn update_status(status: *mut DeviceStatus, delta_temp: f32);
    }
}

// Safe wrapper: encapsulates the unsafe call behind an invariant the
// caller can rely on.
pub fn safe_add(a: i32, b: i32) -> i32 {
    assert!(a >= 0 && b >= 0, "only non-negative values allowed");
    unsafe { ffi::add(a, b) }
}

pub fn safe_update_status(status: &mut ffi::DeviceStatus, delta_temp: f32) {
    unsafe { ffi::update_status(status, delta_temp) };
}

// Only used for logging: turns whatever byte C last wrote into a
// human-readable name, treating anything outside the three valid
// variants as a fault rather than guessing.
fn state_name(state: ffi::DeviceState) -> &'static str {
    match state.checked() {
        Some(ffi::CheckedDeviceState::Idle) => "Idle",
        Some(ffi::CheckedDeviceState::Active) => "Active",
        Some(ffi::CheckedDeviceState::Error) => "Error",
        None => "INVALID",
    }
}

#[entry]
fn main() -> ! {
    let sum = safe_add(1940, 46);
    defmt::info!("add(1940, 46) = {}", sum);

    let mut status = ffi::DeviceStatus {
        id: 7,
        state: ffi::DEVICE_STATE_IDLE,
        temperature: 20.0,
    };
    defmt::info!(
        "initial status: id={} state={} temperature={}",
        status.id,
        state_name(status.state),
        status.temperature
    );

    safe_update_status(&mut status, 5.5);
    defmt::info!(
        "after +5.5C: id={} state={} temperature={}",
        status.id,
        state_name(status.state),
        status.temperature
    );

    // Pushes temperature past the C side's 60.0 threshold, so the C
    // function itself flips the state to Error.
    safe_update_status(&mut status, 60.0);
    defmt::info!(
        "after +60.0C: id={} state={} temperature={}",
        status.id,
        state_name(status.state),
        status.temperature
    );

    loop {
        cortex_m::asm::nop();
    }
}
