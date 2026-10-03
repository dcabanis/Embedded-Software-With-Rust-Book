# example_05 — Rust inside a Zephyr application (NUCLEO-F103RB)

This is the section 8.5 demonstration: Zephyr owns the application (kernel, threads, GPIO driver), and calls into a small `no_std` Rust static library for one calculation. Every 200 ms, `main.c` asks Rust's `led_pattern(step)` whether the LED should be on for the current step, and applies the result through Zephyr's GPIO API. Rust never touches hardware directly.

Unlike examples 01-04, this is not a plain Cargo project — it's a Zephyr application (built with `west`/CMake) that links a separately-built Rust `staticlib`.

## Board

This targets the **NUCLEO-F103RB** with Zephyr's official `nucleo_f103rb` board support, rather than the non-stock board used for examples 01-04's `bluepill_STM32F103C8T6` variant. Zephyr's generic `stm32_min_dev@blue` board definition (used in the chapter text) assumes a stock Blue Pill's `led0` alias on PC13 — untested on our non-stock board, which has no official Zephyr board support of its own. `nucleo_f103rb`'s devicetree already defines `led0` correctly as LD2/PA5, so no devicetree overlay or board-specific code was needed. The C and Rust source here are otherwise identical to the chapter text — the board choice only changes the `west build -b <board>` argument.

A genuine, stock Blue Pill was connected later and given its own port at `../bluepill_stock_STM32F103C8T6`, using `stm32_min_dev@blue` — a complete, board-and-all match with the chapter text. See that project's README, particularly for a reset-wiring quirk specific to that physical setup that doesn't affect this NUCLEO port.

Confirmed on real hardware: LD2 shows two short flashes followed by a longer pause, repeating about every two seconds. Changing the Rust match arm from `0 | 2` to `0 | 2 | 4` (rebuild the Rust library, then rebuild and reflash) produces three flashes instead of two, confirming the whole edit/rebuild/reflash loop.

## One-time environment setup

This needs a full Zephyr workspace, which does not live inside this repo (it's a large, separate toolchain, analogous to `~/.cargo`). If you don't already have one:

```sh
python3 -m venv ~/zephyrproject/.venv
source ~/zephyrproject/.venv/bin/activate
pip install west
cd ~/zephyrproject
west init -m https://github.com/zephyrproject-rtos/zephyr --mr v4.2.0 .
west update
pip install -r zephyr/scripts/requirements.txt

# Zephyr SDK 0.17.2 (no sudo required for any of this):
mkdir ~/zephyr-sdk-install && cd ~/zephyr-sdk-install
wget https://github.com/zephyrproject-rtos/sdk-ng/releases/download/v0.17.2/zephyr-sdk-0.17.2_linux-x86_64_minimal.tar.xz
tar xf zephyr-sdk-0.17.2_linux-x86_64_minimal.tar.xz -C ~/
~/zephyr-sdk-0.17.2/setup.sh -t arm-zephyr-eabi -h -c
```

Also install Zephyr's host package dependencies (this step does need `sudo`): `git cmake ninja-build gperf ccache dfu-util device-tree-compiler wget python3-dev python3-venv python3-tk xz-utils file make gcc gcc-multilib g++-multilib libsdl2-dev libmagic1`. In our own setup, the `nucleo_f103rb` Blinky sample and this example both built successfully even before those packages were installed — devicetree processing here didn't end up needing the system `dtc` binary — but install them anyway for any Zephyr work beyond this one example.

Every build/flash command below assumes:

```sh
source ~/zephyrproject/.venv/bin/activate
export ZEPHYR_SDK_INSTALL_DIR=~/zephyr-sdk-0.17.2
export ZEPHYR_BASE=~/zephyrproject/zephyr
```

## Build the Rust library and generate its header

```sh
rustup target add thumbv7m-none-eabi
cargo install cbindgen --version 0.29.0 --locked

cargo build --manifest-path rust/led_logic/Cargo.toml --release \
    --target thumbv7m-none-eabi --target-dir rust/led_logic/target

cd rust/led_logic
cbindgen --lang c --crate led_logic --output ../../include/led_logic.h
cd ../..
```

`include/led_logic.h` is checked in (matching `Cargo.lock`'s role elsewhere in this book) since it's generated, not hand-written — but regenerate it after changing `led_pattern()`'s signature.

## Build and flash the Zephyr application

```sh
west build -b nucleo_f103rb -d build .
```

**Flashing**: use `probe-rs` rather than `west flash`'s default `stm32cubeprogrammer`/`openocd` runners — see **Notes** below for why.

```sh
probe-rs download --chip STM32F103RBTx --connect-under-reset build/zephyr/zephyr.elf
```

Press the board's physical RESET button afterward. `--connect-under-reset` flashes reliably but leaves the core halted rather than auto-resuming it, so the firmware won't start running until you do.

To confirm the edit/rebuild/reflash loop, change `rust/led_logic/src/lib.rs` line 9 from `0 | 2 => 1,` to `0 | 2 | 4 => 1,`, then repeat the Cargo build, `west build`, and `probe-rs download` steps (no need to regenerate the header — the signature didn't change).

## Notes

**Why `probe-rs` instead of `west flash`** In our own testing, `west flash -d build --runner openocd` (using the Zephyr SDK's bundled OpenOCD) reported a successful write (`wrote 15260 bytes ... in 0.94s`) but then exited with a `FATAL ERROR` during its own post-flash reset/shutdown sequence, leaving the core halted rather than running. A plain `probe-rs download` (no `--connect-under-reset`) can itself fail with `SwdApWdataError` if the board was already idling — see the next note. `probe-rs download --connect-under-reset` was the combination that worked reliably every time in our testing; a manual physical reset afterward starts the flashed firmware.

**The "`wfi()` breaks SWD on STM32F1" gotcha applies here too, from Zephyr's own idle thread** — not just our own bare-metal examples. Zephyr's default idle loop executes `wfi()` on Cortex-M when there's no ready thread, which gates the debug clock the same way our examples 01-04 had to account for by hand. Symptom: a `probe-rs` command issued while the board is idling (a plain `download`, a standalone `reset`) can fail with `SwdApWdataError`/`SwdDpWait`, even though the firmware itself is running correctly. This is a debug-tooling nuisance, not an application bug — a physical reset or `--connect-under-reset` recovers the connection.

**Zephyr's boot banner is a good crash/hang diagnostic** — the NUCLEO's onboard ST-Link exposes a virtual COM port (`/dev/ttyACM0` on Linux, 115200 8N1) carrying Zephyr's console by default (`CONFIG_BOOT_BANNER=y` is on unless disabled). `*** Booting Zephyr OS build v4.2.0 ***` printing confirms the kernel reached console-driver init; silence after a clean reset points to a problem earlier than that, before application code (and before any call into Rust) ever runs.

**`led_logic`'s `Cargo.toml`/`lib.rs` are exactly as the chapter text shows**, unmodified for this board port — the whole point of this integration style is that the Rust component never touches board-specific GPIO details at all; only `main.c`'s devicetree lookup (already board-agnostic via the `led0` alias) and the `west build -b <board>` argument change between boards.
