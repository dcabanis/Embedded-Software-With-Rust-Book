#!/usr/bin/env bash
set -euo pipefail

ELF="target/thumbv7m-none-eabi/release/bluepill-blinky"
OPENOCD_BIN="${OPENOCD_BIN:-/usr/bin/openocd}"

if [[ ! -f "$ELF" ]]; then
  echo "Missing $ELF. Build first with: cargo build --release"
  exit 1
fi

if [[ ! -x "$OPENOCD_BIN" ]]; then
  OPENOCD_BIN="$(command -v openocd)"
fi

# Clear stale sessions that may keep the ST-Link busy.
pkill -x openocd >/dev/null 2>&1 || true
sleep 1

if command -v st-info >/dev/null 2>&1; then
  CHIPID="$(st-info --chipid 2>/dev/null || true)"
  if [[ "$CHIPID" == "0x0000" || -z "$CHIPID" ]]; then
    echo "Warning: st-info cannot read chipid (got '$CHIPID')."
    echo "Try pressing/holding RESET while running this script, then release after 1-2 seconds."
  fi
fi

"$OPENOCD_BIN" \
  -f interface/stlink.cfg \
  -f target/stm32f1x.cfg \
  -c "transport select hla_swd" \
  -c "adapter speed 50" \
  -c "reset_config none" \
  -c "init" \
  -c "halt" \
  -c "arm semihosting enable" \
  -c "program $ELF verify" \
  -c "reset run"
