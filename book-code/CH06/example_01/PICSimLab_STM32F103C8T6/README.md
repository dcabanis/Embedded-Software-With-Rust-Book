# example_01 — Semihosting with hprintln!() in PICSimLab (STM32F103C8T6 simulation)

This example demonstrates semihosting output on the STM32F103. The firmware prints `"Hello from semihosting"` once via the `hprintln!()` macro from the `cortex-m-semihosting` crate, then spins in an infinite loop. Running in PICSimLab lets you see the semihosting output without a physical board or debug probe. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

## Build

```sh
cargo build --release
```

## Run in PICSimLab

```sh
./run_picsimlab.sh
```

The script builds the binary, injects it into the PICSimLab workspace, and launches the simulator. The semihosting output `"Hello from semihosting"` appears in the log terminal window.

## Environment overrides

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```
