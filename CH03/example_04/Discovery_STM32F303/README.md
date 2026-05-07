# example_04  LSM303 accelerometer via I2C and embedded-hal on the STM32F3 Discovery

This example demonstrates the portability of `embedded-hal`-based drivers by
running the same `lsm303agr` accelerometer driver on a different hardware target —
the STM32F3 Discovery board (STM32F303VCT6). The firmware configures I2C1,
initialises the sensor, and drives an onboard LED based on X-axis acceleration,
identical in logic to the STM32F103 variant but using `stm32f3xx-hal` instead of
`stm32f1xx-hal`. Comparing the two projects illustrates how the `embedded-hal`
abstraction decouples driver code from the underlying hardware.

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
