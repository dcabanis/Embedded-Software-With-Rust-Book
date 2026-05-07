# example_05 — Platform-agnostic blink with OutputPin on the NUCLEO-F103RB

This example demonstrates writing MCU-agnostic driver logic using the
`embedded-hal` `OutputPin` trait. A generic `blink_led()` function accepts any
`OutputPin` and toggles it in a loop. The NUCLEO's onboard LED LD2 on PA5 is
active-high, so no polarity adapter is needed and the generic function can be
called directly with the HAL pin handle.

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
