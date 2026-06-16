#![no_std]
#![no_main]

use core::arch::global_asm;
use cortex_m_rt::entry;
use cortex_m::asm;
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*};

// Memory-mapped register address (STM32F103)
const GPIOA_ODR: u32 = 0x4001_080C;
const PA5: u32       = 1 << 5;

// global_asm!() out-of-line counted delay loop

global_asm!(r#"
.global asm_delay_loop
.thumb_func
asm_delay_loop:
    cbz r0, 2f
1:
    subs r0, r0, #1
    bne 1b
2:
    bx lr
"#);

extern "C" {
    fn asm_delay_loop(iterations: u32);
}

// asm!() Writes 1 to PRIMASK via MSR, which masks all exceptions except NMI
// and HardFault.
#[inline(always)]
fn disable_interrupts() {
    unsafe {
        core::arch::asm!(
            "msr PRIMASK, {reg}",
            reg = in(reg) 1u32,
            options(nomem, nostack),
        );
    }
}

// asm!() direct MMIO toggle of PA5
#[inline(always)]
unsafe fn toggle_pa5() {
    core::arch::asm!(
        "ldr  {tmp}, [{addr}]",
        "eor  {tmp}, {tmp}, {bit}",
        "str  {tmp}, [{addr}]",
        addr = in(reg) GPIOA_ODR,
        bit  = in(reg) PA5,
        tmp  = out(reg) _,
        options(nostack, preserves_flags),
    );
}

#[entry]
fn main() -> ! {
    // HAL peripheral setup
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    // Configure PA5 as push-pull output; CRL covers pins 0-7
    let _led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    // asm!() mask all interrupts before entering the blink loop
    disable_interrupts();

    loop {
        // Intrinsic: NOP burst for a short busy-wait
        for _ in 0..10_000u32 {
            unsafe { asm::nop() };
        }

        // global_asm function: longer calibrated delay
        unsafe { asm_delay_loop(400_000) };

        // asm!() inline: toggle the LED via direct MMIO
        unsafe { toggle_pa5() };
    }
}
