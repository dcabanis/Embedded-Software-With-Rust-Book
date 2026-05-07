# example_09  SysTick-Based Cycle Measurement

This example demonstrates SysTick-based timing as a fallback for Cortex-M
cores that lack a DWT cycle counter (Cortex-M0/M0+). The firmware configures
SysTick as a free-running down-counter, measures `do_work()` by reading the
counter before and after, and reports the elapsed cycles over RTT on the
STM32F103RBT6 (NUCLEO-F103RB).

The NUCLEO-F103RB is a Cortex-M3 and does have DWT, so the DWT approach from
example_08 is preferable on this hardware. This example demonstrates the
SysTick technique that would be used on Cortex-M0/M0+/M23 parts; the same
code runs unchanged on a Cortex-M3.

## Build

```sh
cargo build
```

## Run

**Option A, probe-rs (simplest):**

```sh
cargo run --release
```

> The `[profile.release]` section in `Cargo.toml` sets `opt-level = 1` to
> preserve the `nop` loop inside `do_work()`. At the default `opt-level = 3`
> the compiler eliminates the loop and reports 0 cycles.

probe-rs flashes the binary and prints RTT output directly in the terminal.

**Option B, OpenOCD + GDB + telnet:**

Terminal 1, flash and leave OpenOCD running:

```sh
cargo build
./run_openocd.sh
```

Terminal 2, connect GDB and start the RTT server:

```sh
gdb-multiarch target/thumbv7m-none-eabi/debug/app
```

```
(gdb) target extended-remote :3333
(gdb) monitor rtt setup 0x20000000 0x5000 "SEGGER RTT"
(gdb) monitor rtt start
(gdb) monitor rtt server start 9090 0
(gdb) continue
```

Terminal 3, read RTT output:

```sh
telnet localhost 9090
```

**Shutting down (Option B):**

- Terminal 3 (telnet): `Ctrl-]` then type `quit` and press Enter
- Terminal 2 (GDB): `(gdb) quit`
- Terminal 1 (OpenOCD): `Ctrl-C`

## Expected output

> The cycles value will depend on your setup.

```
Starting SysTick profiling...
do_work() took 30000 cycles
```

At `opt-level = 1` on the STM32F103RBT6's 8 MHz HSI clock, `do_work()`
(10 000 `nop` iterations) takes roughly 30 000 cycles. The exact count varies
with optimisation level.

## SysTick limitations

The 24-bit reload value (`0x00FF_FFFF`) limits the maximum measurable
interval to 2^24 / 8 000 000 ≈ **2.1 seconds** at 8 MHz HSI. The
`wrapping_sub` / mask pattern used here produces correct results only when
the counter has not wrapped more than once during the measurement. For
regions that may span multiple reload periods, a SysTick interrupt handler
must count wraps; this example does not demonstrate that technique.
