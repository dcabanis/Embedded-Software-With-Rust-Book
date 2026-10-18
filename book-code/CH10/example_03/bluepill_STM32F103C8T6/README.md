# example_03 — Preempting a busy task with an interrupt executor (STM32F103C8T6)

This is a separate demo from examples 01/02, illustrating how a higher-priority Embassy executor can interrupt a lower-priority task that is busy computing and hasn't reached an `.await` yet. There just a blinking LED and a stop button.

Two executors run concurrently:

- A **thread-mode executor** runs `heartbeat_task()`: turn the LED on, spend 500 ms in a busy loop (deliberately *not* yielding, to simulate an occupied task), turn the LED off, then yield for 500 ms via an async delay.
- An **interrupt executor**, bound to the (otherwise unused) USART2 interrupt vector at a higher NVIC priority, runs `stop_task()`: wait for the stop button, then set a shared flag and turn the LED off immediately, even if the heartbeat task is still in the middle of its 500 ms busy loop.

Both tasks touch the same LED and flag, protected by a `Mutex` so a check and a write always happen together.

## Hardware

Same board as `../../example_01/bluepill_STM32F103C8T6` and `../../example_02/bluepill_STM32F103C8T6`; this example reuses their pins:

- **LED**: **PB8**, active-low ("high means off").
- **Stop button**: **PB3**, internal pull-up, active-low (pressed reads low). PB3 defaults to a JTAG signal on this chip (JTDO) and is released the same way as in examples 01/02.

The LED blinks steadily, and pressing PB3 mid-blink turns it off immediately and permanently (until reset), logging `stop button pressed, heartbeat halted` followed by `heartbeat task ending`.

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
heartbeat demo ready
```

The LED then blinks on 500 ms / off 500 ms indefinitely. Press PB3 at any point — including mid-blink, while the busy loop is still running — and:

```
stop button pressed, heartbeat halted
heartbeat task ending
```

The LED turns off immediately on the press and stays off. Only a board reset restarts the demo.

## Run - Path B: OpenOCD (flash only)

`run_openocd.sh` flashes the firmware but does not decode semihosting output; use Path A to see the log.

```sh
cargo build
./run_openocd.sh
```

## Notes

**`USART2` is borrowed only for its NVIC vector** This example never initializes a UART peripheral. `interrupt::USART2` is repurposed purely as a spare, otherwise-unused interrupt line for the high-priority executor to run on. Don't add a UART driver to this project without picking a different vector.

**PB3 defaults to a JTAG pin function** on this chip (JTDO), same as in examples 01/02. `main()` releases it (and PA15/PB4, JTDI/JTRST, as an unavoidable side effect of the same AFIO register bit) via a direct `stm32-metapac` write, since `embassy-stm32` has no higher-level API for this. SWD debugging (what probe-rs uses here) keeps working afterward.

**No `memory.x` file** — `embassy-stm32`'s `memory-x` feature supplies it.

**Why the busy loop doesn't block the stop button** `heartbeat_task()`'s 500 ms loop only occupies the *low-priority* executor. The interrupt executor runs independently at a higher NVIC priority, so the CPU can still take the USART2 interrupt and run `stop_task()` to completion in the middle of that loop. Interrupts stay enabled throughout the busy loop; nothing here masks them.
