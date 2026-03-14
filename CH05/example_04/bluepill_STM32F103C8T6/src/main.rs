//! Displays "Hello world!" on an external SSD1306 OLED screen via I2C.
//!
//! Connect an SSD1306 display to the I2C1 bus:
//!   PB6 -> SCL
//!   PB7 -> SDA
#![no_main]
#![no_std]

use panic_halt as _;

use embedded_hal::i2c::I2c;

use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
};

use ssd1306::{
    mode::{TerminalDisplaySize, TerminalMode},
    prelude::*,
    I2CDisplayInterface, Ssd1306,
};

// A generic driver function: works with any I²C bus that implements the
// embedded_hal::i2c::I2c trait, regardless of the underlying hardware.
// Probes a device by sending a valid SSD1306 display-off command (0x00, 0xAE).
fn probe_device<I: I2c>(i2c: &mut I, addr: u8) -> bool {
    i2c.write(addr, &[0x00, 0xAE]).is_ok()
}

// Extension trait that fixes the broken fmt::Write impl in ssd1306 0.10.0,
// which only prints the last character of a string due to misuse of next_back().
trait PrintStr {
    fn print_str(&mut self, s: &str);
}

impl<DI, SIZE> PrintStr for Ssd1306<DI, SIZE, TerminalMode>
where
    DI: WriteOnlyDataCommand,
    SIZE: TerminalDisplaySize,
{
    fn print_str(&mut self, s: &str) {
        for c in s.chars() {
            let _ = self.print_char(c);
        }
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Setup I2C1 using PB6/PB7 at 400 kHz (fast mode)
    let scl = gpiob.pb6;
    let sda = gpiob.pb7;

    let mut i2c = BlockingI2c::new(
        dp.I2C1,
        (scl, sda),
        Mode::fast(400.kHz(), DutyCycle::Ratio2to1),
        &mut rcc,
        1000,
        10,
        1000,
        1000,
    );

    // probe_device is written against embedded_hal::i2c::I2c — it knows nothing
    // about the bluepill or stm32f1xx-hal. If the display is not found, we halt
    // immediately.
    if !probe_device(&mut i2c, 0x3C) {
        loop {}
    }

    // Hand the bus over to the display driver.
    let interface = I2CDisplayInterface::new(i2c);
    let mut disp =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();

    disp.init().unwrap();
    let _ = disp.clear();

    disp.print_str("SSD1306 found!\n");
    disp.print_str("Hello world!");

    loop {
        continue;
    }
}
