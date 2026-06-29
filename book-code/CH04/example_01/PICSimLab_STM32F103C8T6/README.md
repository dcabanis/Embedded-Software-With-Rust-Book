# example_01 — Minimal linker script and startup in PICSimLab (STM32F103C8T6 simulation)

This example implements the entire MCU startup from scratch, without `cortex-m-rt`. It manually places a reset vector in the `.vector_table` linker section, initialises `.data` and `.bss` segments in inline code, then configures GPIO and SysTick by writing directly to memory-mapped I/O addresses via raw pointers. The onboard LED on PC13 (active-low) toggles once per SysTick tick. Running in PICSimLab makes the raw startup sequence and MMIO register writes visible without requiring physical hardware or a debug probe. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

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
