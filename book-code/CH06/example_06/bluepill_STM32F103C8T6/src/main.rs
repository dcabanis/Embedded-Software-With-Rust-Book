#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m::peripheral::{scb::Exception, SCB};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use rtt_target::{rprintln, rtt_init_print};

// Routes panic!() calls from the configurable-fault handlers through RTT.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {}
}

// ── Configurable fault handlers ──────────────────────────────────────────────
// These only fire when the corresponding enable bit in SHCSR is set (see main).
// Without that, each fault escalates to HardFault instead.

#[exception]
unsafe fn MemoryManagement() -> ! {
    panic!("MemoryManagement fault");
}

#[exception]
unsafe fn BusFault() -> ! {
    panic!("BusFault");
}

#[exception]
unsafe fn UsageFault() -> ! {
    panic!("UsageFault");
}

// HardFault handler 
// Prints the CPU register snapshot saved by the processor, then reads and
// decodes the SCB fault-status registers to identify the fault type and,
// where available, the faulting address.

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    let scb = &*SCB::PTR;
    let cfsr  = scb.cfsr.read();
    let hfsr  = scb.hfsr.read();
    let mmfar = scb.mmfar.read();
    let bfar  = scb.bfar.read();

    rprintln!("=== HardFault ===");

    rprintln!("ExceptionFrame (CPU state at fault):");
    rprintln!("  r0:   0x{:08x}", ef.r0());
    rprintln!("  r1:   0x{:08x}", ef.r1());
    rprintln!("  r2:   0x{:08x}", ef.r2());
    rprintln!("  r3:   0x{:08x}", ef.r3());
    rprintln!("  r12:  0x{:08x}", ef.r12());
    rprintln!("  lr:   0x{:08x}", ef.lr());
    rprintln!("  pc:   0x{:08x}  <-- faulting instruction", ef.pc());
    rprintln!("  xpsr: 0x{:08x}", ef.xpsr());

    // HFSR: top-level hard-fault classification
    rprintln!("HFSR: 0x{:08x}", hfsr);
    if hfsr & (1 << 30) != 0 {
        rprintln!("  FORCED    escalated from a configurable fault (see CFSR)");
    }
    if hfsr & (1 << 1) != 0 {
        rprintln!("  VECTTBL   vector table read fault during exception entry");
    }
    if hfsr & (1 << 31) != 0 {
        rprintln!("  DEBUGEVT  debug event escalated to HardFault");
    }

    // CFSR: split into its three sub-registers
    rprintln!("CFSR: 0x{:08x}", cfsr);

    // UFSR — UsageFault Status Register (bits 31:16)
    let ufsr = (cfsr >> 16) as u16;
    if ufsr != 0 {
        rprintln!("  UFSR: 0x{:04x}  (UsageFault)", ufsr);
        if ufsr & (1 << 0) != 0 { rprintln!("    UNDEFINSTR   undefined or unsupported instruction"); }
        if ufsr & (1 << 1) != 0 { rprintln!("    INVSTATE     EPSR.T or EPSR.IT invalid for instruction"); }
        if ufsr & (1 << 2) != 0 { rprintln!("    INVPC        integrity check failure on EXC_RETURN"); }
        if ufsr & (1 << 3) != 0 { rprintln!("    NOCP         coprocessor absent or disabled"); }
        if ufsr & (1 << 8) != 0 { rprintln!("    UNALIGNED    unaligned memory access"); }
        if ufsr & (1 << 9) != 0 { rprintln!("    DIVBYZERO    integer divide by zero"); }
    }

    // BFSR — BusFault Status Register (bits 15:8)
    let bfsr_byte = ((cfsr >> 8) & 0xFF) as u8;
    if bfsr_byte != 0 {
        rprintln!("  BFSR: 0x{:02x}  (BusFault)", bfsr_byte);
        if bfsr_byte & (1 << 0) != 0 { rprintln!("    IBUSERR      instruction prefetch bus error"); }
        if bfsr_byte & (1 << 1) != 0 { rprintln!("    PRECISERR    precise data bus error"); }
        if bfsr_byte & (1 << 2) != 0 { rprintln!("    IMPRECISERR  imprecise data bus error"); }
        if bfsr_byte & (1 << 3) != 0 { rprintln!("    UNSTKERR     bus fault during exception unstacking"); }
        if bfsr_byte & (1 << 4) != 0 { rprintln!("    STKERR       bus fault during exception stacking"); }
        if bfsr_byte & (1 << 7) != 0 {
            rprintln!("    BFARVALID    BFAR=0x{:08x}  (faulting address)", bfar);
        }
    }

    // MMFSR — MemManage Fault Status Register (bits 7:0)
    let mmfsr_byte = (cfsr & 0xFF) as u8;
    if mmfsr_byte != 0 {
        rprintln!("  MMFSR: 0x{:02x}  (MemManage)", mmfsr_byte);
        if mmfsr_byte & (1 << 0) != 0 { rprintln!("    IACCVIOL    instruction access violation"); }
        if mmfsr_byte & (1 << 1) != 0 { rprintln!("    DACCVIOL    data access violation"); }
        if mmfsr_byte & (1 << 3) != 0 { rprintln!("    MUNSTKERR   MPU violation during unstacking"); }
        if mmfsr_byte & (1 << 4) != 0 { rprintln!("    MSTKERR     MPU violation during stacking"); }
        if mmfsr_byte & (1 << 7) != 0 {
            rprintln!("    MMARVALID   MMFAR=0x{:08x}  (faulting address)", mmfar);
        }
    }

    loop {}
}

