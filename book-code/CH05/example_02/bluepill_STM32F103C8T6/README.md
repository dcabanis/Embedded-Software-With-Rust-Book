# example_02  Type-safe MMIO via PAC (stm32f1) on the Blue Pill

This example replaces the raw `*mut u32` writes from example_01 with the
type-safe register API of the `stm32f1` Peripheral Access Crate (PAC). GPIO and
RCC are configured through the PAC's `modify()` and `write()` closures,
eliminating raw pointer casts while producing the same result: PC13 (active-low)
toggled every second via SysTick.

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
