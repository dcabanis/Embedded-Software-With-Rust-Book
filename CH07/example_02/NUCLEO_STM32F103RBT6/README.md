# example_02  Global allocator with `buddy_system_allocator` (`LockedHeap`)

This example installs a global heap allocator using the `buddy_system_allocator`
crate on the STM32F103RBT6 (NUCLEO-F103RB).  The buddy allocator organises free
memory into power-of-two size classes.  Each allocation is rounded up to the
next power of two (internal fragmentation), in exchange for bounded worst-case
timing and predictable fragmentation behaviour over long uptimes.


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
INFO  Buddy heap: Box value = 1986
└─ app::main @ src/main.rs
INFO  Buddy heap: Vec len=2, values=[10, 20]
└─ app::main @ src/main.rs
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
