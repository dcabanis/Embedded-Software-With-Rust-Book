#![no_std]
#![no_main]

use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;

extern "C" {
    // Defined in memory.x as __ebss — the first free word above .data + .bss.
    // With semihosting there is no RTT ring buffer in .uninit, so __ebss is
    // the correct low bound for the stack paint region.
    static mut _stack_bottom: u32;
}

const CANARY: u32 = 0xDEAD_BEEF;

/// Fill free RAM from `_stack_bottom` up to just below the current SP with
/// the canary pattern.  Call once at the start of main before any work.
unsafe fn paint_stack() {
    let bottom: *mut u32 = &raw mut _stack_bottom;
    let sp: u32;
    core::arch::asm!("mov {}, sp", out(reg) sp, options(nomem, nostack));
    // 64-byte margin so we do not clobber our own return address or the
    // paint_stack frame itself.
    let limit = sp.saturating_sub(64) as *const u32;
    let mut p = bottom;
    while (p as *const u32) < limit {
        p.write(CANARY);
        p = p.add(1);
    }
}

/// Count how many 32-bit words at the bottom of the stack still hold the
/// canary.  The first missing canary marks the deepest frame reached since
/// paint_stack was called.
unsafe fn unused_stack_words() -> usize {
    let bottom: *const u32 = &raw const _stack_bottom;
    let mut p = bottom;
    let mut count: usize = 0;
    while p.read() == CANARY {
        count += 1;
        p = p.add(1);
    }
    count
}

// #[inline(never)] forces a real call frame in release mode; without it the
// optimiser inlines both functions into main and merges their stack slots.
//
// MaybeUninit + write_volatile pins the allocation without generating
// __aeabi_memclr.  The read_volatile in work_deep keeps buf alive across the
// nested call so both frames coexist on the stack simultaneously.

#[inline(never)]
fn work_shallow() {
    let mut buf = MaybeUninit::<[u8; 256]>::uninit();
    unsafe { core::ptr::write_volatile(buf.as_mut_ptr() as *mut u8, 0u8) };
}

#[inline(never)]
fn work_deep() {
    let mut buf = MaybeUninit::<[u8; 512]>::uninit();
    unsafe { core::ptr::write_volatile(buf.as_mut_ptr() as *mut u8, 0u8) };
    work_shallow();
    // Reading buf after the nested call prevents the compiler from reusing
    // buf's 512-byte slot for work_shallow's frame.
    unsafe { core::ptr::read_volatile(buf.as_ptr() as *const u8) };
}

#[entry]
fn main() -> ! {
    // Paint first — no defmt ring buffer to worry about with semihosting.
    unsafe { paint_stack(); }

    // Take all measurements without any I/O between them.  Semihosting halts
    // the CPU on every hprintln!, which would push its own frame and erase
    // canary words, skewing the per-function numbers.
    let start         = unsafe { unused_stack_words() };
    work_shallow();
    let after_shallow = unsafe { unused_stack_words() };
    work_deep();
    let after_deep    = unsafe { unused_stack_words() };

    // All measurements complete — print now.
    let shallow_bytes = start.saturating_sub(after_shallow) * 4;
    let deep_bytes    = start.saturating_sub(after_deep) * 4;
    hprintln!("Unused stack before work:        {} words ({} bytes)", start,         start * 4);
    hprintln!("Unused stack after work_shallow: {} words ({} bytes)", after_shallow, after_shallow * 4);
    hprintln!("Unused stack after work_deep:    {} words ({} bytes)", after_deep,    after_deep * 4);
    hprintln!("Peak usage  work_shallow alone:  ~{} bytes", shallow_bytes);
    hprintln!("Peak usage  work_deep chain:     ~{} bytes", deep_bytes);

    loop {}
}
