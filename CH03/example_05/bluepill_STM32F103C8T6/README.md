# example_05  Platform-agnostic blink with OutputPin and ActiveLow adapter on the Blue Pill

This example demonstrates writing MCU-agnostic driver logic using the
`embedded-hal` `OutputPin` trait. A generic `blink_led()` function accepts any
`OutputPin` and toggles it in a loop. Because the Blue Pill's onboard LED on
PC13 is active-low, an `ActiveLow` new type wrapper is introduced to invert the
pin polarity, letting the generic function operate correctly without knowing
about board-specific wiring.

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
