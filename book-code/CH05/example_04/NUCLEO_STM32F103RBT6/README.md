# example_04  SSD1306 OLED display over I2C on the NUCLEO-F103RB

This example drives an SSD1306 OLED display using the `ssd1306` platform-agnostic
driver crate over I2C1. I2C1 is remapped to PB8 (SCL) and PB9 (SDA) to align
with the Arduino D15/D14 header pins on the NUCLEO board. The firmware probes
the display, initialises it in terminal mode, and writes `"SSD1306 found!"` and
`"Hello world!"` to the screen. A custom `PrintStr` trait works around a defect
in the `ssd1306` crate's `fmt::Write` implementation.

## I2C pin wiring

| Signal | Pin | Arduino header |
|--------|-----|----------------|
| SCL    | PB8 | D15            |
| SDA    | PB9 | D14            |

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
