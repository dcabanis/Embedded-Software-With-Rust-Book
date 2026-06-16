# example_06  Hybrid inline-or-heap containers: `tinyvec` and `smallvec`

This example demonstrates `TinyVec` and `SmallVec` on the STM32F103C8T6
(Blue Pill).  Both types store the first N elements inline on the stack; once
that inline capacity is exceeded, the data is moved to a heap allocation and
the vector continues growing like a regular `Vec<T>`.

**Important:** both crates still require a global allocator.  This example
installs `embedded-alloc`'s `LlffHeap` (same as example_01) to handle the
heap spill.  If your project does not otherwise need a heap, prefer a
fixed-capacity type from `heapless` (example_04) or `arrayvec` (example_05).

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
INFO  TinyVec: len=8 is_heap=false
INFO  TinyVec after spill: len=9 is_heap=true
INFO  SmallVec: len=8 spilled=false
INFO  SmallVec after spill: len=9 spilled=true
```

## Run  Path B: cargo embed

```sh
cargo embed --release
```

## Run  Path C: OpenOCD (flash only)

```sh
cargo build
./run_openocd.sh
```
