# example_03 — Exception handling with cortex-m-rt in PICSimLab (STM32F103C8T6 simulation)

This example replaces the manual vector table from example_02 with the `#[exception]` macro provided by `cortex-m-rt`. The SysTick timer fires every second and flips an `AtomicBool` flag. The main loop polls the flag in WFI sleep and prints an alternating message via semihosting. Running in PICSimLab lets you observe the idiomatic `cortex-m-rt` approach to safe exception handler registration without a physical board or debug probe. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

## Build

```sh
cargo build --release
```

## Run in PICSimLab

```sh
./run_picsimlab.sh
```

The script builds the binary, injects it into the PICSimLab workspace, and launches the simulator. The semihosting log shows `"Waiting for an interrupt..."` followed by alternating `"You say goodbye..."` / `"And I say hello."` messages once per SysTick second.

## Environment overrides

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```
