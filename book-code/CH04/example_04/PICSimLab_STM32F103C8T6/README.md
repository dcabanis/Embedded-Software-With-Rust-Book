# example_04 — Safe shared access with Mutex in PICSimLab (STM32F103C8T6 simulation)

This example demonstrates how to safely share a variable between the main execution context and an interrupt handler using `cortex_m::interrupt::Mutex<Cell<u32>>`. A monotonic seconds counter is stored in a `static` and incremented by the `SysTick` handler every second. All accesses go through `interrupt::free()` critical sections, preventing data races on Cortex-M. Running in PICSimLab lets you observe both the counter and the shared value printed on each tick without a physical board or debug probe. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

## Build

```sh
cargo build --release
```

## Run in PICSimLab

```sh
./run_picsimlab.sh
```

The script builds the binary, injects it into the PICSimLab workspace, and launches the simulator. The semihosting log displays a monotonic seconds counter and the current shared value on every SysTick tick.

## Environment overrides

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```
