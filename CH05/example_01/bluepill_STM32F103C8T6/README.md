# example_01  Raw-pointer MMIO LED blink on the Blue Pill

This is the chapter-5 baseline: a bare-metal LED blinker written entirely with
raw pointer MMIO, using no HAL or PAC. The firmware initialises data and BSS
sections, enables GPIOC via RCC, and configures PC13 as a push-pull output.
SysTick is polled (not interrupt-driven) to produce a 1-second toggle period.
The goal is to establish a reference point before showing how PAC and HAL crates
simplify the same task in the subsequent examples.

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
