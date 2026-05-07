# example_04  LSM303 accelerometer via I2C and embedded-hal on the NUCLEO-F103RB

This is the NUCLEO-F103RB variant of the LSM303 accelerometer example. The
firmware configures I2C1, initialises the LSM303AGR sensor via the `lsm303agr`
driver crate, and toggles LED LD2 on PA5 when the X-axis acceleration crosses a
threshold. It demonstrates how the same `embedded-hal`-backed driver works
unchanged on both the Blue Pill and the NUCLEO board, with only the GPIO pin and
HAL initialisation differing between projects.

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
