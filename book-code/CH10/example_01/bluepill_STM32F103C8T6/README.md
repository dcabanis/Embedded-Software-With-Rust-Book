# example_01  Calling C from Rust: FFI basics and safe wrappers

This example demonstrates the mechanics of calling hand-written
C code from Rust on the STM32F103C8T6 (Blue Pill).

A small C library (`c/device_status.c` / `c/device_status.h`) exposes two
functions:

- `int32_t add(int32_t a, int32_t b)` — a plain scalar function.
- `void update_status(DeviceStatus *status, float delta_temp)` — mutates a
  struct through a pointer, and flips its `state` field once the
  accumulated temperature crosses a threshold.

On the Rust side, `src/main.rs` shows the full pattern from the chapter:

- The C declarations and their `#[allow(...)]` lint suppressions are
  quarantined inside a private `ffi` module, instead of silencing
  those lints crate-wide.
- `DeviceStatus` is `#[repr(C)]`, matching the C header's struct layout
  exactly. A `const _: () = assert!(...)` pins the 12-byte size
  at compile time, mirrored by a `_Static_assert` on the C side.
- `DeviceState` is *not* a plain `#[repr(u8)] enum` : C only ever
  sees a `uint8_t`, and nothing stops it from writing a value outside
  `{0, 1, 2}`. Reading an arbitrary byte back as a real Rust enum the
  moment it isn't one of the declared variants would be undefined
  behavior. Instead, `DeviceState` is a `#[repr(transparent)]` wrapper
  around a plain `u8`, and only `DeviceState::checked()`, which returns
  `Option<CheckedDeviceState>` , turns a validated byte into a real,
  exhaustively-matchable enum. `state_name()` uses `checked()` for
  logging, treating anything outside the three valid variants as a fault
  (`"INVALID"`) rather than guessing.
- `safe_add()` and `safe_update_status()` wrap the `unsafe extern "C"`
  calls behind ordinary safe functions.
- `#![deny(improper_ctypes)]` (imported C signatures) and
  `#![deny(improper_ctypes_definitions)]` (exported C signatures) at the
  crate root turn any future layout mistake into a compile error rather
  than a warning.

The C file is compiled and linked automatically by `build.rs` using the
`cc` crate, indirect linking, as described in, so no manual
`#[link(...)]` attribute or prebuilt `.a` file is needed.

Results are logged via `defmt`:

```
INFO  add(1940, 46) = 1986
INFO  initial status: id=7 state=Idle temperature=20.0
INFO  after +5.5C: id=7 state=Active temperature=25.5
INFO  after +60.0C: id=7 state=Error temperature=85.5
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
