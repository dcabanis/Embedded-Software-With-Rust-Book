# List of commands

## Build

cargo build --release

## Run with cargo runner

cargo run

## Run with debug backend (new terminal) + interactive GDB session

./run_openOCD.sh

What this script does:
- builds the debug ELF,
- opens a second terminal running the backend (`./run_openOCD.sh --backend`),
- waits for the GDB server on port `3333`,
- launches `gdb-multiarch`, sets `break main`, then `continue`.

Once `main` is hit, you are in an interactive GDB session.
Semihost output appears in the second terminal window.

## Quit

### Exit GDB/script flow

In GDB:

quit

### Quit QEMU console (when using `cargo run`)

Ctrl+a c

quit

## Clean

cargo clean