// ── Fault scenario selector ───────────────────────────────────────────────────
//
//  0 – Software panic via unwrap() on a None value  → #[panic_handler]
//  1 – Hardware UsageFault via the UDF instruction   → #[exception] UsageFault
//        (or HardFault when the configurable-fault block below is commented out)
const FAULT_DEMO: u8 = 0;

// ── Entry point ───────────────────────────────────────────────────────────────

#[entry]
fn main() -> ! {
    // Clear DEMCR.VC_HARDERR so probe-rs's vector-catch does not halt the CPU
    // before our exception handlers execute and can produce RTT output.
    unsafe {
        let demcr = 0xE000_EDFC as *mut u32;
        core::ptr::write_volatile(demcr, core::ptr::read_volatile(demcr) & !(1 << 10));
    }

    rtt_init_print!();

    // Enable MemManage, BusFault, and UsageFault as standalone exceptions so
    // each fault type routes to its own handler above instead of escalating.
    //
    // Comment out this block to see faults escalate to HardFault instead.
    // The HardFault handler will then decode CFSR/HFSR and print the full
    // ExceptionFrame, which is the more informative path for diagnosis.
    let mut p = cortex_m::Peripherals::take().unwrap();
    p.SCB.enable(Exception::MemoryManagement);
    p.SCB.enable(Exception::BusFault);
    p.SCB.enable(Exception::UsageFault);
    rprintln!("MemManage, BusFault, UsageFault handlers enabled.");

    match FAULT_DEMO {
        0 => {
            // Software panic: unwrap() on a None value calls the #[panic_handler]
            // directly — no CPU exception is raised.  The panic message includes
            // the source location and is printed via RTT before looping forever.
            rprintln!("Triggering software panic via unwrap() on None...");
            let value: Option<u32> = None;
            let _ = value.unwrap(); // panics at runtime; unreachable!() satisfies the `!` return
            unreachable!()
        }
        _ => {
            // Hardware fault: UDF is an architecturally undefined instruction.
            // With SHCSR enabled: routes to UsageFault() -> panic!() -> RTT message.
            // With SHCSR disabled: escalates to HardFault() -> full register decode.
            rprintln!("Triggering UsageFault via UDF...");
            cortex_m::asm::udf();
        }
    }
}
