# example_01  Minimal HAL project with semihosting on the Blue Pill

This is the chapter-3 starting-point project. This example prints
`"Hello, world!"` through semihosting. The purpose is to show the canonical
embedded-Rust project structure, `#![no_std]` / `#![no_main]`, runtime entry
point, peripheral singleton, and a HAL import, before adding any real hardware
interaction. **A debugger must be attached to see the semihosting output.**

## Build

```sh
cargo build
```

## Terminal 1: Start the on-chip debugger

Start OpenOCD with semihosting enabled in a separate terminal:

```sh
openocd -f interface/stlink.cfg -f target/stm32f1x.cfg \
  -c "init" -c "halt" -c "arm semihosting enable"
```

## Terminal 2: Flash and debug

Then flash and run with the GDB runner in another terminal:

```sh
cargo run
```

Semihosting output appears in the OpenOCD terminal.

> Note: I you are too long to establish the GDB connection, openOCD with time out and you will get an error message like this one:
>
> ```Error: timed out while waiting for target halted```
>
> At this point you will have to redo the operation once again.

## Basic GDB commands 

```
step

next

continue

Ctrl+c

quit
```
