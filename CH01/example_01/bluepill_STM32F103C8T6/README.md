# example_01  SysTick singleton on the Blue Pill

This example demonstrates the Cortex-M singleton pattern for peripheral access.
The firmware acquires ownership of `CorePeripherals` via
`cortex_m::Peripherals::take()`, configures the SysTick timer for a 1-second
periodic interrupt at 8 MHz, and parks the CPU in WFI in the main loop. A
`#[exception]`-annotated `SysTick` handler fires on each tick, illustrating how
`cortex-m-rt` places exception handlers in the correct vector table slots.

## Build

```sh
cargo build
```

or for a release build:

```sh
cargo build --release
```

## Run — Path A: probe-rs

```sh
cargo run
```

## Run — Path B: OpenOCD

```sh
./run_openOCD.sh
```
