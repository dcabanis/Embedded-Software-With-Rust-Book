// Displays "Hello world!" on an external SSD1306 OLED screen via I2C.
//
// Connect an SSD1306 display to the I2C1 bus (remapped pins):
//   D15 / PB8 -> SCL
//   D14 / PB9 -> SDA
#![no_main]
#![no_std]

use panic_halt as _;

use embedded_hal::i2c::I2c;

use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
    rcc::Config,
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

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.freeze(Config::default(), &mut flash.acr);
    let mut afio = dp.AFIO.constrain(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Setup I2C1 using D15/PB8 and D14/PB9 at 400 kHz (fast mode).
    // PB8/PB9 are the remapped I2C1 pins (remap=1); passing dp.I2C1.remap()
    // forces R=1 so the HAL writes the correct value into AFIO_MAPR.
    let scl = gpiob.pb8;
    let sda = gpiob.pb9;

    let mut i2c: BlockingI2c<pac::I2C1> = BlockingI2c::new(
        dp.I2C1.remap::<1>(&mut afio.mapr),
        (scl, sda),
        Mode::fast(400.kHz(), DutyCycle::Ratio2to1),
        &mut rcc,
        1000,
        10,
        1000,
        1000,
    );

    // probe_device is written against embedded_hal::i2c::I2c it knows nothing
    // about the NUCLEO or stm32f1xx-hal. If the display is not found, we halt
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
