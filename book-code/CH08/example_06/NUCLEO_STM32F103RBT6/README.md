# example_06 Rust inside a Zephyr application (NUCLEO-F103RB)

Zephyr keeps the main thread, the GPIO driver, and the build; Rust supplies one pure-logic function, `led_pattern()`. A Kconfig option, `CONFIG_LED_PATTERN_RUST`, selects between the Rust implementation and the original C one, so the migration can be reverted without editing any source.

LD2 (the green user LED, near the Arduino D13 pin) shows two short flashes followed by a longer pause, repeating about every two seconds. 

The Rust build gives two flashes; changing the match arm to `0 | 2 | 4` gives three, with no C edits; rebuilding with `-DCONFIG_LED_PATTERN_RUST=n` returns to two flashes from the C fallback.

## Board

The chapter text uses a stock Blue Pill (`stm32_min_dev@blue`); see `../bluepill_stock_STM32F103C8T6` for that variant. This port uses Zephyr's official `nucleo_f103rb` board, whose devicetree maps the `led0` alias to LD2 (PA5). No devicetree overlay or board-specific code is needed.

Every source file (`CMakeLists.txt`, `Kconfig`, `prj.conf`, `src/`, `include/`, `rust/led_logic/`) is byte-for-byte identical to the Blue Pill variant. Only the `west build -b` argument and the probe-rs chip name change. That is the point of this integration style: neither the Rust component nor the C application knows which board it runs on.

## Environment

Built with Zephyr 4.2.0, Zephyr SDK 0.17.2, Rust 1.98.1, and cbindgen 0.29.0.

If you don't have a Zephyr workspace yet:

```sh
python3 -m venv ~/zephyrproject/.venv
source ~/zephyrproject/.venv/bin/activate
pip install west
cd ~/zephyrproject
west init -m https://github.com/zephyrproject-rtos/zephyr --mr v4.2.0 .
west update
pip install -r zephyr/scripts/requirements.txt

# Zephyr SDK 0.17.2 (no sudo required):
mkdir ~/zephyr-sdk-install && cd ~/zephyr-sdk-install
wget https://github.com/zephyrproject-rtos/sdk-ng/releases/download/v0.17.2/zephyr-sdk-0.17.2_linux-x86_64_minimal.tar.xz
tar xf zephyr-sdk-0.17.2_linux-x86_64_minimal.tar.xz -C ~/
~/zephyr-sdk-0.17.2/setup.sh -t arm-zephyr-eabi -h -c
```

Also install Zephyr's host packages (needs `sudo`): `git cmake ninja-build gperf ccache dfu-util device-tree-compiler wget python3-dev python3-venv python3-tk xz-utils file make gcc gcc-multilib g++-multilib libsdl2-dev libmagic1`.

Every command below assumes:

```sh
source ~/zephyrproject/.venv/bin/activate
export ZEPHYR_SDK_INSTALL_DIR=~/zephyr-sdk-0.17.2
export ZEPHYR_BASE=~/zephyrproject/zephyr
rustup target add thumbv7m-none-eabi
cargo install cbindgen --version 0.29.0 --locked
```

> If `west` fails with `ModuleNotFoundError: No module named 'west'` after an OS upgrade, the system Python the virtual environment was built on has gone away. Recreate it with a Python version Zephyr supports, for example using `uv`:
>
> ```sh
> cd ~/zephyrproject
> mv .venv .venv.old
> uv venv --python 3.12 .venv
> VIRTUAL_ENV=$PWD/.venv uv pip install west -r zephyr/scripts/requirements.txt
> ```

## Build the Rust library and generate its header

From this directory:

```sh
cargo build \
    --manifest-path rust/led_logic/Cargo.toml \
    --release \
    --target thumbv7m-none-eabi \
    --target-dir rust/led_logic/target

(cd rust/led_logic && \
    cbindgen --config cbindgen.toml --crate led_logic \
             --output ../../include/led_logic.h)
```

## Build the firmware

Rust implementation (the default):

```sh
west build -p always -b nucleo_f103rb -d build .
```

Original C implementation, without editing any file:

```sh
west build -p always -b nucleo_f103rb -d build . \
    -- -DCONFIG_LED_PATTERN_RUST=n
```

Check which one you built with `grep LED_PATTERN build/zephyr/.config`. If the Rust archive is missing while `CONFIG_LED_PATTERN_RUST=y`, CMake stops with `Rust library missing: run the Cargo build command first.`

## Flash

```sh
openocd -f interface/stlink.cfg -f target/stm32f1x.cfg \
    -c "reset_config srst_only connect_assert_srst" \
    -c "program build/zephyr/zephyr.elf verify" \
    -c "reset halt" \
    -c "mww 0xE000EDF0 0xA05F0000" \
    -c shutdown
```

The firmware starts as soon as OpenOCD exits; no reset button needed.

The last two commands matter. They write `DHCSR` (the Cortex-M Debug Halting Control and Status Register) to switch halting debug off and let the core run. Without them, the core keeps `C_DEBUGEN` set after OpenOCD disconnects, and neither the RESET button nor a software reset clears it (only a power cycle does). On this STM32F1, Zephyr's idle `wfi` then stalls the system tick, so `k_msleep()` never returns and LD2 freezes on or off. That is exactly what `program ... verify reset exit` produces: the image verifies, the console prints Zephyr's boot banner, and the LED never blinks. Unplugging and replugging the USB cable also recovers it.

### About `west flash`

The `west flash -d build` command was **not** used to test this example; every successful flash here used the OpenOCD command above. Avoid `west flash` on this board for now:

- `west flash --runner openocd` (the Zephyr SDK's bundled OpenOCD) wrote the image in earlier testing, then failed with a `FATAL ERROR` during its post-flash reset.
- Its flash sequence ends with a reset, much like `program ... verify reset exit`, which is the sequence that left `C_DEBUGEN` set and froze the LED in our testing. We haven't checked whether `west flash` does the same, so treat a frozen LED after `west flash` as this problem first.

If the LED stays solidly on or off after a successful flash, suspect the debug state before suspecting the code: clear `DHCSR` as above, or power-cycle the board.

## Try it

1. Flash the Rust build: two short flashes, then a pause.
2. Change the Rust match arm in `rust/led_logic/src/lib.rs` from `0 | 2` to `0 | 2 | 4`, rebuild the archive, rebuild and flash: three flashes, without touching any C.
3. Rebuild with `-- -DCONFIG_LED_PATTERN_RUST=n` and flash: back to two flashes, now from `src/led_pattern.c`.
