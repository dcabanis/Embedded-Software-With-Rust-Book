# example_03 — Preempting a busy task with an interrupt executor (NUCLEO-F103RB)

This is the NUCLEO-F103RB port of `../bluepill_STM32F103C8T6`, which demonstrates section 8.3.3: a higher-priority Embassy executor interrupting a lower-priority task that is busy computing and hasn't reached an `.await` yet. See that project's README for the full explanation; this one only covers what differs for this board.

There is no `Fan` state machine or host-testable logic in this example — just a blinking LED, a stop button, and a shared flag.

## Hardware

- **LED**: the board's own **LD2** LED (PA5, active-high) — same as examples 01/02's NUCLEO ports. Opposite polarity from the bluepill port's PB8, so `set_high()`/`set_low()` are swapped relative to that version.
- **Stop button**: the board's own **B1 (USER)** button on **PC13**, no wiring required — same as examples 01/02's NUCLEO ports. No internal pull is requested, matching the reasoning in example_01's README (B1 is already pulled up on the board).
- Neither PC13 nor PA5 is a JTAG pin, so — unlike the bluepill port, which needs to release PB3 from its JTAG function — this port needs no AFIO register write at all.
- **Not yet hardware-verified on this specific board.** This port was written and compile/lint-verified while the bluepill_STM32F103C8T6 board was still connected.

## Build

```sh
cargo build --release
```

## Run — Path A: probe-rs (recommended)

**A debugger must be attached to see the semihosting output.**

```sh
cargo run --release
```

Expected sequence:

```
heartbeat demo ready
```

LD2 then blinks on 500 ms / off 500 ms indefinitely. Press B1 at any point — including mid-blink, while the busy loop is still running — and:

```
stop button pressed, heartbeat halted
heartbeat task ending
```

LD2 turns off immediately on the press and stays off. Only a board reset restarts the demo.

## Run — Path B: OpenOCD (flash only)

`run_openocd.sh` flashes the firmware but does not decode semihosting output; use Path A to see the log.

```sh
cargo build
./run_openocd.sh
```

## Notes

**No host tests for this example** — same reasoning as the bluepill port: there's no pure-logic state machine here, just a `bool` flag, and the behavior being demonstrated is a hardware/timing property.

**`USART2` is borrowed only for its NVIC vector**, not used as a UART — same as the bluepill port. Don't add a UART driver to this project without picking a different vector.

**No `memory.x` file** — `embassy-stm32`'s `memory-x` feature supplies the 128 KB/20 KB layout for the `stm32f103rb` chip selection.
