#!/usr/bin/env bash
set -euo pipefail
# Flash only — defmt output requires the ELF symbol table to decode.
# Use 'cargo run --release' (probe-rs) to see the log output.
ELF="target/thumbv7em-none-eabihf/debug/app"
OPENOCD_BIN="${OPENOCD_BIN:-/usr/bin/openocd}"
if [[ ! -f "$ELF" ]]; then
  echo "Missing $ELF. Build first with: cargo build"
  exit 1
fi
if [[ ! -x "$OPENOCD_BIN" ]]; then
  OPENOCD_BIN="$(command -v openocd)"
fi
pkill -x openocd >/dev/null 2>&1 || true
sleep 1
"$OPENOCD_BIN" \
  -f interface/stlink.cfg \
  -f target/stm32f3x.cfg \
  -c "transport select hla_swd" \
  -c "adapter speed 50" \
  -c "reset_config none" \
  -c "init" \
  -c "halt" \
  -c "program $ELF verify" \
  -c "reset run" \
  -c "exit"
