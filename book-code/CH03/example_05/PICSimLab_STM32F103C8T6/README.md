# example_05 — Platform-agnostic LED blink in PICSimLab (STM32F103C8T6 simulation)

This example demonstrates writing MCU-agnostic driver logic using the `embedded-hal` `OutputPin` trait, running in the PICSimLab Blue Pill simulator. A generic `blink_led()` function accepts any `OutputPin` and toggles it every 500 ms. Because the Blue Pill's onboard LED on PC13 is active-low, an `ActiveLow` new-type wrapper inverts the pin polarity so the generic function operates correctly without knowing about board-specific wiring. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

## Build

```sh
cargo build --release
```

## Run in PICSimLab

```sh
./run_picsimlab.sh
```

The script builds the binary, injects it into the PICSimLab workspace, and launches the simulator. The onboard LED on PC13 blinks at 500 ms intervals. 

## Environment overrides

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```
