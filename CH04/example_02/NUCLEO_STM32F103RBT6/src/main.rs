#![no_std]
#![no_main]

mod startup;

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m_semihosting::hprintln;

use cortex_m::asm;
use cortex_m::peripheral::syst::SystClkSource;

const SYSCLK_HZ: u32 = 8_000_000;
const SYSTICK_RELOAD: u32 = SYSCLK_HZ - 1;

// Shared flag toggled by the SysTick interrupt and read by the main loop.
static TOGGLE: AtomicBool = AtomicBool::new(false);

// Application entry called by the reset code in `startup`.
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;

    // Configure SysTick to fire once per second from the core clock.
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(SYSTICK_RELOAD);
    syst.clear_current();
    syst.enable_interrupt();
    syst.enable_counter();

    // Sleep until interrupts arrive, then print a message based on TOGGLE.
    for _ in 0..10 {
        hprintln!("Waiting for an interrupt...");
        asm::wfi();
        if TOGGLE.load(Ordering::Relaxed) {
            hprintln!("Hello!");
        } else {
            hprintln!("How do you do?");
        }
    }
    asm::udf();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn halt_forever() -> ! {
    loop {}
}

// Core exception handlers: print a diagnostic and stop execution.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nmi() {
    hprintln!("NMI handler");
    halt_forever();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hard_fault() {
    hprintln!("Hard Fault handler");
    halt_forever();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memory_fault() {
    hprintln!("Memory Fault handler");
    halt_forever();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bus_fault() {
    hprintln!("Bus Fault handler");
    halt_forever();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usage_fault() {
    hprintln!("Usage Fault handler");
    halt_forever();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn svc_call() {
    hprintln!("SVCall handler");
    halt_forever();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pend_sv() {
    hprintln!("PendSV handler");
    halt_forever();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_tick() {
    // Periodic heartbeat used by the main loop for simple state changes.
    TOGGLE.fetch_xor(true, Ordering::Relaxed);
}

// Fallback for any unimplemented peripheral interrupt.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn default_handler() {
    halt_forever();
}
