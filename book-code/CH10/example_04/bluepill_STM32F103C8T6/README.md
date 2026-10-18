# example_04 - Periodic acquisition and processing with Embassy (STM32F103C8T6)

This is a separate demo from examples 01/02, building on example_03's two-executor arrangement to illustrate: a sampling task reads a measurement every 10 ms with a `Ticker` and queues it in a bounded `Channel`, while a second task processes the queued readings.

Two tasks run on two executors:

- `sample_current()` runs on an **interrupt executor** (bound to the otherwise unused USART2 vector at NVIC priority P6). Every 10 ms it reads the ADC, stamps the reading with a sequence number, and calls `try_send()` on a `Channel<_, Sample, 8>`. If the queue is full, the new reading is discarded and sampling carries on.
- `process_samples()` runs on the **thread-mode executor**. It receives each `Sample`, compares its sequence number with the one it expected, reports any gap, then calls `process_current()`.

`process_current()` stands in for the real processing step. It normally busy-waits about 1 ms. Every 64th call it busy-waits 150 ms, which is more than the 80 ms of readings the queue can hold (8 entries × 10 ms). The interrupt executor preempts that long calculation, so sampling continues, the queue fills, and the consumer then sees a gap of about seven missing sequence numbers.

## Hardware

Same board as the other Blue Pill examples. No wiring is needed.

- **Measurement**: the STM32's **internal temperature sensor** (ADC1 channel 16) stands in for a motor-current sensor, with a 239.5-cycle sample time. The raw 12-bit value is not converted to any unit.
- **LED**: **PB8**, active-low ("high means off"). It turns on when the consumer detects a gap and turns off again at the next summary line.

## Build

```sh
cargo build --release
```

## Run — Path A: probe-rs (recommended)

**A debugger must be attached to see the semihosting output.**

```sh
cargo run --release
```

Output captured on a Blue Pill (the exact sequence numbers will differ from run to run):

```
current sampling demo ready
gap: 7 samples lost before #79
gap: 7 samples lost before #150
gap: 7 samples lost before #221
gap: 7 samples lost before #292
gap: 7 samples lost before #363
gap: 1 samples lost before #375
gap: 7 samples lost before #435
gap: 7 samples lost before #506
received 500, lost 50, last raw 1760
```

The long calculation happens every 64th processed reading and costs 7 lost readings, so the main gaps are 71 sequence numbers apart (79, 150, 221, ...). The summary line appears every 500 received samples. The raw value stays near 1760 because it is the on-chip temperature sensor at room temperature (about 1.42 V on the 12-bit scale).

Depending on how the debugger is attached, you may also see occasional smaller gaps of 1 to 4 readings, and rarely a larger one. These come from the logging itself: `hprintln!` halts the core while the debugger services each print, which delays the sampler. They are not part of the design, and a plain `cargo run --release` in a terminal often shows none.

## Experiment: sampler on the same executor

`SAMPLER_PREEMPTS` at the top of `src/main.rs` selects where `sample_current()` runs. Set it to `false` to put both tasks on the thread-mode executor. During the 150 ms calculation the sampler can no longer be polled, so nothing is discarded and the gap lines disappear (on hardware, only `received N, lost 0` lines are printed), but the readings are taken late: `Ticker` catches up afterwards, producing closely spaced reads for ticks that were already overdue. This is the behaviour the section describes for a long calculation sharing an executor with acquisition.

## Run - Path B: OpenOCD (flash only)

`run_openocd.sh` flashes the firmware but does not decode semihosting output; use Path A to see the log.

```sh
cargo build
./run_openocd.sh
```

## Notes

**`USART2` is borrowed only for its NVIC vector**, as in example_03. Don't add a UART driver to this project without picking a different vector.

**`ADC1_2` is bound with `bind_interrupts!`.** On the STM32F1, `Adc::read()` waits for the end-of-conversion interrupt, and this handler wakes the sampling task. The ADC interrupt keeps its default (highest) NVIC priority, above the executor's P6, so a conversion can complete while the executor is mid-poll.

**Semihosting stops the core.** `hprintln!` halts the processor while the debugger services the call. Output is therefore limited to gap reports and one summary every 500 samples, and it is printed from the thread-mode task, not the sampler. Sampling is delayed while a print is in progress, which is a limitation of this logging method, not of the Embassy design.

**No `memory.x` file**,`embassy-stm32`'s `memory-x` feature supplies it.
