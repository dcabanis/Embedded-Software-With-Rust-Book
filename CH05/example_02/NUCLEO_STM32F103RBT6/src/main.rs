#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32f1::stm32f103;

#[entry]
fn main() -> ! {
    // Get handles to the core and device peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f103::Peripherals::take().unwrap();

    // --- Setup SysTick for a 1-second delay ---
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    // The STM32F1 boots to an 8MHz internal oscillator (HSI).
    // For a 1-second delay, we need 8,000,000 ticks.
    // The reload value is `ticks - 1`.
    syst.set_reload(8_000_000 - 1);
    syst.clear_current(); // Clear the current value to start a fresh countdown
    syst.enable_counter();

    // --- Setup GPIO Port A ---
    let gpioa = &dp.GPIOA;
    let rcc = &dp.RCC;

    // Enable the clock for GPIO Port A using a safe read-modify-write
    rcc.apb2enr().modify(|_, w| w.iopaen().set_bit());

    // Configure PA5 as a push-pull output using the PAC's safe `modify` API.
    // PA5 is in CRL (pins 0-7).
    gpioa.crl().modify(|_, w| {
        w.mode5()
            .output2() // Set mode to output, 2 MHz
            .cnf5()
            .push_pull() // Set configuration to push-pull
    });

    loop {
        // Turn on LED (set PA5 high for an active-high LED)
        gpioa.bsrr().write(|w| w.bs5().set_bit());
        // Wait for the SysTick timer to wrap (1 second)
        while !syst.has_wrapped() {}

        // Turn off LED (set PA5 low for an active-high LED)
        gpioa.bsrr().write(|w| w.br5().set_bit());
        // Wait for the SysTick timer to wrap again
        while !syst.has_wrapped() {}
    }
}
