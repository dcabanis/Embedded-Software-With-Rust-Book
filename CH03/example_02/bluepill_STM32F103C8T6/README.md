# example_02  Cargo feature flags for panic strategy on the Blue Pill

This example demonstrates compile-time feature selection for the panic handler.
Two mutually exclusive features are defined in `Cargo.toml`: the default `panic-halt` feature links `panic-halt`, and the `semihosting` feature links `panic-semihosting` plus `cortex-m-semihosting`. A `#[cfg]` compile-time check enforces that exactly one feature is active. The firmware then triggers a panic so the chosen strategy can be observed in practice.

## Build

Default build (halt on panic, no debugger needed):

```sh
cargo build
```

Build with semihosting panic (shows message via debugger):

```sh
cargo build --no-default-features --features semihosting
```

## Run  Path A: probe-rs

```sh
cargo run --no-default-features --features semihosting
```

## Run  Path B: OpenOCD

```sh
cargo build --no-default-features --features semihosting
./run_openOCD.sh
```

The script builds with the `semihosting` feature, flashes the ELF, enables semihosting, and waits for the core to halt before shutting down OpenOCD.  Note that `arm semihosting_fileio enable` is intentionally omitted: that mode forwards semihosting I/O to a connected GDB client, so without GDB the output is silently dropped.  Basic semihosting (`arm semihosting enable` only) has OpenOCD handle the writes directly and print them to the terminal.

## Expected output — Path A (probe-rs)

probe-rs captures semihosting output and appends a backtrace when the firmware halts on the panic breakpoint:

```sh
Semihosting I/O is enabled.
panicked at src/main.rs:36:5:
This will halt (panic-halt) or print via semihosting (semihosting feature).
Firmware exited unexpectedly: Breakpoint(Unknown)
Core 0
    Frame 0: __bkpt @ 0x08000f60 inline
       ./asm/lib.rs:48:1
    Frame 1: __bkpt @ 0x0000000008000f60
       ./asm/lib.rs:51:17
    Frame 2: panic @ 0x08000496
       /home/dcabanis/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/panic-semihosting-0.6.0/src/lib.rs:84:15
```

## Expected output — Path B (OpenOCD)

OpenOCD prints semihosting output inline with its own log.  The firmware messages appear after `** Verified OK **`:

```sh
Semihosting I/O is enabled.
panicked at src/main.rs:36:5:
This will halt (panic-halt) or print via semihosting (semihosting feature).
```

