# example_04  SSD1306 OLED display over I2C on the Blue Pill

This example drives an SSD1306 OLED display using the `ssd1306` platform-agnostic
driver crate over I2C1. The firmware probes the display, initialises it in
terminal mode, and writes `"SSD1306 found!"` and `"Hello world!"` to the screen.
I2C1 is configured on PB6 (SCL) and PB7 (SDA). The example also introduces a
custom `PrintStr` trait to work around a defect in the `ssd1306` crate's
`fmt::Write` implementation, demonstrating how to patch third-party driver
limitations without forking.

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
