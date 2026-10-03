# example_02 — Fan state machine rebuilt with Embassy (STM32F103C8T6)

This rebuilds example_01's fan controller from section 8.2 as an async Embassy task (section 8.3.2), replacing the interrupt-and-critical-section design with `.await` and `select()`. The fan's operating rules — `Fan`, `FanState`, `Event` in `src/lib.rs` — are unchanged from example_01 and still unit-testable on the host; only how the program *waits* for events changes.

`fan_task()` owns a mode button and a tachometer-pulse input. While the fan is `Off` or `Stalled`, it only waits for a button press. While running, it races a button press against `watch_for_stall()`, a helper that returns if 200 ms pass without a tachometer pulse.

## Hardware

This is the same non-stock STM32F103C8T6-based board used in `../../example_01/bluepill_STM32F103C8T6` — see that project's README for how its pin polarities were determined.

- **Button (mode select)**: **PB3**, internal pull-up, active-low. Fires on the falling edge — the opposite polarity from section 8.3.2's text, which wires its button to PA0 with a pull-down and waits for a *rising* edge.
- **Tachometer stand-in**: **PB4**, same electrical configuration as PB3. There is no real tachometer here, so a second button simulates its pulses: press it repeatedly, faster than every 200 ms, to simulate a running motor, or stop pressing it to simulate a stall. This was confirmed on real hardware — 23 presses in quick succession each reset the 200 ms window with no stall, and the stall fired ~200 ms after the last one.
- **Indicator**: **PB8**, active-low (LOW turns it on). Steady-on while `Stalled`, off otherwise — the same `apply_state()`/`set_fan_speed()` design as example_01, reporting the commanded speed (0-3) over semihosting since one LED can't distinguish `Low`/`Medium`/`High`.
- Both buttons are real mechanical switches and are not debounced here.

## Build

```sh
cargo build --release
```

## Run — Path A: probe-rs (recommended)

**A debugger must be attached to see the semihosting output.**

```sh
cargo run --release
```

Press PB3 once, then either press PB4 repeatedly (faster than every 200 ms) or leave it alone:

```
fan controller ready
fan speed = 0
fan speed = 1
fan speed = 0
```

`fan speed = 1` is `Off -> Low` from the PB3 press; the second `fan speed = 0` is the stall firing 200 ms after the last PB4 pulse (or immediately if PB4 is never pressed), with the fault LED turning on. Pressing PB3 again while stalled resets to `Off` and turns the LED back off.

## Run — Path B: OpenOCD (flash only)

`run_openocd.sh` flashes the firmware but does not decode semihosting output; use Path A to see the log.

```sh
cargo build
./run_openocd.sh
```

## Host tests

`Fan`'s transition table has no dependency on `core`, Embassy, or any embedded crate, so it can be compiled and tested for the host directly:

```sh
cargo test --lib --target x86_64-unknown-linux-gnu
```

(Substitute your own host triple if different — see `rustc -vV`.) This works here specifically because every Embassy/Cortex-M dependency lives in a `[target.'cfg(target_arch = "arm")'.dependencies]` table in `Cargo.toml` rather than plain `[dependencies]` — see the note below.

## Notes

**Why the Embassy dependencies are under a target-conditional table** `embassy-executor`'s `arch-cortex-m` feature contains unconditional inline ARM assembly (`asm!("sev")`), so a plain `[dependencies]` entry fails to even *compile* for a host target — `cargo test --lib --target x86_64-unknown-linux-gnu` would try to build it anyway, since Cargo resolves a package's full dependency graph regardless of which target you're building unless dependencies are scoped with `[target.'cfg(...)'. dependencies]`. Moving every embedded-only dependency into `cfg(target_arch = "arm")` keeps the host build graph free of them entirely.

**No `memory.x` in this project** `embassy-stm32`'s `memory-x` feature supplies the selected chip's memory layout itself; adding a hand-written `memory.x` here would conflict with it. This is a deliberate difference from example_01 (which uses `stm32f1xx-hal` and does need its own `memory.x`), not an oversight.

**PB3 and PB4 default to JTAG pin functions** on this chip (JTDO and JTRST), same as in example_01. Unlike `stm32f1xx-hal`, `embassy-stm32` has no `disable_jtag()`-style helper, so `main()` writes the AFIO register directly through `stm32-metapac` (the PAC crate `embassy-stm32` itself is built on, added here as a direct dependency for exactly this one register access). SWD debugging keeps working afterward; only the JTAG-only signals go away.

**No manual `DBGMCU_CR.DBG_SLEEP` handling needed** Unlike example_01, which sets this bit by hand, `embassy_stm32::Config` has an `enable_debug_during_sleep` field that defaults to `true` on chips with a `DBGMCU` peripheral. `embassy_stm32::init(Default::default())` already keeps the debug port alive through the executor's idle sleep.

**A ~200 ms manual "tachometer" is right at the edge of human reaction time.** Simulating pulses by hand only works because 23 presses over several seconds is achievable; a much shorter timeout would not be humanly demonstrable this way. A real tachometer would pulse far faster than any manual button press could.
