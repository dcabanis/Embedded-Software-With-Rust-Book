# example_02 — Manual vector table and exception handlers in PICSimLab (STM32F103C8T6 simulation)

This example extends the bare-metal startup from example_01 by manually constructing the full Cortex-M exception vector table in a `startup` module. Handlers for NMI, HardFault, MemManage, BusFault, UsageFault, SVCall, and PendSV each print a diagnostic message via semihosting before halting. The main loop prints alternating messages on each SysTick tick, then triggers a deliberate undefined instruction to fire the HardFault handler. Running in PICSimLab lets you observe all of this without a physical board or debug probe. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

## Build

```sh
cargo build --release
```

## Run in PICSimLab

```sh
./run_picsimlab.sh
```

The script builds the binary, injects it into the PICSimLab workspace, and launches the simulator. The semihosting log shows ten iterations of `"Waiting for an interrupt..."` / `"Hello!"` / `"How do you do?"`, followed by `"Hard Fault handler"` after the deliberate undefined-instruction fault.

## Environment overrides

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```
