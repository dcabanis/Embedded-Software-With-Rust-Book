# example_02  UART Logging with `writeln!()`

This example demonstrates standalone UART output on the STM32F103RBT6
(NUCLEO-F103RB). The firmware configures USART2 on PA2 (TX) and PA3 (RX) at
115 200 baud, writes `"UART log message"` once via the `writeln!()` macro,
then spins in an infinite loop. No debugger is required after flashing. The
message appears on any serial terminal connected to the UART.

## Hardware setup

The NUCLEO-F103RB routes USART2 (PA2/PA3) through the on-board ST-Link v2-1,
which exposes it as a USB CDC virtual COM port (VCP). **No external
USB-to-serial adapter is needed.** Plug the NUCLEO into a USB port and the
VCP appears automatically alongside the SWD debug interface.

On Linux the VCP is typically `/dev/ttyACM0`. If you have other USB CDC
devices connected it may be `/dev/ttyACM1` or higher; check with
`ls /dev/ttyACM*` before and after plugging in the board.

## UART pinout

| NUCLEO pin | USART2 function |
|------------|-----------------|
| PA2        | TX (to ST-Link) |
| PA3        | RX (from ST-Link)|

These pins are connected internally on the board; no jumper wires are needed.
The Morpho connector also exposes PA2/PA3 if you want to connect an external
logic analyser or serial sniffer.

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
picocom -b 115200 /dev/ttyACM0
```

Press `Ctrl-a` then `Ctrl-x` to quit.

### Linux / macOS screen

```sh
screen /dev/ttyACM0 115200
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
