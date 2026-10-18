# example_04 - Periodic acquisition and processing with Embassy (NUCLEO-F103RB)

This is the NUCLEO-F103RB port of `../bluepill_STM32F103C8T6`, which illustrates: a `Ticker`-driven sampling task on an Embassy interrupt executor feeding a bounded `Channel`, drained by a processing task on the thread-mode executor.

## Hardware

- **Measurement**: the STM32's internal temperature sensor on ADC1, exactly as on the Blue Pill. No wiring is needed.
- **LED**: the board's own **LD2** LED (PA5, active-high). 

## Build and run

```sh
cargo build --release
cargo run --release
```

**A debugger must be attached to see the semihosting output.** The NUCLEO's onboard ST-LINK provides it. 

```
current sampling demo ready
gap: 7 samples lost before #79
gap: 7 samples lost before #150
...
received 500, lost 49, last raw 1779
```
