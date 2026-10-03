# example_02 — Fan state machine rebuilt with Embassy (NUCLEO-F103RB)

This is the NUCLEO-F103RB port of `../bluepill_STM32F103C8T6`, which rebuilds example_01's fan controller from section 8.2 as an async Embassy task (section 8.3.2). See that project's README for the full explanation of the design; this one only covers what differs for this board.

`Fan`/`FanState`/`Event` in `src/lib.rs` are byte-for-byte identical to every other port of this state machine and still unit-testable on the host.

## Hardware

- **Button (mode select)**: the NUCLEO board's own **B1 (USER)** button on **PC13**, no wiring required — same as example_01's NUCLEO port. B1 is pulled up on the board and pulls PC13 to ground when pressed, so the firmware requests no internal pull and triggers on the falling edge.
- **Tachometer stand-in**: **PB4**, external wiring required — a momentary button between PB4 and ground, using the internal pull-up (idle high, press low, falling edge). This is the *same pin and electrical configuration* as the bluepill_STM32F103C8T6 port's tachometer stand-in, kept identical across both boards on request. PB4 defaults to a JTAG signal (JTRST) on this chip, exactly as it does on the Blue Pill, so it needs the same AFIO release — see **Notes**. As on the bluepill port, press it repeatedly (faster than every 200 ms) to simulate a running motor, or stop pressing it to simulate a stall.
- **Indicator**: the board's own **LD2** LED (PA5, active-high) — same as example_01's NUCLEO port. Opposite polarity from the bluepill port's PB8, so `set_outputs()` here uses `set_high()`/`set_low()` the other way around.
- **Not yet hardware-verified on this specific board.** This port was written and compile/lint/host-test-verified while a different board (bluepill_STM32F103C8T6) was connected; PB4 in particular is assumed free on this board's headers (it carries no special role on the NUCLEO-F103RB board itself, only the chip-level JTRST function shared with every STM32F103), but that assumption has not yet been confirmed by actually wiring a button to it and flashing this firmware.

## Build

```sh
cargo build --release
```

## Run — Path A: probe-rs (recommended)

**A debugger must be attached to see the semihosting output.**

```sh
cargo run --release
```

Press B1 once, then either press the PB4 button repeatedly (faster than every 200 ms) or leave it alone:

```
fan controller ready
fan speed = 0
fan speed = 1
fan speed = 0
```

`fan speed = 1` is `Off -> Low` from the B1 press; the second `fan speed = 0` is the stall firing 200 ms after the last PB4 pulse (or immediately if PB4 is never pressed), with LD2 turning on. Pressing B1 again while stalled resets to `Off` and turns LD2 back off.

## Run — Path B: OpenOCD (flash only)

`run_openocd.sh` flashes the firmware but does not decode semihosting output; use Path A to see the log.

```sh
cargo build
./run_openocd.sh
```

## Host tests

```sh
cargo test --lib --target x86_64-unknown-linux-gnu
```

(Substitute your own host triple if different — see `rustc -vV`.) This works because every Embassy/Cortex-M dependency lives in a `[target.'cfg(target_arch = "arm")'.dependencies]` table in `Cargo.toml` rather than plain `[dependencies]` — see the bluepill port's README for why that split is necessary.

## Notes

**PB4 defaults to a JTAG pin function** on this chip (JTRST), same as on the bluepill_STM32F103C8T6 port. `main()` releases it (and PA15/PB3 as an unavoidable side effect of the same AFIO register bit, even though this example doesn't use them) via a direct `stm32-metapac` write, since `embassy-stm32` has no higher-level API for this:

```rust
stm32_metapac::AFIO
    .mapr()
    .modify(|w| w.set_swj_cfg(stm32_metapac::afio::vals::SwjCfg::JTAG_DISABLE));
```

SWD debugging (what probe-rs uses here) keeps working afterward.

**No `memory.x` file** — `embassy-stm32`'s `memory-x` feature supplies the 128 KB/20 KB layout for the `stm32f103rb` chip selection itself.

**No manual `DBGMCU_CR.DBG_SLEEP` handling needed** — `embassy_stm32::Config`'s `enable_debug_during_sleep` defaults to `true`, unlike example_01's NUCLEO port (which sets the equivalent bit by hand using `stm32f1xx-hal`).
