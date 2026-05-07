# example_06  Configurable Fault Handlers and HardFault Diagnosis

This example demonstrates the following techniques on the STM32F103RBT6
(NUCLEO-F103RB):

1. **Configurable fault handlers** — `MemoryManagement`, `BusFault`, and
   `UsageFault` each route to their own `#[exception]` handler, which calls
   `panic!()` so the panic handler can log the fault via RTT.

2. **A custom `#[panic_handler]`** — routes panic messages through RTT so
   they appear in the probe-rs console.

3. **HardFault handler with full register decode** — prints the CPU register
   snapshot from `ExceptionFrame`, then reads and decodes HFSR, CFSR
   (including the UFSR, BFSR, and MMFSR sub-fields), and prints the faulting
   address from MMFAR or BFAR when the address-valid bits are set.

A deliberate fault is injected on every boot to drive the demonstration.

## Build

```sh
cargo build
```

## Run

Two paths are available depending on which tool you prefer.

**Option A probe-rs (simplest):**

```sh
cargo run --release
```

probe-rs flashes the binary and prints RTT output directly in the terminal.

**Option B OpenOCD + GDB + telnet:**

Terminal 1, flash the binary and leave OpenOCD running:

```sh
cargo build
./run_openocd.sh
```

Terminal 2, connect GDB, start the RTT server:

```sh
gdb-multiarch target/thumbv7m-none-eabi/release/app
```

```
(gdb) target extended-remote :3333
(gdb) monitor rtt setup 0x20000000 0x5000 "SEGGER RTT"
(gdb) monitor rtt start
(gdb) monitor rtt server start 9090 0
(gdb) continue
```

Terminal 3, read RTT output:

```sh
telnet localhost 9090
```

The `0x20000000` / `0x5000` arguments tell OpenOCD where to search for the
RTT control block (start of SRAM, covering the full 20 KiB of the NUCLEO-F103RB).
`"SEGGER RTT"` is the signature string that `rtt-target` writes into the block.

**Shutting down (Option B):**

Stop in reverse order:

- Terminal 3 (telnet): `Ctrl-]` then type `quit` and press Enter
- Terminal 2 (GDB): `(gdb) quit` — GDB disconnects from OpenOCD
- Terminal 1 (OpenOCD): `Ctrl-C` terminates the OpenOCD process

## Demonstration: two modes

The default firmware **enables** the three configurable fault exceptions via
`SHCSR`. With that in place, a `UDF` instruction routes to `UsageFault()`,
which calls `panic!()`. RTT shows:

```
MemManage, BusFault, UsageFault handlers enabled.
Triggering UsageFault via UDF...
panicked at 'UsageFault', src/main.rs:...
```

To see the **HardFault handler** with the full CFSR/HFSR decode instead,
comment out the `SHCSR` enable block in `main` and re-flash. Without those
enable bits the `UDF` escalates to HardFault, and RTT shows:

```
Triggering UsageFault via UDF...
=== HardFault ===
ExceptionFrame (CPU state at fault):
  r0:   0x00000000
  ...
  pc:   0x0800xxxx  <-- faulting instruction
  xpsr: 0x01000000
  HFSR: 0x40000000
  FORCED    escalated from a configurable fault (see CFSR)
  CFSR: 0x00010000
  UFSR: 0x0001  (UsageFault)
  UNDEFINSTR   undefined or unsupported instruction
```

The HFSR `FORCED` bit (0x40000000) confirms escalation from a configurable
fault. The CFSR `UNDEFINSTR` bit in the UFSR field identifies the cause.

## CFSR sub-register layout

| Bits  | Register | Fault class |
|-------|----------|-------------|
| 31:16 | UFSR     | UsageFault  |
| 15:8  | BFSR     | BusFault    |
|  7:0  | MMFSR    | MemManage   |

Faulting addresses appear in BFAR (when BFSR.BFARVALID is set) and MMFAR
(when MMFSR.MMARVALID is set). Both are printed by the handler when valid.
