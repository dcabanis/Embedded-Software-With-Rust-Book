#!/usr/bin/env bash
set -euo pipefail
ELF="target/thumbv7m-none-eabi/debug/app"
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
if command -v st-info >/dev/null 2>&1; then
  CHIPID="$(st-info --chipid 2>/dev/null || true)"
  if [[ "$CHIPID" == "0x0000" || -z "$CHIPID" ]]; then
    echo "Warning: st-info cannot read chipid (got '$CHIPID')."
    echo "Try pressing/holding RESET while running this script, then release after 1-2 seconds."
  fi
fi
echo "RTT output will be available on TCP port 9090."
echo "In a second terminal run:  telnet localhost 9090"
echo "GDB server is also available on port 3333."
echo ""

"$OPENOCD_BIN" \
  -f interface/stlink.cfg \
  -f target/stm32f1x.cfg \
  -c "transport select hla_swd" \
  -c "adapter speed 50" \
  -c "reset_config none" \
  -c "init" \
  -c "halt" \
  -c "program $ELF verify" \
  -c "rtt setup 0x20000000 0x5000 {SEGGER RTT}" \
  -c "reset run" \
  -c "rtt start" \
  -c "rtt server start 9090 0"
# OpenOCD stays running: RTT server on :9090, GDB server on :3333.
# Press Ctrl-c to stop.
