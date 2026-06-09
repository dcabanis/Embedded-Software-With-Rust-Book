# example_01  Global allocator with `embedded-alloc` (`LlffHeap`)

This example installs a global heap allocator using the `embedded-alloc`
crate's linked-list first-fit allocator (`LlffHeap`) on the STM32F103C8T6
(Blue Pill).  Once initialised, the full `alloc` API is available: `Box`,
`Vec`, `String`, and every other growable collection.

The firmware demonstrates two allocations:

1. `Box::new(1986u32)` — allocates a single `u32` on the heap, reads it back,
   and drops it (returns the memory to the allocator).
2. `Vec::<u32>::new()` followed by three `push` calls — shows the allocator
   growing a buffer on demand.

Both results are logged via `defmt` and visible in the `probe-rs` terminal.

## Build

```sh
cargo build --release
```

## Run  Path A: probe-rs (recommended)

```sh
cargo run --release
```

Expected output:

```
INFO  Box::new succeeded, value = 1986
└─ app::main @ src/main.rs
INFO  Vec len=3, values=[10, 20, 30]
└─ app::main @ src/main.rs
```

## Run  Path B: cargo embed

```sh
cargo embed --release
```

Expected output is identical to Path A.

## Run  Path C: OpenOCD (flash only)

`defmt` output cannot be decoded by OpenOCD.  Use `run_openocd.sh` only to
flash the firmware; switch to probe-rs or cargo embed to view the log.

```sh
cargo build
./run_openocd.sh
```

## Notes

**`-Tdefmt.x` linker script**  `.cargo/config.toml` passes
`-C link-arg=-Tdefmt.x` to the linker.  This script (provided by the
`defmt-rtt` build script) places interned format strings in their own ELF
section.  Without it the firmware will not compile.

**`DEFMT_LOG`**  set to `"info"` in `.cargo/config.toml` so `cargo run`
prints without extra shell configuration.  Set to `"trace"` to see all levels.

**`critical-section-single-core`**  `embedded-alloc` serialises allocation
with a critical section.  The `critical-section-single-core` feature on the
`cortex-m` crate provides the implementation for single-core Cortex-M targets.
