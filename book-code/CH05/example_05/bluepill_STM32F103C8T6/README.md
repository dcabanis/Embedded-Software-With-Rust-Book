# example_05  Inline and global assembly for MMIO and timing on the Blue Pill

This example demonstrates three ways to embed assembly in embedded-Rust firmware.
A `global_asm!()` block defines a counted delay loop. `core::arch::asm!()` is
used inline to manipulate `PRIMASK` (disabling interrupts) and to toggle the
PC13 LED by writing directly to the GPIOC BSRR register. `cortex_m::asm::nop()`
provides short NOP-based busy-wait delays. The example shows how to mix Rust and
assembly for performance-critical or hardware-specific code paths while still
leveraging the HAL for initial GPIO configuration.

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
