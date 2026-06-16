# example_09  Hardware-enforced stack guard with the MPU

**Target board: STM32F3DISCOVERY (STM32F303VCT6, Cortex-M4F)**

This example configures the Memory Protection Unit (MPU) to enforce a 32-byte
no-access guard region at the bottom of RAM, providing hardware-level detection
of stack overflows.

The STM32F103C8T6 (Blue Pill, Cortex-M3) used in examples 01–08 does **not**
implement the optional MPU, so this example targets the STM32F3DISCOVERY board
instead.  The STM32F303VCT6 has a Cortex-M4F core with an 8-region ARMv7-M MPU.

## How it works

In the default `cortex-m-rt` layout the stack occupies the top of RAM and grows
downward.  A severe overflow would eventually write below `ORIGIN(RAM)`.

`install_stack_guard` programs MPU region 0 as a 32-byte no-access
execute-never region starting at `_stack_bottom = ORIGIN(RAM) = 0x2000_0000`.
Any read, write, or fetch within that region raises a MemManage fault, which
escalates to HardFault if the MemManage exception is not separately enabled.
The fault is precise and hardware-enforced — no polling, no canary writes.

## Register encoding (ARMv7-M MPU)

| Register | Field | Value | Meaning |
|----------|-------|-------|---------|
| `RBAR` | `ADDR[31:5]` | `0x2000_0000` | guard base address |
| `RBAR` | `VALID` | 1 | write `REGION` field in this access |
| `RBAR` | `REGION` | 0 | configure region 0 |
| `RASR` | `SIZE[5:1]` | 4 | region size = 2^(4+1) = 32 bytes |
| `RASR` | `AP[26:24]` | 0b000 | no access (privileged or unprivileged) |
| `RASR` | `XN` | 1 | execute-never |
| `RASR` | `ENABLE` | 1 | region enabled |
| `CTRL` | `ENABLE` | 1 | MPU enabled |
| `CTRL` | `PRIVDEFENA` | 1 | default map active as background |

## Board and toolchain differences from examples 01–08

| Property | Blue Pill (ex. 01–08) | STM32F3DISCOVERY (this example) |
|----------|-----------------------|----------------------------------|
| MCU | STM32F103C8T6 | STM32F303VCT6 |
| Core | Cortex-M3 (no MPU) | Cortex-M4F (MPU, FPU) |
| Build target | `thumbv7m-none-eabi` | `thumbv7em-none-eabihf` |
| probe-rs chip | `STM32F103C8Tx` | `STM32F303VCTx` |
| FLASH | 64 KiB | 256 KiB |
| RAM | 20 KiB | 40 KiB |
| OpenOCD target | `stm32f1x.cfg` | `stm32f3x.cfg` |

## Build

```sh
cargo build --release
```

## Run  Path A: probe-rs (recommended)

```sh
cargo run --release
```

Expected output:

```
INFO  Installing MPU stack guard at 0x20000000 (32-byte region)
INFO  Stack guard active. Addresses 0x20000000–0x2000001f will fault on any access.
INFO  Normal operation continues — no access to the guard region.
```

The program then loops indefinitely.

To verify the guard fires, uncomment the `write_volatile` call in `main`.
The device will fault immediately and probe-rs will show the target halted.

## Run  Path B: cargo embed

```sh
cargo embed --release
```

## Run  Path C: OpenOCD (flash only)

```sh
cargo build
./run_openocd.sh
```

