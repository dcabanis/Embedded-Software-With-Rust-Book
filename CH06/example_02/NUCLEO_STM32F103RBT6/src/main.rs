#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*, serial::Config};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc);

    // USART2 on the NUCLEO-F103RB is routed to the on-board ST-Link v2-1,
    // which exposes it as a virtual COM port (VCP) — no external USB-UART
    // adapter is needed. PA2 (TX) and PA3 (RX) are in the CRL register
    // (pins 0-7), unlike USART1's PA9/PA10 which use CRH (pins 8-15).
    let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx = gpioa.pa3;

    let mut serial = dp
        .USART2
        .serial((tx, rx), Config::default().baudrate(115_200.bps()), &mut rcc);

    writeln!(serial, "UART log message").unwrap();
    loop {}
}
