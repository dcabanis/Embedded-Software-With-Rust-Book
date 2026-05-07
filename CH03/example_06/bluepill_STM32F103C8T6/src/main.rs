#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use nb::block;

use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc::Config as RccConfig,
    serial::{Config as UartConfig, Serial},
};

// UART echo routine for STM32F1 HAL Tx/Rx halves
fn uart_echo(
    tx: &mut stm32f1xx_hal::serial::Tx<pac::USART1>,
    rx: &mut stm32f1xx_hal::serial::Rx<pac::USART1>,
) -> ! {
    let mut buf = [0u8; 1];

    loop {
        match block!(rx.read()) {
            Ok(byte) => {
                buf[0] = byte;
                let _ = block!(tx.write_u8(buf[0]));
                let _ = block!(tx.flush());
            }
            Err(_e) => {
                // In production you might count errors, reset the peripheral, etc
                // Here we just keep looping
            }
        }
    }
}

#[entry]
fn main() -> ! {
    // --- Peripherals --------------------------------------------------------
    let dp = pac::Peripherals::take().unwrap();

    // --- Clocks -------------------------------------------------------------
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Typical Blue Pill clocking: 8 MHz HSE -> 72 MHz SYSCLK
    rcc = rcc.freeze(
        RccConfig::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz()),
        &mut flash.acr,
    );
    // --- GPIOA / AFIO -------------------------------------------------------
    let _afio = dp.AFIO.constrain(&mut rcc);
    let mut gpioa = dp.GPIOA.split(&mut rcc);

    // USART1 pins:
    // - PA9  = TX (alternate function push-pull)
    // - PA10 = RX (floating input)
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // --- USART1 -------------------------------------------------------------
    // Note: constructor signatures vary slightly across HAL versions
    // The intent is always the same: supply USART1 + (tx, rx) + baud + clocks/rcc
    // Configure your terminal: 115200 baud, 8 data bits, no parity, 1 stop bit, no flow control
    let uart = Serial::new(
        dp.USART1,
        (tx, rx),
        UartConfig::default().baudrate(115_200.bps()),
        &mut rcc,
    );

    let (mut tx, mut rx) = uart.split();

    uart_echo(&mut tx, &mut rx);
}
