# example_03_nucleo, ITM Logging with `iprintln!()` (NUCLEO-F103RB)

This example demonstrates ITM (Instrumentation Trace Macrocell) output on the
STM32F103C8T6 (Blue Pill). The firmware writes `"ITM log message"` to ITM
stimulus port 0 via the `iprintln!()` macro from the `cortex-m` crate, then
spins in an infinite loop.

## Hardware setup

The NUCLEO-F103RB carries a genuine on-board ST-Link v2.1 that supports SWO.
However, the SWO signal is **not connected by default** between the
STM32F103RB target and the ST-Link section of the board. A single jumper wire
is required.

### Required jumper

| From (target MCU) | To (ST-Link connector) |
|---|---|
| **PB3** | **CN4 pin 6** (SWO) |

PB3 is the SWO output on the STM32F103. CN4 pin 6 is the SWO input to the on-board ST-Link v2.1.

Without this wire, the probe cannot receive any ITM data and no output will
appear.

## How ITM output is captured

OpenOCD configures the TPIU peripheral to encode ITM packets in UART/NRZ
format and write them to a capture file. The `itmdump` tool then decodes that
file and prints the messages. No GDB session is needed.

### Install itmdump

```sh
cargo install itm
```

## Build

```sh
cargo build
```

## Run

The firmware prints its message once immediately after reset.

**Step 1  Terminal 1:** flash the firmware and start SWO capture:

```sh
./run_openocd.sh
```

Wait until OpenOCD has finished flashing and the board has reset. The script
creates `/tmp/itm.txt` and keeps OpenOCD running to capture SWO data.

**Step 2  Terminal 2:** start the ITM decoder:

```sh
itmdump -F -f /tmp/itm.txt
```

The `-F` flag keeps the file open and prints new output as OpenOCD appends
to it. Start this only after the script has run, so the capture file already
exists.

**Step 3  press the RESET button** (B2, the black button) on the NUCLEO board.

The board re-executes `main()`, sends the ITM packet, and `itmdump` prints it:

```
ITM log message
```

Press `Ctrl-c` in Terminal 1 to stop OpenOCD when done.

> **Why the reset?** The firmware sends the message once at startup. By the
> time `itmdump` is ready in Terminal 2, that first startup has already passed.
> Pressing RESET triggers a clean re-run that `itmdump` can observe.

## CPU clock note

The `tpiu config` command in `run_openocd.sh` requires the actual CPU clock
frequency so it can set the correct SWO baud rate. This firmware does not
configure the PLL, so the core runs at the STM32F103 HSI default of 8 MHz.
The script uses `CPU_CLOCK_HZ=8000000`. If you modify `main.rs` to configure
a different clock, update that variable accordingly.

## probe-rs note

`cargo run` via probe-rs does not currently capture ITM/SWO output
automatically. The OpenOCD + `itmdump` path above is the recommended
no-GDB workflow for this example.
