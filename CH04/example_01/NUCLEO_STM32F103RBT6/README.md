# example_01  Bare-metal startup without cortex-m-rt on the NUCLEO-F103RB

This example implements the entire MCU startup from scratch, without the
`cortex-m-rt` crate. It manually places a reset vector in the `.vector_table`
linker section, initialises the `.data` and `.bss` segments in inline assembly,
then configures GPIO and SysTick by writing directly to memory-mapped I/O
addresses via raw pointers. LED LD2 on PA5 (active-high) toggles once per
SysTick tick, revealing what the runtime crate does automatically under the
hood.

## Build

```sh
cargo build
```

## Run  OpenOCD

```sh
./run_openOCD.sh
```
