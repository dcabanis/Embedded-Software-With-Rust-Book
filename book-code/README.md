# Code Examples

Companion source code organized by chapter.
Each example lives inside a board-specific sub-folder; clone the repo and build only the variant that matches your hardware.

**PRO Tip:** Hardware boards can be temperamental at times. For one reason or another they stop responding, unwilling to be flashed. If this is something you are experiencing, you can try the following trick:

- Hold the board's *reset* button pressed
- Execute the terminal command (you can try a few times): st-flash --connect-under-reset erase
- Whilst the command execute, you can release the reset button and if the silicon gods are with you, that should set the board back into a state where it will allow you to flash it with your new firmware.

  
## Contents

- [Chapter 1 - Peripheral Singletons and Interrupts](#chapter-1---peripheral-singletons-and-interrupts)
- [Chapter 2 - Tooling, Templates, and Simulation](#chapter-2---tooling-templates-and-simulation)
- [Chapter 3 - Project Structure and the Embedded-Rust Ecosystem](#chapter-3---project-structure-and-the-embedded-rust-ecosystem)
- [Chapter 4 - Bare-Metal Startup, Linker Scripts, and Interrupt Handling](#chapter-4---bare-metal-startup-linker-scripts-and-interrupt-handling)
- [Chapter 5 - Hardware Abstraction: From Raw MMIO to HAL](#chapter-5---hardware-abstraction-from-raw-mmio-to-hal)
- [Chapter 6 - Debugging, Logging, and Profiling](#chapter-6---debugging-logging-and-profiling)
- [Chapter 7 - Managing Stack and Heap in Resource-Constrained Systems](#chapter-7---managing-stack-and-heap-in-resource-constrained-systems)
- [Chapter 10 - Rust-C Integration and Migration](#chapter-10---rust-c-integration-and-migration)

---

## Chapter 1 - Peripheral Singletons and Interrupts

### example_01 - The singleton pattern for peripherals

SysTick fires a 1-second interrupt at 8 MHz while the CPU parks in WFI; shows `cortex-m-rt` placing an exception handler in the vector table.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH01/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH01/example_01/NUCLEO_STM32F103RBT6)

---

## Chapter 2 - Tooling, Templates, and Simulation

### example_01 - Using a project template with cargo generate

The minimal "hello world" template. Prints once via semihosting, then loops forever. **Without an active debugger the firmware will hang at the semihosting call.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH02/example_01/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH02/example_01/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH02/example_01/NUCLEO_STM32F103RBT6)

### example_02 - Hello world with Knurling

A Knurling-based template for `defmt` structured logging over RTT, with separate binaries covering greetings, log levels, custom formatting, bitfields, panics, and stack overflow.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH02/example_02/knurling_bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH02/example_02/knurling_NUCLEO_STM32F103RBT6)

### example_03 - Using QEMU for running examples

Runs on QEMU (STM32F100, Cortex-M3) instead of physical hardware; a way to validate embedded Rust without a board on hand.

**Available on:**
[QEMU STM32F100](CH02/example_03/QEMU_STM32F100)

### example_04 - LED blink in PICSimLab simulation

LED blink targeting the PICSimLab simulator: 72 MHz clock, PC13 toggled every 500 ms via SysTick, no physical device needed.

**Available on:**
[PICSimLab (STM32F103C8T6)](CH02/example_04/PICSimLab_STM32F103C8T6)

---

## Chapter 3 - Project Structure and the Embedded-Rust Ecosystem

### example_01 - Typical embedded Rust project layout

The chapter-3 starting point: canonical `#![no_std]` / `#![no_main]` structure, peripheral singleton, and a HAL import. **A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_01/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH03/example_01/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_01/NUCLEO_STM32F103RBT6)

### example_02 - Conditional compile with features

Compile-time feature selection for the panic handler, switching between `panic-halt` and `panic-semihosting` via `Cargo.toml`.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_02/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_02/NUCLEO_STM32F103RBT6)

### example_03 - Using cortex-m crates together

`cortex-m` and `cortex-m-rt` together to configure and register a SysTick exception handler.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_03/NUCLEO_STM32F103RBT6)

### example_04 - embedded-hal used with the LSM303AGR accelerometer

Reads an LSM303AGR accelerometer over I2C via `embedded-hal`; the LED toggles when X-axis acceleration crosses a threshold.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_04/bluepill_STM32F103C8T6) ·
[STM32F3DISCOVERY (STM32F303)](CH03/example_04/Discovery_STM32F303) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_04/NUCLEO_STM32F103RBT6)

### example_05 - Platform-agnostic driver with embedded-hal

A generic `blink_led()` built on the `embedded-hal` `OutputPin` trait, with an `ActiveLow` wrapper handling the Blue Pill's inverted LED polarity.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_05/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH03/example_05/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_05/NUCLEO_STM32F103RBT6)

