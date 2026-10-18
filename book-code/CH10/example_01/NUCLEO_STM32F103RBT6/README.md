# example_01 - Fan state machine driven by a button interrupt (NUCLEO-F103RB)

This is the firmware for a fan controller expressed as an explicit state machine (`FanState`, `Event`, `Fan`) rather than a set of Boolean flags. A button press advances `Off -> Low -> Medium -> High -> Off`; a stall event moves any running speed to `Stalled`, and a button press from `Stalled` resets to `Off` instead of resuming.

The state machine itself lives in `src/lib.rs`, separate from the firmware entry point, so its transition table can be unit-tested on the host without any embedded target or hardware, see **Host tests** below. `src/main.rs` wires that machine to two button interrupts on the STM32F103RB.

## Hardware

- **Button (cycle speeds)**: the NUCLEO board's own **B1 (USER)** button on **PC13**, no wiring required. B1 is pulled up on the board and pulls PC13 to ground when pressed, so the firmware leaves PC13 floating and triggers on the falling edge.
- **Button (stall)**: a momentary button wired between **PA0** and 3.3 V — external wiring required, since this board has no onboard second button. The firmware configures PA0 as an internal pull-down input, so PA0 reads low when released and high when pressed (rising edge = press); this differs from the Blue Pill port, which uses a pull-up button to ground on PB4 (section 10.2.2 shows the Blue Pill wiring). It stands in for a tachometer or other stall sensor, submitting `Event::StallDetected` through the same `FAN` mutex the cycle button uses.
- **Indicator**: the board's own **LD2** LED (PA5, active-high) is used as a stand-in for `set_fault_indicator()`. It is steady-on while `Stalled` and off otherwise. Since one LED cannot distinguish `Low`/`Medium`/`High`, `set_fan_speed()` reports the commanded speed (0-3) over semihosting instead of driving real motor-driver hardware.
- Both buttons are real mechanical switches and are not debounced here, so one physical press may register as more than one event.

## Build

```sh
cargo build --release
```

## Run - Path A: probe-rs (recommended)

**A debugger must be attached to see the semihosting output** — semihosting works by trapping into the debugger on each print call, so there is nothing to view if nothing is attached.

```sh
cargo run --release
```

Press B1 a few times and watch the log:

```
fan controller ready
fan speed = 0
fan speed = 1
fan speed = 2
fan speed = 3
fan speed = 0
```

Press the PA0 stall button at any point to force a stall, the state jumps straight to `Stalled` (logged as `fan speed = 0` alongside the fault LED turning on) regardless of the current speed. Pressing B1 while stalled resets to `Off`; pressing the stall button again while already stalled changes nothing, as the transition table requires.

## Run - Path B: OpenOCD (flash only)

`run_openocd.sh` flashes the firmware but does not decode semihosting output; use Path A to see the log.

```sh
cargo build
./run_openocd.sh
```

## Host tests

`Fan`'s transition table has no dependency on `core` or any embedded crate, so it can be compiled and tested for the host directly, bypassing the `thumbv7m-none-eabi` target set in `.cargo/config.toml`:

```sh
cargo test --lib --target x86_64-unknown-linux-gnu
```

(Substitute your own host triple if different — see `rustc -vV`.) `--lib` is required: without it, Cargo also tries to build the `main.rs` binary for the host, which fails because it depends on `cortex-m-rt` and the STM32 PAC.

## Notes

**`DBGMCU_CR.DBG_SLEEP`** STM32F1 gates the debug clock domain during Sleep mode by default, so a probe attached over SWD can lose contact with the core the moment `wfi()` runs, breaking the SWD connection for any later reflash/reconnect until the board is reset. `main()` sets this bit before entering the loop so the debug clock and with it, the probe connection used for flashing and semihosting stays alive through Sleep.

**Critical section around the sleep boundary** The main loop reads the fan state, applies it to the LED/log, and calls `wfi()` all inside one `cortex_m::interrupt::free()` block. This prevents the button handler from changing the state after the outputs were applied but before the processor sleeps.
