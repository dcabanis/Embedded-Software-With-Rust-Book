#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
QEMU_BIN="qemu-system-x86_64"
DISK="$SCRIPT_DIR/xubuntu-rust-distribution.qcow2"

if ! command -v "$QEMU_BIN" >/dev/null 2>&1; then
    echo "Error: '$QEMU_BIN' was not found on this system." >&2
    echo >&2
    echo "QEMU (x86_64 system emulation) is required to run this script." >&2
    echo "Install it on Debian/Ubuntu with:" >&2
    echo >&2
    echo "    sudo apt update" >&2
    echo "    sudo apt install qemu-system-x86 qemu-utils" >&2
    echo >&2
    echo "For hardware-accelerated virtualization (recommended), also install KVM support:" >&2
    echo >&2
    echo "    sudo apt install qemu-kvm" >&2
    echo "    sudo adduser \"\$USER\" kvm   # then log out/in for the group change to take effect" >&2
    echo >&2
    echo "After installing, re-run this script." >&2
    exit 1
fi

if [ ! -f "$DISK" ]; then
    echo "Error: disk image not found at: $DISK" >&2
    exit 1
fi

# Use KVM acceleration when available (requires /dev/kvm to exist and be
# accessible), otherwise fall back to software emulation (tcg).
if [ -r /dev/kvm ] && [ -w /dev/kvm ]; then
    ACCEL="kvm"
    CPU="host"
else
    echo "Warning: /dev/kvm is not accessible; falling back to software emulation (slower)." >&2
    echo "To enable KVM acceleration: sudo apt install qemu-kvm && sudo adduser \"\$USER\" kvm" >&2
    ACCEL="tcg"
    CPU="qemu64"
fi

exec "$QEMU_BIN" \
    -machine q35,accel="$ACCEL" \
    -cpu "$CPU" \
    -smp 4 \
    -m 8192 \
    -drive file="$DISK",format=qcow2,if=virtio \
    -device virtio-vga \
    -display gtk \
    -device virtio-net-pci,netdev=net0 \
    -netdev user,id=net0 \
    -usb \
    -device usb-tablet
