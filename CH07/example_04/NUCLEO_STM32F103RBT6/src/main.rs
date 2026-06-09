#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use defmt_rtt as _;
use heapless::{String, Vec};
use panic_probe as _;

const LINE_LEN: usize = 64;
const RESP_LEN: usize = 32;

// Append one byte to the line buffer.  Returns Some(response) when a
// complete line has been processed, None while still accumulating.
fn feed_byte(
    line: &mut Vec<u8, LINE_LEN>,
    byte: u8,
) -> Option<String<RESP_LEN>> {
    if byte == b'\n' {
        let response = handle_command(line.as_slice());
        line.clear();
        return Some(response);
    }

    if line.push(byte).is_err() {
        // The line buffer filled before a newline arrived.
        // Discard the partial line and report the overflow.
        line.clear();
        let mut err = String::<RESP_LEN>::new();
        let _ = err.push_str("ERR overflow");
        return Some(err);
    }

    None
}

fn handle_command(line: &[u8]) -> String<RESP_LEN> {
    let mut response = String::<RESP_LEN>::new();
    match line {
        b"HELLO" => {
            let _ = response.push_str("HOW DO YOU DO?");
        }
        b"VERSION" => {
            let _ = write!(response, "v{}.{}", 1, 0);
        }
        _ => {
            let _ = response.push_str("ERR unknown");
        }
    }
    response
}

#[entry]
fn main() -> ! {
    let mut line: Vec<u8, LINE_LEN> = Vec::new();

    // Simulate a UART byte stream: three well-formed commands.
    for &byte in b"HELLO\nVERSION\nFOO\n" {
        if let Some(response) = feed_byte(&mut line, byte) {
            defmt::info!("rx> {}", response.as_str());
        }
    }

    // Trigger the overflow path: LINE_LEN + 1 bytes of 'X' without a newline.
    // The 65th byte causes push() to fail; feed_byte clears the buffer and
    // returns "ERR overflow" immediately, before the newline arrives.
    for &byte in &[b'X'; LINE_LEN + 1] {
        if let Some(response) = feed_byte(&mut line, byte) {
            defmt::info!("rx> {}", response.as_str());
        }
    }

    loop {}
}
