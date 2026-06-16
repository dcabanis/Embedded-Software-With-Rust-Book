#![no_std]
#![no_main]

use cortex_m_rt::entry;

// If both are enabled, fail early with a clear message.
#[cfg(all(feature = "semihosting", feature = "panic-halt"))]
compile_error!(
    "Do not enable both semihosting and panic-halt. \
Use: --no-default-features --features semihosting"
);

// Ensure *some* panic backend is selected.
#[cfg(not(any(feature = "semihosting", feature = "panic-halt")))]
compile_error!(
    "No panic backend selected. Enable default features (panic-halt) \
or build with --features semihosting."
);

#[cfg(feature = "semihosting")]
use {
    cortex_m_semihosting::hprintln,
    panic_semihosting as _,
};

#[cfg(feature = "panic-halt")]
use panic_halt as _;

#[entry]
fn main() -> ! {
    #[cfg(feature = "semihosting")]
    {
        let _ = hprintln!("Semihosting I/O is enabled.");
    }

    panic!("This will halt (panic-halt) or print via semihosting (semihosting feature).");
}