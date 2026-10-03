# example_04 — Traffic light with RTIC (NUCLEO-F103RB)

This is the NUCLEO-F103RB port of `../bluepill_STM32F103C8T6`, which implements section 8.4's RTIC 2 traffic-light demonstration. See that project's README for the full task/priority breakdown and the `stm32f1xx-hal` 0.11 API differences from the text (written against 0.10); this one only covers what differs for this board.

| Task                  | Kind            | Priority | Responsibility                                          |
| --------------------- | --------------- | -------- | -------------------------------------------------------- |
| `traffic_cycle`       | Software, async | 1        | Advance the normal sequence when no override is active   |
| `resume_normal`       | Software, async | 2        | Wait fifteen seconds, then release the override          |
| `emergency_override`  | Hardware, EXTI15_10 | 3    | Acknowledge the button and start an override              |

## Hardware

This board has only one onboard LED, not the three this example needs, so two of the three are external:

- **Red**: the board's own **LD2** LED (**PA5**, active-high) — no wiring required. Chosen for the onboard LED because it's the color the emergency override cares about: pressing B1 forces red and holds it, so the entire override behavior can be observed with no external wiring at all, using only onboard hardware.
- **Green**: **PB8**, external wiring required — an LED in series with a current-limiting resistor, from PB8 to ground (active-high).
- **Yellow**: **PB9**, same external wiring as green.
- **Emergency button**: the board's own **B1 (USER)** button on **PC13**, no wiring required — same as examples 01-03's NUCLEO ports. No internal pull is requested (B1 is already pulled up on the board), and the interrupt triggers on the falling edge.
- PC13's interrupt is bound to **EXTI15_10** (the shared vector for lines 10-15), not the bluepill port's dedicated `EXTI3`, since PC13 is EXTI line 13. The handler calls `check_interrupt()` before acting, matching examples 01-03's NUCLEO ports, even though nothing else currently shares that vector in this example.
- **Not yet hardware-verified on this specific board.** This port was written and compile/lint-verified while the bluepill_STM32F103C8T6 board was still connected.

Unlike the bluepill port, none of PA5/PB8/PB9/PC13 are JTAG pins, so this port needs no `AFIO`/JTAG-release call at all.

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
traffic light ready
```

LD2 (red), and the two external LEDs (green, yellow) if wired, then cycle indefinitely (~10 s / ~3 s / ~8 s). Press B1 at any point:

```
emergency override engaged
emergency override released
```

LD2 turns on immediately and holds for about fifteen seconds — observable even without wiring the external LEDs — before the cycle resumes wherever it would normally continue (at green).

## Run — Path B: OpenOCD (flash only)

`run_openocd.sh` flashes the firmware but does not decode semihosting output; use Path A to see the log.

```sh
cargo build
./run_openocd.sh
```

## Notes

**No host tests for this example** — same reasoning as the bluepill port: no pure-logic type worth isolating on the host.

**No `AFIO`/JTAG release needed** — PA5, PB8, PB9, and PC13 are all plain GPIO on this chip; none of them double as JTAG signals the way PB3 does on the bluepill port.

**`USART1`/`USART2` are borrowed only as RTIC dispatchers**, not used as UARTs — same as the bluepill port.

**Uses `memory.x`** (128 KB flash / 20 KB RAM), matching example_01's NUCLEO port — this example is `stm32f1xx-hal`/RTIC based, not Embassy, so it needs a hand-written linker script rather than `embassy-stm32`'s `memory-x` feature.
