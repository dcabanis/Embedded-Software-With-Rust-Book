# example_02  UART Logging with `writeln!()`

This example demonstrates standalone UART output on the STM32F103C8T6 (Blue
Pill). The firmware configures USART1 on PA9 (TX) and PA10 (RX) at 115 200
baud, writes `"UART log message"` once via the `writeln!()` macro, then spins
in an infinite loop. No debugger is required after flashing. The message
appears on any serial terminal connected to the UART pins.

## Hardware setup

This example assumes:
- An **ST-Link v2.1** (or clone) connected via SWD for flashing and debugging.
  ST-Link v2.1 clones expose only the SWD signals; they do not provide a
  virtual serial port.
- A separate **USB-to-serial adapter** (e.g. FTDI FT232R, CH340) wired to
  USART1 for UART output. This adapter appears as `/dev/ttyUSB0` on Linux.

## UART pinout

| Blue Pill pin | USB-serial adapter pin |
|---------------|------------------------|
| PA9  (TX)     | RX                     |
| PA10 (RX)     | TX                     |
| GND           | GND                    |

Connect the adapter to your host machine before opening the terminal.

## Build

```sh
cargo build
```

or for a release build:

```sh
cargo build --release
```

## Run  Path A: probe-rs

```sh
cargo run --release
```

probe-rs flashes the firmware and resets the board. The UART message will
appear in your serial terminal.

## Run  Path B: OpenOCD

```sh
cargo build
./run_openocd.sh
```

`run_openocd.sh` flashes the firmware and resets the board, then exits.
The UART message appears in the serial terminal once the board starts running.

## Viewing the output

> **Timing matters.** The firmware sends the message once at startup and then
> loops silently. Open the terminal *before* flashing or resetting the board,
> otherwise the message is gone before the terminal is listening.

### Linux picocom (recommended on Ubuntu)

```sh
picocom -b 115200 /dev/ttyUSB0
```

Press `Ctrl-a` then `Ctrl-x` to quit.

### Linux / macOS screen

```sh
screen /dev/ttyUSB0 115200
```

Press `Ctrl-a` then `k` to quit. Note that `screen` may miss a one-shot
startup message if it finishes initialising after the board has already reset.

### Windows

Open PuTTY, select *Serial*, set the COM port shown in Device Manager, and
set the baud rate to 115 200.

## Expected output

```
UART log message
```

The board sends the message once at startup and then loops forever.
