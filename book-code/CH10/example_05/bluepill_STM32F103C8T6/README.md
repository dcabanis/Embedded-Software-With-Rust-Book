# example_05 - Traffic light with RTIC (STM32F103C8T6)

This is the RTIC 2 traffic-light demonstration: a normal task cycles three LEDs through green, yellow, and red, while an emergency button preempts that sequence and holds red for fifteen seconds. It illustrates RTIC's task priorities and shared-resource locking.

Three tasks:

| Task                  | Kind            | Priority | Responsibility                                          |
| --------------------- | --------------- | -------- | -------------------------------------------------------- |
| `traffic_cycle`       | Software, async | 1        | Advance the normal sequence when no override is active   |
| `resume_normal`       | Software, async | 2        | Wait fifteen seconds, then release the override          |
| `emergency_override`  | Hardware, EXTI3 | 3        | Acknowledge the button and start an override              |

## Hardware

- **Green**: **PB8**, active-low.
- **Yellow**: **PB9**, active-low.
- **Red**: **PA9**, active-low. 
- **Emergency button**: **PB3**, internal pull-up, active-low, triggers on the falling edge. PB3 defaults to a JTAG signal (JTDO) on this chip and is released the same way as in examples 01-03.
- The button's interrupt is bound to **EXTI3** (`binds = EXTI3` in the `emergency_override` task), PB3's dedicated vector.

The three LEDs cycle in sequence (~10 s / ~3 s / ~8 s), and pressing PB3 at any point immediately forces the red LED on and holds it for ~15 seconds before the cycle resumes, logging `emergency override engaged` then `emergency override released`.

## Build

```sh
cargo build --release
```

## Run - Path A: probe-rs (recommended)

**A debugger must be attached to see the semihosting output.**

```sh
cargo run --release
```

Expected sequence:

```
traffic light ready
```

The three LEDs then cycle indefinitely. Press PB3 at any point:

```
emergency override engaged
emergency override released
```

Further presses while the fifteen-second hold is already active are acknowledged (the button's interrupt still fires and clears) but produce no new log line and no controller change, `resume_normal::spawn()` returns an error while that task is still active, so `emergency_override` skips the controller update entirely.

## Run - Path B: OpenOCD (flash only)

`run_openocd.sh` flashes the firmware but does not decode semihosting output; use Path A to see the log.

```sh
cargo build
./run_openocd.sh
```

## Notes

**PB3 defaults to a JTAG pin function** on this chip (JTDO), same as examples 01-03. `init()` releases it (and PA15/PB4, JTDI/JTRST, as an unavoidable side effect of the same AFIO register bit) via `afio.mapr.disable_jtag(...)`, exactly as in example_01.

**`USART1`/`USART2` are borrowed only as RTIC dispatchers**, not used as UARTs. They schedule the two software tasks (`traffic_cycle` at priority 1, `resume_normal` at priority 2) rather than handling any UART traffic. Don't add a UART driver to this project without picking different vectors.

**Uses `memory.x`, not `embassy-stm32`'s `memory-x` feature**, this example is `stm32f1xx-hal`/RTIC based, not Embassy (examples 02/03 use Embassy and get their memory layout from the `memory-x` feature instead), so it needs the same hand-written `memory.x` as example_01.
