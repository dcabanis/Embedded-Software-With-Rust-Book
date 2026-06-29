# example_04  LED blink in PICSimLab (STM32F103C8T6 simulation)

This example targets the PICSimLab simulator rather than physical hardware. The firmware configures the STM32F103C8T6 system clock to 72 MHz via the HSE PLL, initialises GPIO PC13 as a push-pull output, and toggles the LED every 500 ms using a SysTick-based delay. It is intended to be run inside PICSimLab's Blue Pill board workspace, demonstrating how to develop and test firmware without a physical device.

## Build

```sh
cargo build --release
```

or for a debug build:

```sh
cargo build
```

## Run  PICSimLab

The `run_picsimlab.sh` script builds the release binary, loads the workspace,
launches PICSimLab, and opens a log terminal when one is available.

```sh
./run_picsimlab.sh
```

Show script help:

```sh
./run_picsimlab.sh --help
```

## Environment overrides

Override the PICSimLab executable:

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
```

Override the terminal used for the log window:

```sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```

If no supported terminal is found, PICSimLab still starts but the log window is
skipped.
