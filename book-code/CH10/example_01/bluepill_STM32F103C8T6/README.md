# example_01 — Fan state machine driven by button interrupts (STM32F103C8T6)

This is the firmware for a fan controller expressed as an explicit state machine (`FanState`, `Event`, `Fan`) rather than a set of Boolean flags. A button press advances `Off -> Low -> Medium -> High -> Off`; a stall event moves any running speed to `Stalled`, and a button press from `Stalled` resets to `Off` instead of resuming.

The state machine itself lives in `src/lib.rs`, separate from the firmware entry point, so its transition table can be unit-tested on the host without any embedded target or hardware, see **Host tests** below. `src/main.rs` wires that machine to two button interrupts on the STM32F103C8T6.

## Hardware

This board uses the same chip and debug probe as the Blue Pill but is not a stock Blue Pill — it has extra onboard buttons and LEDs a bare-bones Blue Pill does not, which this example takes advantage of. Pin roles below were confirmed by direct testing on this specific board with the `discover` tool (see **Notes**), not assumed from a reference schematic.

- **Button (cycle speeds)**: **PB3**, internal pull-up, active-low (idle high, press reads low). Fires `Event::ButtonPress` on the falling edge.
- **Button (stall)**: **PB4**, same electrical configuration as PB3. Stands in for a tachometer or other stall sensor — which chapter 8.2.2 deliberately leaves outside the example — by firing `Event::StallDetected` on the falling edge, through the same `FAN` mutex the cycle button uses.
- **Indicator**: **PB8**, active-low (LOW turns it on). Steady-on while `Stalled`, off otherwise. Since one LED cannot distinguish `Low`/`Medium`/`High`, `set_fan_speed()` reports the commanded speed (0-3) over semihosting instead of driving real motor-driver hardware.
- Both buttons are real mechanical switches and are not debounced here, so one physical press may register as more than one event.
- A fourth onboard LED (PA10) was excluded — it lit noticeably dimmer than the other three during testing (see **Notes**), suggesting a board-level difference (resistor value or similar) rather than anything configurable in firmware.

## Build

```sh
cargo build --release --bin app
```

## Run — Path A: probe-rs (recommended)

**A debugger must be attached to see the semihosting output** — semihosting works by trapping into the debugger on each print call, so there is nothing to view if nothing is attached.

```sh
cargo run --release --bin app
```

Press PB3 a few times and watch the log:

```
fan controller ready
fan speed = 0
fan speed = 1
fan speed = 2
fan speed = 3
fan speed = 0
```

Press PB4 at any point to force a stall — the state jumps straight to `Stalled` (logged as `fan speed = 0` alongside the fault LED turning on) regardless of the current speed. Pressing PB3 while stalled resets to `Off`; pressing PB4 again while already stalled changes nothing, as the transition table requires.

## Run — Path B: OpenOCD (flash only)

`run_openocd.sh` flashes the firmware but does not decode semihosting output; use Path A to see the log.

```sh
cargo build --bin app
./run_openocd.sh
```

## Host tests

`Fan`'s transition table has no dependency on `core` or any embedded crate, so it can be compiled and tested for the host directly, bypassing the `thumbv7m-none-eabi` target set in `.cargo/config.toml`:

```sh
cargo test --lib --target x86_64-unknown-linux-gnu
```

(Substitute your own host triple if different — see `rustc -vV`.) `--lib` is required: without it, Cargo also tries to build the firmware binaries for the host, which fails because they depend on `cortex-m-rt` and the STM32 PAC.

## Notes

**`src/bin/discover.rs`** A one-off hardware-discovery tool, not part of the book example, kept here in case this board (or a future one) needs its button/LED wiring re-verified. It drives PB8/PB9/PA9/PA10 through HIGH and LOW one at a time while continuously printing PB3/PB4/PB5/PA15's state, so polarity and pull requirements can be read directly off the log and the board rather than assumed. Run it with `cargo run --release --bin discover`.

**PB3, PB4, and PA15 default to JTAG pin functions** on this chip (JTDO, JTRST, JTDI), not plain GPIO, so `main()` calls `afio.mapr.disable_jtag(pa15, pb3, pb4)` before configuring them as inputs. This frees all three at once — PA15 is unused here, but the call requires all three pins together since it's a single AFIO register bit. SWD debugging (all `probe-rs`/`cargo run` in this project use SWD) keeps working afterward; only the additional JTAG-only signals go away.

**PB3/EXTI3 and PB4/EXTI4 are both dedicated, non-shared vectors** — unlike PC13 on the NUCLEO port, which shares `EXTI15_10` with seven other pins. That means neither interrupt handler here needs the `check_interrupt()` disambiguation the NUCLEO port uses; each vector only ever means one thing.

**Why semihosting instead of defmt/RTT** An earlier version of this example used `defmt`/RTT, matching the newer convention elsewhere in this book. `probe-rs` reads RTT output by polling target RAM over SWD, and on the NUCLEO-F103RB that polling could not reliably catch the core awake during the long stretches it spends asleep in `wfi()` between button presses — log lines sat in the RTT buffer and only appeared in one delayed burst once something forced a fresh read (for example, pressing the board's physical RESET button, which also made `probe-rs` report a spurious "Exception" — the core resetting unexpectedly from the debugger's point of view, not a firmware crash). Neither setting `DBGMCU_CR.DBG_SLEEP` (below) nor adding a periodic SysTick wake-up to shorten the sleep windows changed this behavior in testing. Semihosting sidesteps the problem entirely: each `hprintln!` call executes a `BKPT` instruction that halts the CPU synchronously and hands control straight to the debugger, so it does not depend on the host catching a narrow polling window.

**`DBGMCU_CR.DBG_SLEEP`** STM32F1 gates the debug clock domain during Sleep mode by default, so a probe attached over SWD can lose contact with the core the moment `wfi()` runs, breaking the SWD connection for any later reflash/reconnect until the board is reset. `main()` sets this bit before entering the loop so the debug clock — and with it, the probe connection used for flashing and semihosting — stays alive through Sleep.

**Critical section around the sleep boundary** The main loop reads the fan state, applies it to the LED/log, and calls `wfi()` all inside one `cortex_m::interrupt::free()` block. This prevents a button handler from changing the state after the outputs were applied but before the processor sleeps — see section 8.2.2 for the full explanation of why `wfi()` cannot be moved outside the critical section here.

**Relationship to the NUCLEO port** The NUCLEO-F103RB port only has one onboard button (B1/PC13), so it implements `ButtonPress` alone; there is no onboard second button or spare LED to test `StallDetected` against there. This board's extra I/O made it possible to validate the stall wiring on real hardware before documenting the equivalent external wiring (a second button and LED on header pins) for NUCLEO readers.