### example_06 - UART echo on USART1

Byte echo loop on USART1 at 115,200 baud. You'll need a USB-to-UART adapter and a terminal emulator on the host side.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_06/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_06/NUCLEO_STM32F103RBT6)

---

## Chapter 4 - Bare-Metal Startup, Linker Scripts, and Interrupt Handling

### example_01 - Minimal linker script and startup

MCU startup built from scratch, no `cortex-m-rt`: a hand-placed reset vector, manual `.data`/`.bss` init, and raw-pointer GPIO/SysTick setup.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_01/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH04/example_01/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_01/NUCLEO_STM32F103RBT6)

### example_02 - Manually creating a vector table and exception handlers

A hand-built Cortex-M exception vector table with handlers for NMI, HardFault, and friends; shows what `cortex-m-rt` normally does for you. **A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_02/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH04/example_02/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_02/NUCLEO_STM32F103RBT6)

### example_03 - Simplifying exception and interrupt handling with a run-time crate

The manual vector table from example_02, replaced by the `#[exception]` macro; the idiomatic `cortex-m-rt` way to register a handler. **A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_03/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH04/example_03/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_03/NUCLEO_STM32F103RBT6)

### example_04 - Safe shared access using `cortex_m::interrupt::Mutex`

A seconds counter shared between the main loop and a `SysTick` handler via `cortex_m::interrupt::Mutex<Cell<u32>>`, with all access inside `interrupt::free()`. **A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_04/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH04/example_04/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_04/NUCLEO_STM32F103RBT6)

### example_05 - Sharing peripherals with mutex and interior mutability

Extends the Mutex pattern to a whole peripheral: the `SYST` timer lives in a `static Mutex<RefCell<Option<SYST>>>`, borrowed by both the main loop and the handler. **A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_05/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH04/example_05/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_05/NUCLEO_STM32F103RBT6)

---

## Chapter 5 - Hardware Abstraction: From Raw MMIO to HAL

### example_01 - Raw-pointer MMIO LED blink

The chapter-5 baseline: an LED blinker written entirely with raw pointer MMIO, no HAL or PAC in sight.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_01/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH05/example_01/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_01/NUCLEO_STM32F103RBT6)

### example_02 - Type-safe MMIO via PAC

The same blink, this time through the `stm32f1` PAC's type-safe `modify()` / `write()` register API instead of raw pointers.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_02/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH05/example_02/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_02/NUCLEO_STM32F103RBT6)

### example_03 - 1 Hz LED blink using stm32f1xx-hal

The same blink again, now through `stm32f1xx-hal`; register-level detail disappears behind a typed pin and a HAL `Delay`.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_03/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH05/example_03/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_03/NUCLEO_STM32F103RBT6)

### example_04 - SSD1306 OLED display over I2C

Drives an SSD1306 OLED over I2C1 using the `ssd1306` crate, plus a small `PrintStr` trait that patches around one of its quirks.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_04/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_04/NUCLEO_STM32F103RBT6)

### example_05 - Inline and global assembly for MMIO and timing

Three ways to mix assembly into Rust: `global_asm!()`, inline `core::arch::asm!()`, and `cortex_m::asm::nop()` busy-waits.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_05/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH05/example_05/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_05/NUCLEO_STM32F103RBT6)

---

## Chapter 6 - Debugging, Logging, and Profiling

### example_01 - Semihosting with `hprintln!()`

Prints once via `hprintln!()`, then spins. No peripherals configured; the debugger handles I/O on the host side.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_01/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH06/example_01/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_01/NUCLEO_STM32F103RBT6)

### example_02 - UART logging with `writeln!()`

Writes a message over USART1 via `writeln!()`. No debugger needed; read it on any serial terminal.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_02/bluepill_STM32F103C8T6) ·
[PICSimLab (STM32F103C8T6)](CH06/example_02/PICSimLab_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_02/NUCLEO_STM32F103RBT6)

### example_03 - ITM logging with `iprintln!()`

Writes a message to an ITM stimulus port via `iprintln!()` from the `cortex-m` crate.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_03/NUCLEO_STM32F103RBT6)

### example_04 - RTT logging with `rtt-target`

Writes a message over RTT via `rprintln!()`; no extra hardware pins beyond the standard SWD connection.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_04/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_04/NUCLEO_STM32F103RBT6)

### example_05 - Structured logging with `defmt`

Logs once a second via `defmt::info!()`, the token-based binary logging framework that keeps firmware small.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_05/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_05/NUCLEO_STM32F103RBT6)

### example_06 - Configurable fault handlers and HardFault diagnosis

Configurable fault handlers plus a HardFault handler that decodes HFSR/CFSR and prints the faulting address, all logged over RTT; a fault is injected on every boot to demonstrate it.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_06/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_06/NUCLEO_STM32F103RBT6)

