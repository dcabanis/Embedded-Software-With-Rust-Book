# example_03  Semihosting in QEMU (STM32F100 simulation)

This example runs on QEMU instead of physical hardware, targeting the STM32F100
(Cortex-M3) chip model. The firmware prints `"Fly like a bird!"` via
semihosting, then enters an infinite loop. It demonstrates how to validate
embedded Rust code on a host machine without a development board, using QEMU's
ARM system emulation as the execution environment.

## Build

```sh
cargo build
```

or for a release build:

```sh
cargo build --release
```

## Run  Path A: QEMU via cargo runner

Semihosting output appears directly in the terminal.

```sh
cargo run
```

Expected output:

```
Fly like a bird!
```

To quit the QEMU console: press `Ctrl+a c` to open the QEMU monitor, then type
`quit`.

## Run  Path B: QEMU backend + interactive GDB session

`run_GDB.sh` builds the debug ELF, opens a second terminal running the QEMU
GDB server, waits for the server on port `3333`, then launches `gdb-multiarch`
with a breakpoint at `main`.

```sh
./run_GDB.sh
```

Semihosting output appears in the backend terminal. To exit the GDB session,
type `quit` at the GDB prompt.
