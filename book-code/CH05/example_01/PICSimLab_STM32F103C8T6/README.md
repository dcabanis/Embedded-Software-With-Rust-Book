# example_01 — Raw-pointer MMIO LED blink in PICSimLab (STM32F103C8T6 simulation)

This is the chapter-5 baseline: a bare-metal LED blinker written entirely with raw pointer MMIO, using no HAL or PAC. The firmware initialises data and BSS sections, enables GPIOC via RCC, and configures PC13 as a push-pull output. SysTick is polled (not interrupt-driven) to produce a 1-second toggle period. Running in PICSimLab lets you observe the raw MMIO register writes driving the LED without physical hardware. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

## Build

```sh
cargo build --release
```

## Run in PICSimLab

```sh
./run_picsimlab.sh
```

The script builds the binary, injects it into the PICSimLab workspace, and launches the simulator. The onboard LED on PC13 toggles every second.

## Environment overrides

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```
