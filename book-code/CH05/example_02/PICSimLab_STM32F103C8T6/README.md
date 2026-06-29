# example_02 — Type-safe MMIO via PAC in PICSimLab (STM32F103C8T6 simulation)

This example replaces raw `*mut u32` writes with the type-safe register API of the `stm32f1` Peripheral Access Crate (PAC). GPIO and RCC are configured through the PAC's `modify()` and `write()` closures, producing the same result as example_01: PC13 (active-low) toggled every second via SysTick. Running in PICSimLab lets you compare the PAC-based approach against raw MMIO without physical hardware. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

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
