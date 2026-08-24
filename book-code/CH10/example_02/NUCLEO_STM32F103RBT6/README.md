# example_02  Calling Rust from C: a `no_std` static library

This example inverts the usual project shape and demonstrates a plain C application owns `main()`, the startup code, and the linker script, and links against a Rust `no_std` static library built separately.
This is the brownfield arrangement, Rust enters underneath an existing C application, not the other way around.

## Layout

- `rust_lib/`  a `staticlib` crate (`crate-type = ["staticlib"]`) exposing
  two `#[unsafe(no_mangle)]` functions:
  - `pub extern "C" fn square(value: i32) -> i32`
  - `pub unsafe extern "C" fn divide(a: i32, b: i32, result: *mut i32) -> i32`,
    which reports success or failure through an integer error code (`0`
    success, `-1` divide by zero or overflow, `-2` null output pointer)
    rather than a `Result`. It's declared `unsafe extern "C" fn`
    because a null check on `result` narrows the failure mode but doesn't
    prove the pointer is valid for a write; `a.checked_div(b)` covers both
    division by zero and the one case a manual `b == 0` check misses,
    `i32::MIN / -1`, which panics unconditionally in Rust regardless of
    build profile.
- `my_rust_lib.h` the hand-written C header declaring both functions.
- `startup.s` a minimal hand-written Cortex-M3 vector table and
  `Reset_Handler` (copies `.data` from Flash, zeroes `.bss`, calls `main`).
- `stm32f103.ld` the linker script placing the vector table, `.text`,
  `.data`/`.bss`, and defining `_stack_top` (128 KiB Flash on the
  STM32F103RBT6, vs. 64 KiB on the Blue Pill).
- `main.c` calls into the Rust library, checks every result (including
  the deliberate divide-by-zero, `INT32_MIN / -1` overflow, and
  null-pointer error paths), then blinks the NUCLEO-F103RB's user LED
  (LD2, on PA5) steadily if every check passed. LD2 is active-high, the opposite polarity from the Blue Pill's PC13, so the on/off logic is inverted from example_02's Blue Pill variant. If any check fails, meaning the two sides disagree about the ABI, the LED latches on solid instead, so a layout mismatch is visible without a debugger.

## Build

The `Makefile` reproduces the exact two-step process.
first `cargo build` compiles the Rust static library for
`thumbv7m-none-eabi`, then `arm-none-eabi-gcc` compiles and links
`main.c` + `startup.s` against it.

```sh
make
```

This produces `app.elf` and `app.bin`. `-Wl,--gc-sections` (paired with
`-ffunction-sections`/`-fdata-sections` on the C side) matters here more
than usual: `core` compiles to a single object file inside
`librust_lib.a`, and without section garbage collection the linker pulls
in the whole thing just to resolve `square`/`divide`, overflowing the
Flash region. With it, the final image is well under 1 KiB.

## Flash

```sh
./run_openocd.sh
```

Or flash `app.bin` at `0x08000000` with your tool of choice (`st-flash`,
`probe-rs`, etc.) there is no `cargo run` here since this isn't a Cargo
firmware project.

## Expected behaviour

LD2 blinks at a steady ~4 Hz (250 ms SysTick-driven toggle), confirming
`square(9) == 81` and that `divide()`'s success path, its two error
paths, and the `INT32_MIN / -1` overflow case all returned exactly what
the Rust side promised.

## Clean

```sh
make clean
```
