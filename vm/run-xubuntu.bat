@echo off
set QEMU="C:\Program Files\qemu\qemu-system-x86_64.exe"
set DISK="%~dp0xubuntu-rust-distribution.qcow2"

%QEMU% ^
  -machine q35,accel=whpx ^
  -cpu qemu64 ^
  -smp 4 ^
  -m 8192 ^
  -drive file=%DISK%,format=qcow2,if=virtio ^
  -device virtio-vga ^
  -display gtk ^
  -device virtio-net-pci,netdev=net0 ^
  -netdev user,id=net0 ^
  -usb ^
  -device usb-tablet
  
pause