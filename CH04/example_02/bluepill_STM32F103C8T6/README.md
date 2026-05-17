# example_02  Manual Cortex-M vector table and exception handlers on the Blue Pill

This example extends the bare-metal startup from example_01 by manually
constructing the full Cortex-M exception vector table in a `startup` module.
Handlers for NMI, HardFault, MemManage, BusFault, UsageFault, SVCall, and
PendSV are provided; each prints a diagnostic message via semihosting before
halting. The example shows what `cortex-m-rt` generates automatically, making
the underlying mechanism visible. **A debugger must be attached to see the
semihosting output.**

## Build

```sh
cargo build
```

## Run  Path B: OpenOCD

```sh
./run_openOCD.sh
```
