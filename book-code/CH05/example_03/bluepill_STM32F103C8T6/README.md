# example_03 — 1 Hz LED blink using stm32f1xx-hal on the Blue Pill

This example demonstrates the top HAL layer of the embedded-Rust stack. The
`stm32f1xx-hal` crate configures the RCC, takes ownership of the GPIOC
peripheral, and returns a typed push-pull output pin for PC13 (active-low). A
HAL `Delay` provider built from SysTick abstracts the timing. The result is a
concise, readable 1 Hz blink loop that hides all register-level detail,
showing the contrast with example_01 (raw pointers) and example_02 (PAC).

## Build

```sh
cargo build
```

## Run — Path A: probe-rs

```sh
cargo run
```

## Run — Path B: OpenOCD

```sh
./run_openOCD.sh
```
