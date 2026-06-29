# example_01 — Hello world with semihosting in PICSimLab (STM32F103C8T6 simulation)

This is the minimal "hello world" template for an STM32F103 project. The firmware prints `"Fly like a bird!"` once via the `hprintln!()` semihosting macro and then loops forever. Running in PICSimLab lets you see the semihosting output without a physical board or debug probe. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

## Build

```sh
cargo build --release
```

## Run in PICSimLab

```sh
./run_picsimlab.sh
```

The script builds the binary, injects it into the PICSimLab workspace, and launches the simulator. The semihosting output `"Fly like a bird!"` appears in the log terminal window.

## Environment overrides

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```
