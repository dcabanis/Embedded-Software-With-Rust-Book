# example_01 — Minimal project template with semihosting on the NUCLEO-F103RB

This is the minimal "hello world" template for an STM32F103 project. The
firmware prints `"Fly like a bird!"` once via the `hprintln!()` semihosting
macro and then loops forever. No peripherals are configured; all I/O is handled
by the attached debugger on the host side. **Without an active debugger the
firmware will hang at the semihosting call.**

## Build

```sh
cargo build --release
```

## Run — Path A: probe-rs

`probe-rs` handles semihosting natively. Output appears in the terminal that
invokes `cargo run`.

```sh
cargo run --release
```

Expected output:

```
Fly like a bird!
```

## Run — Path B: OpenOCD

```sh
./run_openOCD.sh
```

Semihosting output appears in the OpenOCD terminal.
