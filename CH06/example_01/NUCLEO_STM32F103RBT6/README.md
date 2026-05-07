# example_01  Semihosting with `hprintln!()`

This example demonstrates semihosting output on the STM32F103RBT6 (NUCLEO-F103RB).
The firmware prints `"Hello from semihosting"` once via the `hprintln!()` macro
from the `cortex-m-semihosting` crate, then spins in an infinite loop. No
peripherals are configured; all I/O is handled by the debugger on the host side.

## Important: debugger required

Semihosting works by executing a `BKPT 0xAB` instruction that traps execution
and asks the attached debugger to perform host-side I/O on the firmware's behalf.
**Without an active debugger the firmware will hang at the `hprintln!()` call.**
On Cortex-M3 (Armv7-M, which the STM32F103 uses) the trap does not escalate to
HardFault when no debugger is present  (the core simply stalls). This is different
from Cortex-M0/M0+ (Armv6-M) and Cortex-M23 (Armv8-M baseline) targets, where
the same situation causes an immediate HardFault.

## Build

```sh
cargo build
```

or for a release build:

```sh
cargo build --release
```

## Run  Path A: probe-rs

`probe-rs` handles semihosting natively. Output appears in the terminal that
invokes `cargo run`.

```sh
cargo run --release
```

Expected output:

```
Hello from semihosting
```

The program then loops forever; press Ctrl-C to stop `probe-rs`.

## Run  Path B: OpenOCD

`run_openocd.sh` flashes the firmware and starts OpenOCD with semihosting
already enabled (`arm semihosting enable` is part of the script's init
sequence). Semihosting output appears directly in that terminal.

```sh
cargo build
./run_openocd.sh
```

Expected output in the OpenOCD terminal:

```
Hello from semihosting
```

Press Ctrl-C to stop OpenOCD.
