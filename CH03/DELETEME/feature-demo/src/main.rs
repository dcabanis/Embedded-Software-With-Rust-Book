#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f1xx_hal as _;

#[cfg(feature = "semihosting")]
use cortex_m_semihosting::hprintln;

// Compile-time guard: if someone enables both the default and semihosting features by mistake,
// fail early with a clear error.
#[cfg(all(feature = "semihosting", feature = "default"))]
compile_error!("Don't enable both 'semihosting' and the default 'panic-halt'. Use: --no-default-features --features semihosting");

// Select exactly one panic backend and (optionally) I/O
#[cfg(feature = "semihosting")]
use panic_semihosting as _;

#[cfg(not(feature = "semihosting"))]
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Optional non-panic output to prove semihosting is active
    #[cfg(feature = "semihosting")]
    {
        let _ = hprintln!("Semihosting I/O is enabled.");
    }

    // Force a panic to demonstrate behavior in each build mode.
    panic!("This is a demo panic: it will halt (default) or print via semihosting (semihosting feature).");
}


