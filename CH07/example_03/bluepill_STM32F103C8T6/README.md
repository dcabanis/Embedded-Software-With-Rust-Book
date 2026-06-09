# example_03  Heap in use: dynamic event log with `Vec<String>`

This example builds on example_01's allocator setup to demonstrate a realistic
heap usage pattern: a dynamic event log backed by `Vec<String>`.  Three events
are appended at runtime using `format!` to produce heap-allocated strings.  The
log is then iterated and each entry is emitted via `defmt`.

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
INFO  uart: code 0x00000001
└─ app::main @ src/main.rs
INFO  i2c: code 0x0000002a
└─ app::main @ src/main.rs
INFO  gpio: code 0x000000ff
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
