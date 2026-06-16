#!/usr/bin/env bash
set -euo pipefail
gdb-multiarch -q -x .gdbinit target/thumbv7m-none-eabi/debug/app
