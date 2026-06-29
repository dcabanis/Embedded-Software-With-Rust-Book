# example_01 — Typical embedded Rust project layout in PICSimLab (STM32F103C8T6 simulation)

This is the chapter-3 starting-point project. The firmware takes STM32 device peripherals via `stm32f1xx_hal::pac::Peripherals::take()` and prints `"Hello, world!"` through the `hprintln!()` semihosting macro. Running in PICSimLab lets you see the semihosting output and observe the canonical embedded-Rust project structure without a physical board or debug probe. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

## Build

```sh
cargo build --release
```

## Run in PICSimLab

```sh
./run_picsimlab.sh
```

The script builds the binary, injects it into the PICSimLab workspace, and launches the simulator. The semihosting output `"Hello, world!"` appears in the log terminal window.

## Environment overrides

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```
