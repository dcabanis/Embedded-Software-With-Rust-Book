# example_07  Stack overflow detection with `flip-link`

This example demonstrates the difference between a stack overflow with and
without `flip-link` on the STM32F103RBT6 (NUCLEO-F103RB).  The firmware calls an
infinitely recursive function (`recurse`) that allocates 512 bytes of stack per
frame until the 20 KiB RAM is exhausted (approximately 35–40 frames).

## Setup

Install `flip-link` once per machine:

```sh
cargo install flip-link
```

To **enable** flip-link, uncomment the `"-C", "linker=flip-link"` line in
`.cargo/config.toml`.  To **disable** it for comparison, leave the line
commented out.

## Build

```sh
cargo build --release
```

## Run  Path A: probe-rs (recommended)

```sh
cargo run --release
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
