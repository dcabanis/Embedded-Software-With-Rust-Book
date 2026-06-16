# example_03  SysTick exception handler using cortex-m-rt on the NUCLEO-F103RB

This example uses both the `cortex-m` and `cortex-m-rt` crates together to configure SysTick and register its exception handler. The firmware programs SysTick for a 1-second reload interval at 8 MHz. 

## Build

```sh
cargo build --release
```

## Run  Path A: probe-rs

```sh
cargo run --release
```

## Run  Path B: OpenOCD

```sh
./run_openOCD.sh
```
