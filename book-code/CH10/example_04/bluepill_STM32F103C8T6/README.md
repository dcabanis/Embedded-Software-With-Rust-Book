# example_04  bindgen: generating Rust bindings from a real C library

This example covers *bindgen* to generate Rust FFI bindings automatically instead of hand-writing them, against a real third-party C library rather than the chapter's own `example.h`, on the STM32F103C8T6 (Blue Pill).

`c/jsmn/jsmn.h` vendors [zserge/jsmn](https://github.com/zserge/jsmn)
unmodified (MIT licensed), a minimal, dependency-free JSON tokenizer that
parses into a caller-supplied, fixed-size token array with no dynamic
allocation, which is exactly what makes it usable in `no_std` firmware.

## The bindgen workflow 

1. **Collect the header** `c/jsmn/jsmn.h`, vendored as-is.
2. **Create a wrapper header** `c/wrapper.h` defines `JSMN_HEADER` before
   including `jsmn.h`, so bindgen sees only the public declarations
   (`jsmn_init`, `jsmn_parse`, `jsmntok_t`, `jsmntype_t`, `jsmn_parser`),
   not jsmn's internal static helper functions.
3. **Run bindgen on the wrapper, allowlisted, target-aware** `build.rs`
   does this automatically: it passes `--target=<TARGET>` (read
   from Cargo's `TARGET` env var, not the host), so libclang parses the
   headers with the *firmware's* type sizes, not the build machine's.
4. **Bring the generated file into the crate** `src/main.rs` pulls it in
   from `OUT_DIR` via `include!()`.

The actual library code is built separately: `c/jsmn_impl.c` is a
one-line file that `#include`s `jsmn.h` *without* `JSMN_HEADER`, giving
`jsmn_init()`/`jsmn_parse()` real external linkage; `build.rs` compiles it
with the `cc` crate.

## A real enum-sizing mismatch, not a hypothetical one

`jsmntype_t` is a plain C `enum`. Building this example surfaced a genuine instance of that warning, not just the book's illustrative one: `arm-none-eabi-gcc` applies `-fshort-enums` by default on this target, shrinking `jsmntype_t` to a single byte, but libclang (which bindgen runs headers through) does *not*
assume that on its own left unset, bindgen would bind `jsmntype_t` as a
4-byte `c_uint`, silently disagreeing with what the compiled library
actually contains.

`build.rs` passes `-fshort-enums` to both the `cc` compilation and the
bindgen invocation, so the two stay in lock-step. 

`bindgen`'s default output represents the enum as a type alias plus loose
`pub const` values (not a Rust `enum`), the automatic equivalent of the
`typedef uint8_t` + `#define` pattern.

## What it does

`src/main.rs` parses a small embedded JSON string
(`{"id":42,"name":"blue-pill","active":true}`) into a fixed 16-token
array and logs each token's type and boundaries via `defmt`:

```
INFO  parsed 7 tokens from the embedded JSON string
INFO    token[0]: type=OBJECT start=0 end=42
INFO    token[1]: type=STRING start=2 end=4
INFO    token[2]: type=PRIMITIVE start=6 end=8
INFO    token[3]: type=STRING start=10 end=14
INFO    token[4]: type=STRING start=17 end=26
INFO    token[5]: type=STRING start=29 end=35
INFO    token[6]: type=PRIMITIVE start=37 end=41
```

## Build

```sh
cargo build --release
```

`bindgen-cli` is not required to build this example, bindgen is used as a
library from `build.rs`, not the command line. It does need `libclang`
installed on the build machine (`apt install libclang-dev` or similar).

## Run Path A: probe-rs (recommended)

```sh
cargo run --release
```

## Run Path B: OpenOCD (flash only)

`defmt` output cannot be decoded by OpenOCD. Use `run_openocd.sh` only to
flash the firmware; switch to probe-rs to view the log.

```sh
cargo build
./run_openocd.sh
```
