#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::interrupt::{free, Mutex};
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f1xx_hal as _;

// Shared core peripheral: protected behind Mutex + RefCell and initialized once in main.
static G_SYST: Mutex<RefCell<Option<SYST>>> = Mutex::new(RefCell::new(None));

// 8 MHz core clock on STM32F1, using SysTick external clock source (HCLK/8)
const SYSCLK_HZ: u32 = 8_000_000;
const SYSTICK_EXT_HZ: u32 = SYSCLK_HZ / 8;
const SYSTICK_RELOAD: u32 = SYSTICK_EXT_HZ - 1; // 1-second wrap period
const WRAPS_FOR_30_SECONDS: u32 = 30;

#[entry]
fn main() -> ! {
    // Configure SysTick locally while `main` owns it
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;

    syst.set_clock_source(SystClkSource::External);
    syst.set_reload(SYSTICK_RELOAD);
    syst.clear_current();
    syst.enable_interrupt();
    syst.enable_counter();

    // Move ownership into global shared storage
    // All later access from both contexts goes through this single path
    free(|cs| {
        G_SYST.borrow(cs).replace(Some(syst));
    });

    // Main loop waits for interrupts and checks wrap events
    // Every 30 wraps (30 x 1s), print a coarse elapsed-time message
    let mut wraps = 0u32;
    loop {
        // Sleep until an interrupt wakes the core
        cortex_m::asm::wfi();

        free(|cs| {
            // Critical section: safe shared mutable access to the same peripheral.
            if let Some(syst) = G_SYST.borrow(cs).borrow_mut().as_mut() {
                if syst.has_wrapped() {
                    wraps += 1;
                    if wraps >= WRAPS_FOR_30_SECONDS {
                        wraps = 0;
                        let _ = hprintln!("30 seconds elapsed");
                    }
                }
            }
        });
    }
}

#[exception]
fn SysTick() {
    // Handler mode access to the same global `SYST`
    // Here we print the current value register on each interrupt
    free(|cs| {
        if let Some(syst) = G_SYST.borrow(cs).borrow_mut().as_mut() {
            let current = syst.cvr.read();
            let _ = hprintln!("SysTick CVR reloaded: {}", current);
        }
    });
}
