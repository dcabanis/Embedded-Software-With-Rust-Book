# example_05 — Inline and global assembly LED blink in PICSimLab (STM32F103C8T6 simulation)

This example demonstrates three ways to embed assembly in embedded-Rust firmware: a `global_asm!()` counted delay loop, `core::arch::asm!()` inline blocks for PRIMASK manipulation and direct GPIOC BSRR toggling of PC13, and `cortex_m::asm::nop()` for short busy-wait delays. Running in PICSimLab lets you observe the assembly-driven LED toggle without physical hardware. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

## Build

```sh
cargo build --release
```

## Run in PICSimLab

```sh
./run_picsimlab.sh
```

The script builds the binary, injects it into the PICSimLab workspace, and launches the simulator. The onboard LED on PC13 toggles at a rate determined by the busy-wait loop counts.

> **Note:** The toggle rate in simulation may differ from physical hardware because PICSimLab does not simulate cycle-accurate timing for NOP loops.

## Environment overrides

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```
