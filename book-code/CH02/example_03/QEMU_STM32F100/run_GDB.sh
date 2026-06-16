#!/usr/bin/env bash
set -euo pipefail

# Single-entry debug script:
# - default mode: build debug, spawn backend terminal, launch gdb-multiarch
# - --backend mode: run the QEMU backend + semihost output + gdb server

ELF_PATH="target/thumbv7m-none-eabi/debug/qemu-stm32f100-blinky"
GDB_PORT="3333"
SCRIPT_PATH="$(realpath "$0")"

launch_backend_terminal() {
  local backend_cmd
  backend_cmd="cd \"$(pwd)\" && \"$SCRIPT_PATH\" --backend; echo; echo \"Backend terminal finished. Press Enter to close.\"; read -r"

  if command -v gnome-terminal >/dev/null 2>&1; then
    gnome-terminal -- bash -lc "$backend_cmd"
    return
  fi

  if command -v x-terminal-emulator >/dev/null 2>&1; then
    x-terminal-emulator -e bash -lc "$backend_cmd"
    return
  fi

  if command -v xterm >/dev/null 2>&1; then
    xterm -hold -e bash -lc "$backend_cmd"
    return
  fi

  echo "Error: no supported terminal emulator found (gnome-terminal, x-terminal-emulator, xterm)." >&2
  echo "Run backend manually in another terminal: $SCRIPT_PATH --backend" >&2
  exit 1
}

wait_for_gdb_server() {
  local tries=0
  local max_tries=60
  while (( tries < max_tries )); do
    if (echo >"/dev/tcp/127.0.0.1/$GDB_PORT") >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.25
    tries=$((tries + 1))
  done
  return 1
}

if ! command -v qemu-system-arm >/dev/null 2>&1; then
  echo "Error: qemu-system-arm not found in PATH." >&2
  exit 1
fi

if [[ "${1:-}" == "--backend" ]]; then
  if [[ ! -f "$ELF_PATH" ]]; then
    echo "Error: missing $ELF_PATH" >&2
    echo "Build first with: cargo build" >&2
    exit 1
  fi

  echo "Starting QEMU backend for STM32VLDISCOVERY (Cortex-M3)"
  echo "Semihost output will appear in this terminal."
  echo "GDB server listening on :$GDB_PORT"

  exec qemu-system-arm \
    -cpu cortex-m3 \
    -machine stm32vldiscovery \
    -nographic \
    -kernel "$ELF_PATH" \
    -semihosting-config enable=on,target=native \
    -S \
    -gdb "tcp::$GDB_PORT"
fi

if ! command -v gdb-multiarch >/dev/null 2>&1; then
  echo "Error: gdb-multiarch not found in PATH." >&2
  exit 1
fi

echo "[1/4] Building debug binary..."
cargo build

if [[ ! -f "$ELF_PATH" ]]; then
  echo "Error: ELF not found at $ELF_PATH" >&2
  exit 1
fi

echo "[2/4] Starting backend in a new terminal..."
launch_backend_terminal

echo "[3/4] Waiting for GDB server on :$GDB_PORT ..."
if ! wait_for_gdb_server; then
  echo "Error: GDB server did not become available on :$GDB_PORT." >&2
  exit 1
fi

echo "[4/4] Launching gdb-multiarch (break main, continue)..."
exec gdb-multiarch \
  -q \
  "$ELF_PATH" \
  -ex "target extended-remote :$GDB_PORT" \
  -ex "monitor reset halt" \
  -ex "break main" \
  -ex "continue"
