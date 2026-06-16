#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub static MY_DATA: u32 = 0xDEADBEEF;

const SYSTICK: u32 = 0xE000_E010;
const SYST_CSR: *mut u32 = SYSTICK as *mut u32;
const SYST_RVR: *mut u32 = (SYSTICK + 0x04) as *mut u32;
const SYST_CVR: *mut u32 = (SYSTICK + 0x08) as *mut u32;

const RCC_APB2ENR: *mut u32 = 0x4002_1018 as *mut u32;
const GPIOA_CRL: *mut u32 = 0x4001_0800 as *mut u32;
const GPIOA_ODR: *mut u32 = 0x4001_080C as *mut u32;

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
        let _ = core::ptr::read_volatile(&MY_DATA as *const u32);
    }

    // Configure board hardware (NUCLEO-F103RB user LED LD2 on PA5)
    gpio_init_pa5();
    // Configure SysTick as a periodic 1 s time base
    systick_init();

    // Main loop: poll SysTick and toggle LED on each tick
    loop {
        if unsafe { SYST_CSR.read_volatile() } & (1 << 16) != 0 {
            led_toggle_pa5();
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
            dst.write_volatile(src.read_volatile());
            dst = dst.add(1);
            src = src.add(1);
        }

        // Zero-fill uninitialized data (.bss) in RAM
        let mut bss = core::ptr::addr_of_mut!(_glob_ZI);
        let end_bss = core::ptr::addr_of_mut!(_eglob_ZI);
        while bss < end_bss {
            bss.write_volatile(0);
            bss = bss.add(1);
        }
    }
}

fn systick_init() {
    unsafe {
        // Stop timer and clear current value before reconfiguration
        SYST_CSR.write_volatile(0);
        SYST_CVR.write_volatile(0);
    }

    // Reset clock is HSI 8 MHz on STM32F103
    const RELOAD_VALUE: u32 = 8_000_000 - 1;
    unsafe {
        SYST_RVR.write_volatile(RELOAD_VALUE);
    }

    unsafe {
        // ENABLE=1, TICKINT=0, CLKSOURCE=core clock
        SYST_CSR.write_volatile(0x5);
    }
}

fn gpio_init_pa5() {
    unsafe {
        // Enable GPIOA clock on APB2
        let apb2 = RCC_APB2ENR.read_volatile();
        RCC_APB2ENR.write_volatile(apb2 | (1 << 2));

        // PA5 = output push-pull, max speed 2 MHz
        let crl = GPIOA_CRL.read_volatile();
        let cleared = crl & !(0xF << 20);
        GPIOA_CRL.write_volatile(cleared | (0b0010 << 20));

        // Nucleo LD2 is active-high, start it OFF
        let odr = GPIOA_ODR.read_volatile();
        GPIOA_ODR.write_volatile(odr & !(1 << 5));
    }
}

fn led_toggle_pa5() {
    unsafe {
        GPIOA_ODR.write_volatile(GPIOA_ODR.read_volatile() ^ (1 << 5));
    }
}

// Place reset handler pointer in the vector table at index 1
#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static RESET_VECTOR: extern "C" fn() -> ! = _start;
