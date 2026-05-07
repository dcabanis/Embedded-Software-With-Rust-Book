#!/usr/bin/env bash
set -euo pipefail
ELF="target/thumbv7m-none-eabi/debug/app"
OPENOCD_BIN="${OPENOCD_BIN:-/usr/bin/openocd}"
ITM_FILE="${ITM_FILE:-/tmp/itm.txt}"
# CPU clock in Hz. This firmware does not configure the PLL, so the core
# runs at the HSI default of 8 MHz. Change this value if you configure a
# different clock in main.rs.
CPU_CLOCK_HZ=8000000

if [[ ! -f "$ELF" ]]; then
  echo "Missing $ELF. Build first with: cargo build"
  exit 1
fi
if [[ ! -x "$OPENOCD_BIN" ]]; then
  OPENOCD_BIN="$(command -v openocd)"
fi
pkill -x openocd >/dev/null 2>&1 || true
sleep 1

# Create / clear the capture file before OpenOCD starts.
: > "$ITM_FILE"

echo "ITM capture → $ITM_FILE"
echo "Once flashing is done, start in a second terminal:  itmdump -F -f $ITM_FILE"
echo "Then press the RESET button on the board to capture the message."
echo ""

"$OPENOCD_BIN" \
  -f interface/stlink-v2-1.cfg \
  -f target/stm32f1x.cfg \
  -c "transport select hla_swd" \
  -c "adapter speed 50" \
  -c "reset_config none" \
  -c "init" \
  -c "halt" \
  -c "tpiu config internal $ITM_FILE uart off $CPU_CLOCK_HZ" \
  -c "itm port 0 on" \
  -c "program $ELF verify" \
  -c "reset run"
# OpenOCD stays running to capture SWO data. Press Ctrl-C to stop.
