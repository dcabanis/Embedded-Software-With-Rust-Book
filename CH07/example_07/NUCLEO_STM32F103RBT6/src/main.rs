#![no_std]
#![no_main]

use core::mem::MaybeUninit;
use cortex_m_rt::{entry, exception};
use defmt_rtt as _;
use panic_probe as _;

// In the default layout (flip-link disabled) .data lives at the bottom of
// RAM.  As the stack grows downward from the top, an overflow eventually
// reaches this address and silently overwrites it.  With flip-link the stack
// is placed at the bottom of RAM so an overflow hits unmapped memory first,
// producing a deterministic bus fault instead of silent corruption.
static mut DEVICE_ID: u32 = 0xDEAD_BEEF;

// Each recursive call allocates 512 bytes on the stack.
// MaybeUninit is used deliberately: it avoids zero-initialisation and the
// resulting __aeabi_memclr call, which would add extra function-call frames
// and obscure the stack trace at the point of overflow.
fn recurse(depth: u32) {
    // Log every frame so the RTT-corruption difference is visible:
    // - flip-link ON:  RTT buffer is at the top of RAM (protected); all depth
    //                  values print cleanly until the bus fault.
    // - flip-link OFF: RTT buffer is at the bottom of RAM; the stack overwrites
    //                  it in the last few frames, cutting off the stream early.
    defmt::info!("depth = {}", depth);

    // MaybeUninit avoids zero-initialisation (__aeabi_memclr).
    // The single volatile write pins the 512-byte allocation on the stack
    // (one STR instruction) without triggering a bulk-copy (memcpy) call.
    let mut buf = MaybeUninit::<[u8; 512]>::uninit();
    unsafe {
        core::ptr::write_volatile(buf.as_mut_ptr() as *mut u8, depth as u8);
    }
    recurse(depth + 1);
}

// This handler is registered so the fault is visible in the ELF symbol table
// and the debugger can name the exception.  On Cortex-M3 (no MSPLIM, no MPU
// on this device) a severe stack overflow exhausts the stack before the CPU
// can save the exception frame, so the processor enters lockup rather than
// executing this handler.  flip-link still provides the key benefit: the
// lockup occurs at a deterministic address (the bottom of RAM) rather than
// after silently corrupting .data / .bss.
#[exception]
unsafe fn HardFault(_ef: &cortex_m_rt::ExceptionFrame) -> ! {
    defmt::error!("HardFault: stack overflow (flip-link boundary crossed)");
    loop {
        cortex_m::asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    defmt::info!("DEVICE_ID = {:#x}  (should be 0xdeadbeef)", unsafe { DEVICE_ID });
    defmt::info!("Starting infinite recursion — 512 bytes of stack per frame...");

    // With flip-link enabled the recursion halts at a deterministic depth
    // when the SP crosses the bottom-of-RAM boundary, generating an
    // SwdApFault visible to probe-rs.
    //
    // With flip-link disabled the recursion first corrupts the RTT buffer
    // (in .bss, just above .data) and then overwrites DEVICE_ID (in .data).
    // The crash is non-deterministic and may not appear to probe-rs until
    // well after the corruption has begun.
    recurse(0);

    loop {}
}
