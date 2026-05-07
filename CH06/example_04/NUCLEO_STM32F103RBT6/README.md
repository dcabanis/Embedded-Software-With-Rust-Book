# example_04  RTT Logging with `rtt-target`

This example demonstrates Real-Time Transfer (RTT) output on the
STM32F103RBT6 (NUCLEO-F103RB). The firmware initialises an RTT channel with
`rtt_init_print!()`, writes `"RTT log message"` via `rprintln!()`,
then spins in an infinite loop.

RTT works by placing a control block in RAM that the debug probe reads in the
background over SWD while the firmware runs. No extra hardware pins are
required beyond the standard SWD connection (SWDIO, SWCLK, GND) already used
for flashing.

## Probe requirement

RTT requires the debug probe to remain connected and active while the firmware
is running. Unlike UART (example_02), RTT output disappears if the probe is
unplugged. The NUCLEO-F103RB's on-board ST-Link v2-1 handles both flashing
and RTT without any additional hardware.

## Build

```sh
cargo build
```

or for a release build:

```sh
cargo build --release
```

## Run Path A: probe-rs

`probe-rs` is the complete solution for RTT: it flashes the firmware, attaches
to the target, locates the RTT control block in RAM, and streams output to the
terminal — all in a single command.

```sh
cargo run --release
```

Because probe-rs attaches before the firmware starts executing, it catches the
very first `rprintln!()` call with no timing concerns. Expected output:

```
RTT log message
```

The program then loops forever. Press `Ctrl-c` to stop.

## Run Path B: OpenOCD RTT server (no GDB)

OpenOCD's RTT commands are pure TCL and run entirely via `-c` flags (no GDB
session is required). The script flashes the firmware, sets up the RTT control
block search, starts the RTT poller, and opens a TCP server on port 9090.

**Terminal 1** — flash and start the RTT server:

```sh
cargo build
./run_openocd.sh
```

OpenOCD stays running. When it prints `Listening on port 9090`, the server
is ready.

**Terminal 2** — connect and read RTT output:

```sh
telnet localhost 9090
```

Expected output:

```
RTT log message
```

The message is sent once at startup. If you connected after the board had
already reset, press the RESET button to trigger another run.

Press `Ctrl-c` in Terminal 1 to stop OpenOCD. Use `Ctrl-]` then `quit` to
exit `telnet`.
