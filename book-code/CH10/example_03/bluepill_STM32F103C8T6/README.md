# example_03  Advanced FFI: arrays, C strings, and stateful callbacks

This example covers  the "hard cases" of FFI beyond plain scalars and `#[repr(C)]` structs on the STM32F103C8T6 (Blue Pill).

`c/callback_lib.c` / `.h` provide three C functions:

- `sum_array(const int32_t *arr, size_t len)`  a pointer + length pair,
  the standard C shape for passing an array (10.4.1).
- `c_string_len(const char *s)`  walks a NUL-terminated C string. There's
  no `<string.h>` on this bare-metal target, so the library implements its
  own `strlen`-equivalent.
- `register_callback_ctx()` / `trigger_callback_ctx()`  the two-step
  callback pattern from 10.4.3: register a function now, the library
  calls it later. Unlike the book's first, simpler example, this is the
  *context-carrying* version from the start: the C side also stores an
  opaque `void *ctx` alongside the function pointer and hands it back
  unchanged on every call.

On the Rust side, `src/main.rs` shows:

- `safe_sum_array()` passing `slice.as_ptr()` / `slice.len()` straight
  through — Rust's slice already guarantees the safety C can't.
- `safe_c_string_len()` taking `&CStr` rather than `&str`, so the type
  system itself rules out passing an unterminated Rust string. The `c"..."`
  literal syntax builds the `&CStr` at compile time.
- `count_events()`, the *trampoline*: an `unsafe extern "C" fn` that casts
  the incoming `void *ctx` back into `&AtomicU32` and increments it.
  Nothing in the type system proves the pointer C hands back is still
  valid — that's a promise `main()` makes by construction, not something
  the compiler checks — so the function itself is marked `unsafe`, not
  just the cast inside it.

A polled SysTick (no interrupt) fires `trigger_callback_ctx()` roughly
once a second from the main loop; C then calls back into `count_events()`,
incrementing the counter and logging each firing via `defmt`.

Expected output:

```
INFO  sum_array([10, 20, 30, 40, 50]) = 150
INFO  c_string_len("Hello from Rust") = 15
INFO  callback fired: value=1 (event #1)
INFO  callback fired: value=2 (event #2)
...
```

## Build

```sh
cargo build --release
```

## Run  Path A: probe-rs (recommended)

```sh
cargo run --release
```

## Run  Path B: OpenOCD (flash only)

`defmt` output cannot be decoded by OpenOCD. Use `run_openocd.sh` only to
flash the firmware; switch to probe-rs to view the log.

```sh
cargo build
./run_openocd.sh
```
