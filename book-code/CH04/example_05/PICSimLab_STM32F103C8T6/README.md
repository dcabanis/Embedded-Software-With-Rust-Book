# example_05 — Sharing peripherals with Mutex and interior mutability in PICSimLab (STM32F103C8T6 simulation)

This example extends the Mutex pattern to a full peripheral by moving the `SYST` timer into a `static Mutex<RefCell<Option<SYST>>>` after configuration. Both the main loop and the `SysTick` exception handler borrow it inside `interrupt::free()` critical sections. The handler reads the current value register (CVR) and prints it via semihosting; the main loop detects 30-second intervals and prints elapsed time. Running in PICSimLab lets you observe safe peripheral sharing without a physical board or debug probe. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

## Build

```sh
cargo build --release
```

## Run in PICSimLab

```sh
./run_picsimlab.sh
```

The script builds the binary, injects it into the PICSimLab workspace, and launches the simulator. The semihosting log displays the SysTick CVR register value on every interrupt, and a `"30 seconds elapsed"` message every 30 ticks.

## Environment overrides

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```
