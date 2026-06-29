# example_03 — HAL LED blink in PICSimLab (STM32F103C8T6 simulation)

This example demonstrates the top HAL layer of the embedded-Rust stack. The `stm32f1xx-hal` crate configures the RCC, takes ownership of GPIOC, and returns a typed push-pull output pin for PC13 (active-low). A HAL `Delay` provider built from SysTick abstracts the timing. Running in PICSimLab shows the contrast with the raw-pointer (example_01) and PAC (example_02) approaches without physical hardware. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

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
