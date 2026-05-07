# example_03  SysTick exception and atomic flag with cortex-m-rt on the NUCLEO-F103RB

This example replaces the manual vector table from example_02 with the `#[exception]` macro provided by `cortex-m-rt`. The SysTick timer fires every second and flips an `AtomicBool` flag. The main loop polls the flag in WFI sleep and prints an alternating message via semihosting, demonstrating the idiomatic `cortex-m-rt`  approach to safe exception handler registration. **A debugger must be attached to see the semihosting output.**

## Build

```sh
cargo build
```

## Run  Path A: probe-rs

```sh
cargo run
```

## Run  Path B: OpenOCD

```sh
./run_openOCD.sh
```
