# example_05  Structured Logging with `defmt`

This example demonstrates `defmt` output on the STM32F103RBT6 (NUCLEO-F103RB).
The firmware logs one message at the `INFO` level via `defmt::info!()` once
per second, repeating indefinitely.

`defmt` is a binary logging framework. Instead of formatting strings on the
device, it emits a compact token that references a format string stored in the
ELF binary. The host tool reconstructs the message by reading both the token
stream and the ELF symbol table. This makes each log call cheaper and keeps
the binary smaller than a `core::fmt`-based approach.

## Transport

This example uses `defmt-rtt` as the transport backend. Log frames are
delivered over RTT, the same SWD-based memory-read mechanism used in
example_04. The same probe requirement applies: the debug probe must remain
connected while the firmware runs.

## Probe requirement

The NUCLEO-F103RB's on-board ST-Link v2-1 handles everything. No SWO pin is
required.

## OpenOCD cannot display defmt output

OpenOCD has no defmt decoder. It can receive the raw RTT bytes via its RTT
server, but those bytes are binary-encoded log frames that are meaningless
without the ELF symbol table to map them back to format strings. Viewing
defmt output requires a tool that can read both the byte stream and the ELF:
`probe-rs run` and `cargo embed` both do this. `run_openocd.sh` is provided
for flashing only.

## Build

```sh
cargo build --release
```

## Run Path A: probe-rs (recommended)

`probe-rs run` flashes the firmware, attaches to the target, and decodes
defmt frames automatically by reading the ELF symbol table alongside the RTT
stream.

```sh
cargo run --release
```

Expected output (one line per second):

```
INFO  Structured log from defmt
└─ app::__cortex_m_rt_main @ src/main.rs:12
INFO  Structured log from defmt
└─ app::__cortex_m_rt_main @ src/main.rs:12
```

Press `Ctrl-c` to stop.

## Run Path B: cargo embed

`cargo embed` reads `Embed.toml` in the project root and performs the same
flash, reset, and defmt-decoded RTT display cycle. It ships as part of
`probe-rs-tools` alongside `probe-rs run`.

Install if not already present:

```sh
cargo install probe-rs-tools
```

Then run:

```sh
cargo embed --release
```

Expected output is identical to Path A.

## .cargo/config.toml notes

**`-Tdefmt.x` linker script** `rustflags` includes `-C link-arg=-Tdefmt.x`
in addition to the usual `-Tlink.x`. The `defmt.x` linker script is provided
by the `defmt-rtt` build script and places the interned format strings and log
metadata in their own ELF section. Without it the firmware will not compile.

**`DEFMT_LOG` environment variable** — probe-rs filters defmt output using
this variable, the same way `RUST_LOG` controls host-side Rust logging. If it
is not set, probe-rs receives the defmt frames over RTT but prints nothing.
`.cargo/config.toml` sets `DEFMT_LOG = "info"` so `cargo run` works without
any extra shell setup. The available levels are `trace`, `debug`, `info`,
`warn`, and `error`; set to `trace` to see all log levels.

## Cargo.toml note: debug = 2 in release profile

The `[profile.release]` section sets `debug = 2`, which keeps full DWARF
debug information in the release binary. Without it, probe-rs emits:

```
WARN probe_rs::util::rtt: Insufficient DWARF info; compile with `debug = 2`
```

and the source location (`@ src/main.rs:N`) is omitted from log output.
The `debug = 2` flag does not affect optimisation level or runtime performance;
it only increases the size of the ELF file on the host, not the firmware
flashed to the device.
