#![no_std]
#![no_main]

use cortex_m::peripheral::MPU;
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

extern "C" {
    // Defined in memory.x: lowest valid stack address.
    // The MPU guard region is placed here.
    // Must be aligned to the guard region size (32 bytes).
    static _stack_bottom: u32;
}

// RASR: Region Attribute and Size Register bit fields (ARMv7-M MPU)
const RASR_ENABLE: u32 = 1 << 0;
const RASR_SIZE_32B: u32 = 4 << 1; // region size = 2^(4+1) = 32 bytes
const RASR_AP_NONE: u32 = 0b000 << 24; // no access from any privilege level
const RASR_XN: u32 = 1 << 28; // execute-never

// RBAR: Region Base Address Register bit fields
const RBAR_VALID: u32 = 1 << 4; // use the REGION field in this write
const RBAR_REGION_0: u32 = 0; // configure region 0

// CTRL: MPU Control Register bit fields
const CTRL_ENABLE: u32 = 1 << 0;
const CTRL_PRIVDEFENA: u32 = 1 << 2; // default memory map active as background

/// Configure MPU region 0 as a 32-byte no-access execute-never guard at
/// `guard_addr`.  `guard_addr` must be 32-byte aligned.
///
/// Any read, write, or fetch within the guard region raises a MemManage fault
/// (or HardFault if MemManage is not separately enabled).
fn install_stack_guard(mpu: &MPU, guard_addr: u32) {
    unsafe {
        // Disable the MPU while reconfiguring to avoid mis-attributed faults.
        mpu.ctrl.write(0);

        // Program region 0: base address + size + attributes in one write.
        mpu.rbar.write(guard_addr | RBAR_VALID | RBAR_REGION_0);
        mpu.rasr.write(RASR_ENABLE | RASR_SIZE_32B | RASR_AP_NONE | RASR_XN);

        // Re-enable the MPU with the default memory map as background so
        // unrelated regions keep their normal access permissions.
        mpu.ctrl.write(CTRL_ENABLE | CTRL_PRIVDEFENA);

        // Data and instruction synchronisation barriers ensure the MPU
        // configuration takes effect before the next memory access.
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    }
}

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();

    let guard_addr = (&raw const _stack_bottom) as u32;

    defmt::info!("Installing MPU stack guard at {:#x} (32-byte region)", guard_addr);
    install_stack_guard(&cp.MPU, guard_addr);
    defmt::info!(
        "Stack guard active. Addresses {:#x}–{:#x} will fault on any access.",
        guard_addr,
        guard_addr + 31
    );

    defmt::info!("Normal operation continues — no access to the guard region.");

    // To verify the guard is active, uncomment the lines below.
    // WARNING: this causes an immediate MemManage (or Hard) fault.
    //
    // unsafe {
    //     core::ptr::write_volatile(guard_addr as *mut u32, 0xDEAD);
    // }

    loop {}
}
