# NUCLEO-STM32F103RB setup

## Build

cargo build

## Flash and run

./run_openOCD.sh

Alternatively:

cargo run

## Notes

- `.cargo/config.toml` is configured for `probe-rs` chip `STM32F103RBTx`.
- `memory.x` is configured for STM32F103RB memory map (128K flash / 20K RAM).