### example_07 - GDB + OpenOCD interactive debugging

The full GDB + OpenOCD workflow, with an automated `.gdbinit` that connects, flashes, and stops at `main`.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_07/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_07/NUCLEO_STM32F103RBT6)

### example_08 - DWT cycle-counter profiling

Cycle-accurate timing via the Cortex-M DWT unit, comparing two `nop`-loop tasks of different lengths over RTT.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_08/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_08/NUCLEO_STM32F103RBT6)

### example_09 - SysTick-based cycle measurement

SysTick-based timing as a DWT fallback for cores without a cycle counter (Cortex-M0/M0+); runs unchanged on the Blue Pill's Cortex-M3 too.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_09/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_09/NUCLEO_STM32F103RBT6)

---

## Chapter 7 - Managing Stack and Heap in Resource-Constrained Systems

### example_01 - Dynamic allocation with `embedded-alloc` (LLFF heap)

Installs `embedded-alloc`'s LLFF heap as the global allocator over a 4 KiB static buffer, then exercises `Box::new` and `Vec::push`; the minimum wiring to bring `alloc` online in `no_std`.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_01/NUCLEO_STM32F103RBT6)

### example_02 - Buddy-system allocator with `buddy_system_allocator`

The same `Box`/`Vec` workload from example_01, this time on a buddy-system allocator, for bounded worst-case timing at the cost of some rounding up.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_02/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_02/NUCLEO_STM32F103RBT6)

### example_03 - Heap-allocated event log with `Vec<String>`

Builds a runtime event log as a `Vec<String>`, logged via `defmt`; idiomatic `no_std` heap usage once the allocator is installed.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_03/NUCLEO_STM32F103RBT6)

### example_04 - Fixed-capacity collections with `heapless`

A UART byte-stream parser built entirely on `heapless::Vec` and `String`; no allocator, fixed capacity known at compile time.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_04/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_04/NUCLEO_STM32F103RBT6)

### example_05 - Stack-only collections with `arrayvec`

`ArrayVec` and `ArrayString` from the `arrayvec` crate, stack-only with no heap involved; both the panicking and the fallible `try_push` APIs are shown.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_05/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_05/NUCLEO_STM32F103RBT6)

### example_06 - Hybrid stack-or-heap collections with `tinyvec` and `smallvec`

`TinyVec` and `SmallVec` compared side by side: both start inline on the stack and spill to the heap once capacity runs out.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_06/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_06/NUCLEO_STM32F103RBT6)

### example_07 - Stack-overflow detection with `flip-link`

The `flip-link` linker wrapper flips the RAM layout so a stack overflow hits unmapped memory and faults cleanly, instead of silently corrupting `.data`.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_07/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_07/NUCLEO_STM32F103RBT6)

### example_08 - Stack-usage measurement with canary painting

Measures peak stack usage without an RTOS or debugger by painting free RAM with a canary pattern and counting what survives.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_08/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_08/NUCLEO_STM32F103RBT6)

### example_09 - MPU stack guard on the STM32F3DISCOVERY

The Cortex-M4 MPU set up as a hardware stack guard, turning a silent overflow into a deterministic, debugger-visible fault.

**Available on:**
[STM32F3DISCOVERY (STM32F303VCT6)](CH07/example_09/stm32f3discovery_STM32F303VCT6)

---

## Chapter 10 - Rust-C Integration and Migration

### example_01 - Calling C from Rust: FFI basics and safe wrappers

Calls hand-written C code from Rust, wrapping the `unsafe extern "C"` calls behind ordinary safe functions; `build.rs` compiles and links the C file automatically.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH10/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH10/example_01/NUCLEO_STM32F103RBT6)

### example_02 - Calling Rust from C: a `no_std` static library

Flips the usual shape: a plain C application owns `main()` and the vector table, linking against a Rust `no_std` static library built separately.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH10/example_02/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH10/example_02/NUCLEO_STM32F103RBT6)

### example_03 - Advanced FFI: arrays, C strings, and stateful callbacks

The harder FFI cases: passing slices as pointer + length, `&CStr` instead of `&str`, and a context-carrying C callback pattern.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH10/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH10/example_03/NUCLEO_STM32F103RBT6)

### example_04 - bindgen: generating Rust bindings from a real C library

Uses *bindgen* to generate Rust FFI bindings from a real third-party C library ([zserge/jsmn](https://github.com/zserge/jsmn)), including a fix for an enum-sizing mismatch between `arm-none-eabi-gcc` and libclang.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH10/example_04/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH10/example_04/NUCLEO_STM32F103RBT6)

### example_05 - cbindgen: generating a C header from Rust

The inverse of example_04: *cbindgen* generates a C header automatically from a Rust `checksum()` function, regenerated on every build.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH10/example_05/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH10/example_05/NUCLEO_STM32F103RBT6)
