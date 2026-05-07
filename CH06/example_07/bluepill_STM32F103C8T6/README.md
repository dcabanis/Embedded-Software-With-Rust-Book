# example_07 GDB + OpenOCD Interactive Debugging

This example demonstrates the full GDB + OpenOCD debugging workflow , including 
an automated `.gdbinit` that connects to the target, flashes the binary, and 
stops at `main` ready for interactive use. 

## Build

```sh
cargo build
```

## Run

This example is driven through GDB, not `cargo run`. Two terminals are
required.

**Terminal 1, start OpenOCD:**

```sh
./run_openocd.sh
```

OpenOCD connects to the ST-Link, flashes the binary, and listens on port
3333 for an incoming GDB connection. Leave this terminal open.

**Terminal 2, start GDB:**

```sh
./run_gdb.sh
```

This runs:

```sh
gdb-multiarch -q -x .gdbinit target/thumbv7m-none-eabi/debug/app
```

The `.gdbinit` file automatically:
1. Connects to OpenOCD on port 3333 (`target extended-remote :3333`)
2. Sets breakpoints on `main`, `HardFault`, `DefaultHandler`, and `rust_begin_unwind`
3. Flashes the binary (`load`)
4. Runs to the first breakpoint (`continue`)

GDB will stop at `main`. From there, exercise the session:

```
(gdb) info breakpoints
```

List all active breakpoints to confirm the `.gdbinit` setup.

```
(gdb) break app::slow_increment
(gdb) continue
```

Set a breakpoint on `slow_increment` then continue. GDB will stop at the
entry of that function. Rust symbols are stored with their full crate path
(`app::slow_increment`); the bare name `slow_increment` will not resolve.

```
(gdb) next
(gdb) next
(gdb) step
```

`next` steps over a line (staying in the current function); `step` steps
into a called function. Step through the increment loop to observe the
counter changing.

```
(gdb) info registers
(gdb) x /4xw 0x20000000
```

Inspect CPU registers and the first four words of SRAM.

```
(gdb) continue
```

Resume execution. The firmware will run `slow_increment` to completion,
print the result via RTT, and spin in a `nop` loop. Use `Ctrl+C` in GDB
to halt the CPU and regain the prompt.

```
(gdb) quit
```

Quit GDB. Then stop OpenOCD in Terminal 1 with `Ctrl+C`.

## `.gdbinit` and auto-load

GDB will not auto-load a project-local `.gdbinit` unless the directory is
listed in its `auto-load safe-path`. Running `gdb` from the project root
without `-x` produces a warning and silently ignores the file. `run_gdb.sh`
uses `-x .gdbinit` to bypass this restriction entirely.
