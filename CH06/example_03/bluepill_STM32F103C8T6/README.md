# example_03, ITM Logging with `iprintln!()`

This example demonstrates ITM (Instrumentation Trace Macrocell) output on the
STM32F103C8T6 (Blue Pill). The firmware writes `"ITM log message"` to ITM
stimulus port 0 via the `iprintln!()` macro from the `cortex-m` crate, then
spins in an infinite loop.

## Probe requirement, read before proceeding

ITM output is carried over the **SWO** (Serial Wire Output) pin. SWO is a
separate signal from the SWD debug pins (SWDIO / SWCLK) and must be supported
by both the debug probe and its firmware.

**ST-Link v2.1 clones do not support SWO.** The SWO pin is not routed to the
connector on these boards. This example will flash and run correctly, but no
ITM output will be captured with a clone probe.

Probes known to support SWO:

| Probe | SWO support |
|---|---|
| Genuine ST-Link v2 / v2.1 (STMicroelectronics) | Yes |
| ST-Link v3 | Yes |
| CMSIS-DAP v2 | Depends on implementation |
| J-Link | Yes |

If you only have an ST-Link clone, refer to example_02 (UART logging) for a
standalone logging approach that has no probe dependency at runtime.

## NUCLEO-F103RB alternative

If you are using a NUCLEO-F103RB board instead of the Blue Pill, a dedicated
project is provided in `../NUCLEO_STM32F103RBT6/`. The NUCLEO board carries a
genuine on-board ST-Link v2.1 that does support SWO, but requires a jumper
wire from PB3 (MCU) to CN4 (board) pin 6 to route the SWO signal to the probe. See that
project's README for the full setup and wiring instructions.

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

The firmware prints its message once immediately after reset. Because of this,
the order of steps matters, follow them exactly.

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

**Step 3  press the RESET button** on the board.

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
