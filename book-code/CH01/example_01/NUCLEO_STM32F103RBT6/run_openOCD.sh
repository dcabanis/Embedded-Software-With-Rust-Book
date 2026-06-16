#!/usr/bin/env bash
set -euo pipefail

# Override with:
#   OPENOCD_ADAPTER_KHZ=400 ./run_openOCD.sh
#   ELF=target/thumbv7m-none-eabi/release/NUCLEO-blinky ./run_openOCD.sh
ELF="${ELF:-target/thumbv7m-none-eabi/release/NUCLEO-blinky}"
OPENOCD_ADAPTER_KHZ="${OPENOCD_ADAPTER_KHZ:-100}"

if [[ ! -f "$ELF" ]]; then
  echo "ELF not found: $ELF" >&2
  echo "Build first with: cargo build --release" >&2
  exit 1
fi

echo "Flashing ELF: ${ELF}"
echo "OpenOCD speed: ${OPENOCD_ADAPTER_KHZ} kHz"

openocd \
  -f interface/stlink.cfg \
  -f target/stm32f1x.cfg \
  -c "transport select hla_swd" \
  -c "adapter speed ${OPENOCD_ADAPTER_KHZ}" \
  -c "reset_config srst_only srst_nogate connect_assert_srst" \
  -c "adapter srst delay 100" \
  -c "adapter srst pulse_width 100" \
  -c "init" \
  -c "reset halt" \
  -c "arm semihosting enable" \
  -c "program ${ELF} verify reset" \
  -c "shutdown"
