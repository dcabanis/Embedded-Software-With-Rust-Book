# example_05  Lightweight fixed-capacity containers with `arrayvec`

This example demonstrates `ArrayVec<T, N>` and `ArrayString<N>` from the
`arrayvec` crate on the STM32F103C8T6 (Blue Pill).

`arrayvec` is a lighter dependency than `heapless` when the project only needs
a fixed-capacity vector or string without the broader toolbox (`Deque`,
`BinaryHeap`, lock-free queues).  No global allocator is required.


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
INFO  ArrayVec: 16 elements, last = 15
INFO  try_push rejected: buffer full, value = 85
INFO  ArrayString: "sensor-01" (len=9)
INFO  try_push_str rejected: would exceed 32-byte capacity
INFO  ArrayString unchanged: "sensor-01"
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

