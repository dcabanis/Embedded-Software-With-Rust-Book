#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use cortex_m::{asm, peripheral::{syst::SystClkSource, scb::SystemHandler, SCB}};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f1xx_hal as _;

/// 8 MHz core clock -> 1 second SysTick period.
/// Reload must stay within the 24-bit SysTick counter range.
const SYSCLK_HZ: u32 = 8_000_000;
const SYSTICK_RELOAD: u32 = SYSCLK_HZ - 1;

// Shared flag toggled by the SysTick interrupt and read by the main loop.
static TOGGLE: AtomicBool = AtomicBool::new(false);

// Application entry called by the reset code in `cortex-m-rt`.
#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;
    let mut scb = cp.SCB;

    unsafe {
        SCB::clear_pendst(); // clear any pending SysTick
        scb.set_priority(SystemHandler::SysTick, 16); // set SysTick priority
    }

    // Configure SysTick to fire once per second from the core clock.
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(SYSTICK_RELOAD);
    syst.clear_current();
    syst.enable_interrupt(); // "unmask" SysTick
    syst.enable_counter(); // start ticking

    // Sleep until interrupts arrive, then print a message based on TOGGLE.
    for _ in 0..10 {
        let _ = hprintln!("Waiting for an interrupt...");
        asm::wfi();
        if TOGGLE.load(Ordering::Relaxed) {
            let _ = hprintln!("You say goodbye...");
        } else {
            let _ = hprintln!("And I say hello.");
        }
    }
    asm::udf();
}

#[exception]
fn SysTick() {
    // Periodic heartbeat used by the main loop for simple state changes.
    TOGGLE.fetch_xor(true, Ordering::Relaxed);
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    let _ = hprintln!("Hard Fault handler"); // Only for demonstration!

    let scb = unsafe { &*SCB::PTR };
    let cfsr = scb.cfsr.read();

    if (cfsr & (1 << 16)) != 0 {
        // Bit 0 of UFSR is set: undefined instruction caused the fault
    }
    loop {}
}

#[exception]
unsafe fn DefaultHandler(_irqn: i16) -> ! {
    loop {}
}
