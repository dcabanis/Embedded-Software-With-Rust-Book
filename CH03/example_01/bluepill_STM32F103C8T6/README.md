# example_01  Minimal HAL project with semihosting on the Blue Pill

This is the chapter-3 starting-point project. The firmware takes STM32 device
peripherals via `stm32f1xx_hal::pac::Peripherals::take()` and prints
`"Hello, world!"` through semihosting. The purpose is to show the canonical
embedded-Rust project structure, `#![no_std]` / `#![no_main]`, runtime entry
point, peripheral singleton, and a HAL import, before adding any real hardware
interaction. **A debugger must be attached to see the semihosting output.**

## Build

```sh
cargo build
```

## Run  Path A: probe-rs

`probe-rs` handles semihosting natively. Output appears in the same terminal.

```sh
cargo run
```

## Run  Path B: OpenOCD

Start OpenOCD with semihosting enabled in a separate terminal:

```sh
openocd -f interface/stlink.cfg -f target/stm32f1x.cfg \
  -c "init" -c "halt" -c "arm semihosting enable"
```

Then flash and run with the GDB runner in another terminal:

```sh
cargo run
```

Semihosting output appears in the OpenOCD terminal.
