# example_05 — Rust inside a Zephyr application (stock Blue Pill)

This is the same section 8.5 demonstration as `../NUCLEO_STM32F103RBT6`, built for a **genuine, stock Blue Pill** (STM32F103C8T6) using Zephyr's `stm32_min_dev@blue` board — matching the chapter text exactly, board and all. See the NUCLEO port's README for the full explanation of the design; this one only covers what's specific to this board.

**Naming note:** this board is a real, unmodified Blue Pill, connected here via a separate external ST-Link V2 dongle. This is deliberately named `bluepill_stock_STM32F103C8T6` rather than plain `bluepill_STM32F103C8T6`, because that name is already used in examples 01-04 for a *different*, non-stock board that merely shares the same chip. Don't assume the two `bluepill_*` names across this chapter's examples refer to the same physical hardware.

## Files

Every file here — `CMakeLists.txt`, `prj.conf`, `src/main.c`, `rust/led_logic/{Cargo.toml,src/lib.rs}` — is byte-for-byte identical to the NUCLEO port. This is the whole point of the integration style the chapter demonstrates: neither the Rust component nor the C application code contains any board-specific detail. Only the board name passed to `west build` differs.

Confirmed on real hardware: two short flashes followed by a longer pause, repeating about every two seconds, and the same edit/rebuild/reflash loop (changing the Rust match arm to get three flashes) confirmed working here too.

## One-time environment setup

Same Zephyr workspace as the NUCLEO port — see `../NUCLEO_STM32F103RBT6/README.md` for the full setup instructions. Every command below assumes the same environment variables are exported:

```sh
source ~/zephyrproject/.venv/bin/activate
export ZEPHYR_SDK_INSTALL_DIR=~/zephyr-sdk-0.17.2
export ZEPHYR_BASE=~/zephyrproject/zephyr
```

## Build the Rust library and generate its header

```sh
cargo build --manifest-path rust/led_logic/Cargo.toml --release \
    --target thumbv7m-none-eabi --target-dir rust/led_logic/target

cd rust/led_logic
cbindgen --lang c --crate led_logic --output ../../include/led_logic.h
cd ../..
```

## Build and flash

```sh
west build -b stm32_min_dev@blue -d build .
```

**Flashing needs a manual reset, timed with the flash command** — see **Notes** below for why. The command itself:

```sh
probe-rs download --chip STM32F103C8Tx build/zephyr/zephyr.elf
```

## Notes

**This board's debug wiring has no reset line, unlike every other board in this chapter.** `probe-rs download --connect-under-reset` times out here ("target is not responding to the reset sequence"), meaning the external ST-Link V2 dongle's cable to this particular Blue Pill only carries SWDIO/SWCLK/GND/VCC, not NRST. This is a property of this specific physical wiring, not of the Blue Pill or the ST-Link V2 in general.

**Consequence: flashing after the first time needs a manually-timed reset.** Once Zephyr's idle thread is in `wfi()` (the same STM32F1 gotcha noted in the NUCLEO port's README, and in examples 01-04's), a plain `probe-rs download` fails with `SwdApWdataError`, and there's no reset line for `--connect-under-reset` to use to recover it here. Holding the reset button down while flashing does not help either — in our testing, that also left the debug port unresponsive, presumably because this chip's reset line holds the whole debug interface in reset too. What worked reliably: **press and release the reset button, then run `probe-rs download` immediately** — the very first tick after boot, before the core reaches `wfi()`, is enough of a window for `probe-rs` to attach. This needs a bit of practical timing and can take more than one attempt; a fresh `west build` (rather than a stale `build/`) plus this press-release-then-flash sequence is the reliable recipe we converged on.

**A board with a proper wired reset line (like the NUCLEO, or the non-stock board in examples 01-04) does not have this problem** — this is specific to this physical debug-probe-to-board wiring, not something inherent to using a stock Blue Pill with Zephyr in general. If your own ST-Link connection does carry a reset line, `--connect-under-reset` should work as it does elsewhere in this chapter.
