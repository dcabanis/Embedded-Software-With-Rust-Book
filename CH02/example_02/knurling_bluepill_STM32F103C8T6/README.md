# example_02 — defmt RTT logging with the Knurling template on the Blue Pill

This is a Knurling-based project template that demonstrates `defmt` structured
logging over RTT. Multiple binary targets are provided: `hello` prints a
greeting, `levels` shows all defmt log levels controlled by the `DEFMT_LOG`
environment variable, `format` demonstrates the `Format` derive macro for custom
struct formatting, `bitfield` shows bitfield extraction syntax for reading
register fields, `panic` triggers a defmt panic, and `overflow` exhausts the
stack with a recursive Ackermann function to demonstrate stack-overflow
detection.

## Build

```sh
cargo build --release --bin hello
```

Replace `hello` with any of the available targets: `hello`, `levels`, `format`,
`bitfield`, `panic`, `overflow`.

## Run — probe-rs

RTT output appears in the terminal that invokes `cargo run`.

```sh
cargo run --release --bin hello
```

Replace `hello` with any available target. To control log verbosity for the
`levels` binary:

```sh
DEFMT_LOG=trace cargo run --release --bin levels
```
