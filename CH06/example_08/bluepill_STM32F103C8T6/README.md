# example_08  DWT Cycle-Counter Profiling

This example demonstrates cycle-accurate timing measurement using the
Cortex-M Data Watchpoint and Trace (DWT) unit.
The firmware enables the DWT cycle counter, times two `nop`-loop
tasks of different lengths, and reports the elapsed cycles over RTT.

## Build

```sh
cargo build
```

## Run

**Option A, probe-rs (simplest):**

```sh
cargo run --release
```

probe-rs flashes the binary and prints RTT output directly in the terminal.

> The `[profile.release]` section in `Cargo.toml` sets `opt-level = 1`.
> This is required to preserve the `nop` loops. At the default `opt-level =
> 3` the compiler eliminates them entirely and both tasks report 0 cycles.

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

```
Starting DWT profiling...
Task A took 1500000 cycles
Task B took 3000000 cycles
```

> The cyles number are dependent on your setup.

Task B takes approximately twice as long as Task A because its loop runs
twice as many iterations (1 000 000 vs 500 000). Absolute cycle counts
depend on the optimization level; at `opt-level = 1`.

## Why `wrapping_sub`

`CYCCNT` is a free-running 32-bit counter. `wrapping_sub` produces the
correct elapsed count even when the counter wraps between the two reads.

## Why `opt-level = 1`

At `opt-level = 3`, LLVM recognises that the `nop` loops have no
observable side effects and removes them. `opt-level = 1` applies only
basic optimisations and preserves the loops. In production profiling you
would wrap real code in DWT measurements rather than nop loops, so this
constraint does not apply.
