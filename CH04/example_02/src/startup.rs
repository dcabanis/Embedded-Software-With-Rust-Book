use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping, write_bytes};

const INTERRUPT_COUNT: usize = 68;

unsafe extern "C" {
    static mut _glob_RW: u32;
    static mut _eglob_RW: u32;
    static mut _glob_ZI: u32;
    static mut _eglob_ZI: u32;
    static _sidata: u32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset() -> ! {
    unsafe {
        let data_words = section_len_words(addr_of!(_glob_RW), addr_of!(_eglob_RW));
        if data_words > 0 {
            copy_nonoverlapping(
                &_sidata as *const u32,
                addr_of_mut!(_glob_RW) as *mut u32,
                data_words,
            );
        }

        let bss_words = section_len_words(addr_of!(_glob_ZI), addr_of!(_eglob_ZI));
        if bss_words > 0 {
            write_bytes(addr_of_mut!(_glob_ZI) as *mut u32, 0, bss_words);
        }
    }

    crate::main()
}

fn section_len_words(start: *const u32, end: *const u32) -> usize {
    (end as usize)
        .saturating_sub(start as usize)
        .saturating_div(core::mem::size_of::<u32>())
}

unsafe extern "C" fn reset_trampoline() {
    unsafe { reset() }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[unsafe(link_section = ".vector_table.exceptions")]
#[unsafe(no_mangle)]
static EXCEPTIONS: [Vector; 15] = [
    Vector {
        handler: reset_trampoline,
    },
    Vector {
        handler: crate::nmi,
    },
    Vector {
        handler: crate::hard_fault,
    },
    Vector {
        handler: crate::memory_fault,
    },
    Vector {
        handler: crate::bus_fault,
    },
    Vector {
        handler: crate::usage_fault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: crate::svc_call,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: crate::pend_sv,
    },
    Vector {
        handler: crate::sys_tick,
    },
];

#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
static INTERRUPTS: [Vector; INTERRUPT_COUNT] = [Vector {
    handler: crate::default_handler,
}; INTERRUPT_COUNT];