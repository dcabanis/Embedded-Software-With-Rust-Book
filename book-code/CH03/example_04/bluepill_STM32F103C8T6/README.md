# example_04  LSM303 accelerometer via I2C and embedded-hal on the Blue Pill

This example demonstrates platform-agnostic hardware driver usage via the
`embedded-hal` I2C trait. The firmware configures I2C1, initialises an LSM303AGR
accelerometer in Normal mode at 50 Hz ODR using the `lsm303agr` driver crate,
and toggles the onboard LED on PC13 whenever the X-axis acceleration exceeds a
threshold. It shows that the same high-level driver works across STM32 families
because it depends only on the `embedded-hal` I2C abstraction.

## I2C pin wiring

| Signal | Pin |
|--------|-----|
| SCL    | PB6 |
| SDA    | PB7 |

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
