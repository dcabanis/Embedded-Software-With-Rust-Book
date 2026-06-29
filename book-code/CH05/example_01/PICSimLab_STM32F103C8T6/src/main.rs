#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;

#[unsafe(no_mangle)]
pub static MY_DATA: u32 = 0xDEADBEEF;

const SYSTICK: u32 = 0xE000_E010;
const SYST_CSR: *mut u32 = SYSTICK as *mut u32;
const SYST_RVR: *mut u32 = (SYSTICK + 0x04) as *mut u32;
const SYST_CVR: *mut u32 = (SYSTICK + 0x08) as *mut u32;

const RCC_APB2ENR: *mut u32 = 0x4002_1018 as *mut u32;
const GPIOC_CRH: *mut u32 = 0x4001_1004 as *mut u32;
const GPIOC_ODR: *mut u32 = 0x4001_100C as *mut u32;

unsafe extern "C" {
    // Linker-defined section boundaries used during early startup
    static mut _glob_RW: u32;
    static mut _eglob_RW: u32;
    static mut _glob_ZI: u32;
    static mut _eglob_ZI: u32;
    // Flash source for initialized .data values
    static _sidata: u32;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Minimal runtime init for a freestanding binary:
    // copy .data from Flash to RAM and zero .bss
    init_data_bss();

    unsafe {
        // Keep one explicit read so the example has a referenced global in Flash
        let _ = ptr::read_volatile(&MY_DATA as *const u32);
    }

    // Configure board hardware (Blue Pill user LED on PC13)
    gpio_init_pc13();
    // Configure SysTick as a periodic 1 s time base
    systick_init();

    // Main loop: poll SysTick and toggle LED on each tick
    loop {
        if unsafe { ptr::read_volatile(SYST_CSR) } & (1 << 16) != 0 {
            led_toggle_pc13();
        }
    }
}

fn init_data_bss() {
    unsafe {
        // Copy initialized data (.data) from load address in Flash to RAM
        let mut src = core::ptr::addr_of!(_sidata);
        let mut dst = core::ptr::addr_of_mut!(_glob_RW);
        let end_data = core::ptr::addr_of_mut!(_eglob_RW);

        while dst < end_data {
            ptr::write_volatile(dst, ptr::read_volatile(src));
            dst = dst.add(1);
            src = src.add(1);
        }

        // Zero-fill uninitialized data (.bss) in RAM
        let mut bss = core::ptr::addr_of_mut!(_glob_ZI);
        let end_bss = core::ptr::addr_of_mut!(_eglob_ZI);
        while bss < end_bss {
            ptr::write_volatile(bss, 0);
            bss = bss.add(1);
        }
    }
}

fn systick_init() {
    unsafe {
        // Stop timer and clear current value before reconfiguration
        ptr::write_volatile(SYST_CSR, 0);
        ptr::write_volatile(SYST_CVR, 0);
    }

    // Reset clock is HSI 8 MHz on STM32F103
    const RELOAD_VALUE: u32 = 8_000_000 - 1;
    unsafe {
        ptr::write_volatile(SYST_RVR, RELOAD_VALUE);
    }

    unsafe {
        // ENABLE=1, TICKINT=0, CLKSOURCE=core clock
        ptr::write_volatile(SYST_CSR, 0x5);
    }
}

fn gpio_init_pc13() {
    unsafe {
        // Enable GPIOC clock on APB2
        let apb2 = ptr::read_volatile(RCC_APB2ENR);
        ptr::write_volatile(RCC_APB2ENR, apb2 | (1 << 4));

        // PC13 = output push-pull, max speed 2 MHz
        let crh = ptr::read_volatile(GPIOC_CRH);
        let cleared = crh & !(0xF << 20);
        ptr::write_volatile(GPIOC_CRH, cleared | (0b0010 << 20));

        // Blue Pill LED is active-low, start it OFF
        let odr = ptr::read_volatile(GPIOC_ODR);
        ptr::write_volatile(GPIOC_ODR, odr | (1 << 13));
    }
}

fn led_toggle_pc13() {
    unsafe {
        ptr::write_volatile(GPIOC_ODR, ptr::read_volatile(GPIOC_ODR) ^ (1 << 13));
    }
}

// Place reset handler pointer in the vector table at index 1
#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static RESET_VECTOR: extern "C" fn() -> ! = _start;
