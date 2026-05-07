# example_04  Interrupt-safe shared data with Mutex<Cell<>> on the NUCLEO-F103RB

This example demonstrates how to safely share a variable between the main
execution context and an interrupt handler using
`cortex_m::interrupt::Mutex<Cell<u32>>`. A monotonic seconds counter is stored in a `static` and incremented by the `SysTick` handler every second. All ccesses go through `interrupt::free()` critical sections, preventing data races on Cortex-M. The current count is printed via semihosting on each tick.
**A debugger must be attached to see the semihosting output.**

## Build

```sh
cargo build
```

## Run  Path A: probe-rs

```sh
cargo run
```

## Run  Path B: OpenOCD

```sh
./run_openOCD.sh
```
