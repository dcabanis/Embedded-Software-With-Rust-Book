# example_05 cbindgen: generating a C header from Rust

This example covers *cbindgen* to generate a C header automatically from Rust source on the STM32F103RBT6 (NUCLEO-F103RB).

`rust_lib/src/lib.rs` is self-contained, side-effect-free byte-sum function, the kind of pure logic, since it needs none of the hard FFI machinery (no callbacks, no ownership transfer, no interrupt context) and can be tested on the host before firmware ever sees it.

## The cbindgen workflow

1. **Shape the exported API** `checksum()` carries
   `#[unsafe(no_mangle)] pub unsafe extern "C" fn`,  a null check
   narrows the failure mode but doesn't prove `data` is valid for `len`
   bytes, so the function can't claim to be fully safe.
2. **`cbindgen.toml`** sets `language = "C"`, an include guard, and
   disables doc-comment passthrough.
3. **Generate the header automatically from `build.rs`** rather than
   running `cbindgen` by hand from the command line, this example goes straight to the automated `build.rs` form every `cargo build` regenerates
   `rust_lib/embedded_rust_lib.h`, so it can never fall out of sync with
   the Rust source.
4. **Hand the header and static library to the C build** `main.c`
   `#include`s the generated header and links against `librust_lib.a`.

Generated header (`rust_lib/embedded_rust_lib.h`):

```c
#ifndef EMBEDDED_RUST_LIB_H
#define EMBEDDED_RUST_LIB_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

uint8_t checksum(const uint8_t *data, uintptr_t len);

#endif  /* EMBEDDED_RUST_LIB_H */
```

## What it does

`main.c` reuses example_02's bare-metal harness (hand-written
`startup.s` + `stm32f103.ld`, no `cortex-m-rt`) and checks three cases:
a normal 4-byte buffer, an empty buffer, and a null pointer, the same
defensive checks `checksum()` makes on the Rust side. If every result
matches what the Rust source promises, the NUCLEO-F103RB's user LED (LD2,
on PA5, active-high) blinks steadily; a mismatch (meaning the generated
header disagrees with the compiled library) latches the LED on solid
instead. 

## Build

```sh
make
```

This runs `cargo build` inside `rust_lib/` (regenerating
`embedded_rust_lib.h` as a build-script side effect), then links `main.c`
+ `startup.s` against the resulting static library with
`arm-none-eabi-gcc`, producing `app.elf` and `app.bin`.

## Flash

```sh
./run_openocd.sh
```

Or flash `app.bin` at `0x08000000` with your tool of choice. There is no
`cargo run` here since this isn't a Cargo firmware project.

## Expected behaviour

LD2 blinks at a steady ~4 Hz, confirming `checksum(payload, 4) == 0xA0`,
`checksum(payload, 0) == 0`, and `checksum(NULL, 4) == 0` all matched the
header cbindgen generated.

## Clean

```sh
make clean
```
