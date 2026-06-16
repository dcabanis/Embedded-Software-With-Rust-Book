# example_04  Fixed-capacity containers with `heapless`

This example demonstrates three `heapless` types in a realistic UART command
parser on the STM32F103C8T6 (Blue Pill): a `Vec<u8, LINE_LEN>` accumulates
incoming bytes, a `String<RESP_LEN>` holds the formatted response, and the
`write!` macro formats a version string at runtime.

No global allocator is needed.  All buffers live on the stack for the duration
of `main`; their capacity is part of their type and fixed at compile time.

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
INFO  rx> HOW DO YOU DO?
INFO  rx> v1.0
INFO  rx> ERR unknown
INFO  rx> ERR overflow
```

## Run  Path B: cargo embed

```sh
cargo embed --release
```

## Run  Path C: OpenOCD (flash only)

```codededsh
cargo build
./run_openocd.sh
```
