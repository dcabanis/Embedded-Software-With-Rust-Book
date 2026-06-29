# example_02 — UART logging in PICSimLab (STM32F103C8T6 simulation)

This example demonstrates standalone UART output. The firmware configures USART1 on PA9 (TX) and PA10 (RX) at 115 200 baud, writes `"UART log message"` once via `writeln!()`, then spins in an infinite loop. The PICSimLab workspace includes an **IO Virtual Term** component wired to PA9/PA10 at 115 200 baud so the log message appears without any physical hardware or USB-to-UART adapter. The source code is identical to the `bluepill_STM32F103C8T6` variant; only the deployment method differs.

## Build

```sh
cargo build --release
```

## Run in PICSimLab

```sh
./run_picsimlab.sh
```

The script builds the binary, injects it into the PICSimLab workspace, and launches the simulator. The **IO Virtual Term** window displays `"UART log message"`.

> **Baud rate:** The virtual terminal is pre-configured at 115 200 baud to match the firmware.

## Environment overrides

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```
