#![no_std]
#![no_main]

use core::cell::Cell;
use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m::interrupt::{free, Mutex};
use cortex_m::peripheral::{scb::SystemHandler, syst::SystClkSource, SCB};
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f1xx_hal as _;

// Shared data incremented from both execution contexts (main and SysTick)
static SHARED: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
// Monotonic "seconds" counter updated only by the SysTick handler
static SECONDS: AtomicU32 = AtomicU32::new(0);
// 8 MHz core clock with SysTick configured for a 1 Hz interrupt period
const SYSCLK_HZ: u32 = 8_000_000;
const SYSTICK_RELOAD: u32 = SYSCLK_HZ - 1;

#[entry]
fn main() -> ! {
    // Initialize core peripherals and configure the SysTick timer interrupt
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;
    let mut scb = cp.SCB;

    unsafe {
        SCB::clear_pendst();
        scb.set_priority(SystemHandler::SysTick, 16);
    }

    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(SYSTICK_RELOAD);
    syst.clear_current();
    syst.enable_interrupt();
    syst.enable_counter();

    // Main work loop: keep incrementing shared data under a critical section
    loop {
        // Entering critical section with
        // interrupts disabled.
        free(|cs| {
            // Borrow the mutex-protected Cell
            // `.borrow(cs)` ensures only code in this
            // critical section may access the data
            let data = SHARED.borrow(cs);
            // SysTick cannot preempt here
            let next = data.get() + 1;
            data.set(next);
        });
    }
}

#[exception]
fn SysTick() {
    // Periodic interrupt path: bump shared data and track elapsed seconds
    let next = free(|cs| {
        let data = SHARED.borrow(cs);
        // Safe shared mutation
        let next = data.get() + 1;
        data.set(next);
        next
    });

    let seconds = SECONDS.fetch_add(1, Ordering::Relaxed) + 1;
    // Periodic trace showing both logical time and shared state
    hprintln!("Seconds: {}, shared value: {}", seconds, next);
}
