# example_08  Stack usage measurement with software canaries

This example demonstrates the software canary technique for measuring actual
stack usage at runtime on the STM32F103C8T6 (Blue Pill).

The technique has two steps:

1. **Paint** — at the very start of `main`, fill the unused stack region with
   a recognisable 32-bit pattern (`0xDEAD_BEEF`).  The region starts at
   `_stack_bottom` (the first free word above `.data` and `.bss`) and
   extends to just below the current stack pointer.
2. **Inspect** — at any later point, walk upward from `_stack_bottom` and
   count how many words still hold the pattern.  The first word that differs
   marks the deepest stack frame reached since painting.

## Output transport: semihosting

This example uses `cortex-m-semihosting` (`hprintln!`) instead of
defmt + RTT.  

Semihosting has no in-RAM ring buffer.  Each `hprintln!` call issues a
semihosting trap that probe-rs intercepts and prints to the terminal.  

## `memory.x` change

The canary functions reference a linker symbol `_stack_bottom` that is not
defined by cortex-m-rt's default `link.x`.  It is added in `memory.x`:

```text
_stack_bottom = __ebss;   /* first free word above .data + .bss */
```

`__ebss` is defined by cortex-m-rt after placing the `.bss` output section.

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
Unused stack before work:        4986 words (19944 bytes)
Unused stack after work_shallow: 4913 words (19652 bytes)
Unused stack after work_deep:    4779 words (19116 bytes)
Peak usage  work_shallow alone:  ~292 bytes
Peak usage  work_deep chain:     ~828 bytes
```

Reading the numbers:

| Measurement | Words consumed | Bytes |
|---|---|---|
| `work_shallow` frame | 73 | 292 |
| `work_deep` + nested `work_shallow` | 207 | 828 |

`work_shallow` allocates a 256-byte buffer; the remaining ~36 bytes are the
function call overhead (saved link register, alignment, frame pointer).

`work_deep` allocates a 512-byte buffer and calls `work_shallow` while that
buffer is still live.  

## Run  Path B: cargo embed (flash only — no output)

`cargo embed` is an RTT-centric tool and does not forward semihosting output
to the terminal.  The firmware will hang at the first `hprintln!` call
because the semihosting BKPT trap has no handler.  Use Path B only if you
want to flash the device and then attach a separate GDB session that has
semihosting enabled.

```sh
cargo embed --release   # flashes and resets; no output visible
```

Use **Path A** (`cargo run --release`) to see the canary measurements.

## Run  Path C: OpenOCD (flash only)

```sh
cargo build
./run_openocd.sh
```
