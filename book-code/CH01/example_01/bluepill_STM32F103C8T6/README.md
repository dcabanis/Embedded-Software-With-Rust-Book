# example_01  SysTick singleton on the Blue Pill

This example demonstrates the Cortex-M singleton pattern for peripheral access.
The firmware acquires ownership of `CorePeripherals` via
`cortex_m::Peripherals::take()`, configures the SysTick timer for a 1-second
periodic interrupt at 8 MHz, and parks the CPU in WFI in the main loop. A
`#[exception]`-annotated `SysTick` handler fires on each tick, illustrating how
`cortex-m-rt` places exception handlers in the correct vector table slots.

## Build

```sh
cargo build
```

or for a release build:

```sh
cargo build --release
```

## Run — Path A: probe-rs

```sh
cargo run
```

## Run — Path B: OpenOCD

```sh
cargo build --release
./run_openOCD.sh
```

> Note: I some circumstances when an earlier execution wasn't terminated cleanly, the device might **refuse to connect** to a debug session. You may get a message such as this one:
> ``` 
> Warning: st-info cannot read chipid (got '0x0000').
> Try pressing/holding RESET while running this script, then release after 1-2 seconds.
> Open On-Chip Debugger 0.12.0
> Licensed under GNU GPL v2
> For bug reports, read
> 	http://openocd.org/doc/doxygen/bugs.html
> Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
> Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
> Warn : Transport "hla_swd" was already selected
> hla_swd
> adapter speed: 50 kHz
> 
> none separate
> 
> Info : clock speed 50 kHz
> Info : STLINK V2J46S7 (API v2) VID:PID 0483:3748
> Info : Target voltage: 3.196328
> Error: init mode failed (unable to connect to the target)
> 
> ```
>
> There are a couple of things you can try to get out of this situation:
>
> 1. Press and hold the reset button whilst issuing the `./run_openOCD.sh` command. Once the programming of the device has taken place, you can release the reset button.
> 2. When things are getting more difficult you can use this command: `st-flash --connect-under-reset erase` whilst holding the reset button. Once the erasing had competed you can release the button and try once again either `./run_openOCD.sh` or `cargo run`.
