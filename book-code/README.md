# Code Examples

Companion source code organised by chapter.
Each example lives inside a board-specific sub-folder — clone the repo and build only the variant that matches your hardware.

## Contents

- [Chapter 1 — Peripheral Singletons and Interrupts](#chapter-1--peripheral-singletons-and-interrupts)
- [Chapter 2 — Tooling, Templates, and Simulation](#chapter-2--tooling-templates-and-simulation)
- [Chapter 3 — Project Structure and the Embedded-Rust Ecosystem](#chapter-3--project-structure-and-the-embedded-rust-ecosystem)
- [Chapter 4 — Bare-Metal Startup, Linker Scripts, and Interrupt Handling](#chapter-4--bare-metal-startup-linker-scripts-and-interrupt-handling)
- [Chapter 5 — Hardware Abstraction: From Raw MMIO to HAL](#chapter-5--hardware-abstraction-from-raw-mmio-to-hal)
- [Chapter 6 — Debugging, Logging, and Profiling](#chapter-6--debugging-logging-and-profiling)
- [Chapter 7 — Managing Stack and Heap in Resource-Constrained Systems](#chapter-7--managing-stack-and-heap-in-resource-constrained-systems)

---

## Chapter 1 — Peripheral Singletons and Interrupts

### example_01 — The singleton pattern for peripherals

This example configures the SysTick timer for a 1-second periodic interrupt at 8 MHz, and parks the CPU in WFI in the main loop. A `#[exception]`-annotated `SysTick` handler fires on each tick, illustrating how `cortex-m-rt` places exception handlers in the correct vector table slots.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH01/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH01/example_01/NUCLEO_STM32F103RBT6)

---

## Chapter 2 — Tooling, Templates, and Simulation

### example_01 — Using a project template with cargo generate

This is the minimal "hello world" template for an STM32F103 project. The firmware prints `"Fly like a bird!"` once via the `hprintln!()` semihosting macro and then loops forever. No peripherals are configured; all I/O is handled by the attached debugger on the host side. **Without an active debugger the firmware will hang at the semihosting call.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH02/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH02/example_01/NUCLEO_STM32F103RBT6)

### example_02 — Hello world with Knurling

This is a Knurling-based project template that demonstrates `defmt` structured logging over RTT. Multiple binary targets are provided: `hello` prints a greeting, `levels` shows all defmt log levels controlled by the `DEFMT_LOG` environment variable, `format` demonstrates the `Format` derive macro for custom struct formatting, `bitfield` shows bitfield extraction syntax for reading register fields, `panic` triggers a defmt panic, and `overflow` exhausts the stack with a recursive Ackermann function to demonstrate stack-overflow detection.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH02/example_02/knurling_bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH02/example_02/knurling_NUCLEO_STM32F103RBT6)

### example_03 — Using QEMU for running examples

This example runs on QEMU instead of physical hardware, targeting the STM32F100 (Cortex-M3) chip model. The firmware prints `"Fly like a bird!"` via semihosting, then enters an infinite loop. It demonstrates how to validate embedded Rust code on a host machine without a development board, using QEMU's ARM system emulation as the execution environment.

**Available on:**
[QEMU STM32F100](CH02/example_03/QEMU_STM32F100)

### example_04 — LED blink in PICSimLab simulation

This example targets the PICSimLab simulator rather than physical hardware. The firmware configures the STM32F103C8T6 system clock to 72 MHz via the HSE PLL, initialises GPIO PC13 as a push-pull output, and toggles the LED every 500 ms using a SysTick-based delay. It is intended to be run inside PICSimLab's Blue Pill board workspace, demonstrating how to develop and test firmware without a physical device.

**Available on:**
[PICSimLab (STM32F103C8T6)](CH02/example_04/PICSimLab_STM32F103C8T6)

---

## Chapter 3 — Project Structure and the Embedded-Rust Ecosystem

### example_01 — Typical embedded Rust project layout

This is the chapter-3 starting-point project. The firmware takes STM32 device peripherals via `stm32f1xx_hal::pac::Peripherals::take()` and prints `"Hello, world!"` through semihosting. The purpose is to show the canonical embedded-Rust project structure, `#![no_std]` / `#![no_main]`, runtime entry point, peripheral singleton, and a HAL import, before adding any real hardware interaction. **A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_01/NUCLEO_STM32F103RBT6)

### example_02 — Conditional compile with features

This example demonstrates compile-time feature selection for the panic handler. Two mutually exclusive features are defined in `Cargo.toml`: the default `panic-halt` feature links `panic-halt`, and the `semihosting` feature links `panic-semihosting` plus `cortex-m-semihosting`. A `#[cfg]` compile-time check enforces that exactly one feature is active. The firmware then triggers a panic
so the chosen strategy can be observed in practice.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_02/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_02/NUCLEO_STM32F103RBT6)

### example_03 — Using cortex-m crates together

This example uses both the `cortex-m` and `cortex-m-rt` crates together to configure SysTick and register its exception handler. The firmware programs SysTick for a 1-second reload interval at 8 MHz.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_03/NUCLEO_STM32F103RBT6)

### example_04 — embedded-hal used with the LSM303AGR accelerometer

This example demonstrates platform-agnostic hardware driver usage via the `embedded-hal` I2C trait. The firmware configures I2C1, initialises an LSM303AGR accelerometer in Normal mode at 50 Hz ODR using the `lsm303agr` driver crate, and toggles the onboard LED on PC13 whenever the X-axis acceleration exceeds a threshold. It shows that the same high-level driver works across STM32 families because it depends only on the `embedded-hal` I2C abstraction.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_04/bluepill_STM32F103C8T6) ·
[STM32F3DISCOVERY (STM32F303)](CH03/example_04/Discovery_STM32F303) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_04/NUCLEO_STM32F103RBT6)

### example_05 — Platform-agnostic driver with embedded-hal

This example demonstrates writing MCU-agnostic driver logic using the `embedded-hal` `OutputPin` trait. A generic `blink_led()` function accepts any `OutputPin` and toggles it in a loop. Because the Blue Pill's onboard LED on PC13 is active-low, an `ActiveLow` new-type wrapper is introduced to invert the pin polarity, letting the generic function operate correctly without knowing about board-specific wiring.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_05/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_05/NUCLEO_STM32F103RBT6)

### example_06 — UART echo on USART1

This example configures USART1 at 115 200 baud, 8N1 and implements a byte echo loop using the `nb::block!()` macro for blocking I/O. Bytes received on PA10 (RX) are immediately re-transmitted on PA9 (TX). A USB-to-UART adapter and a terminal emulator such as `picocom` are required on the host side to send and see the echoed characters.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_06/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_06/NUCLEO_STM32F103RBT6)

---

## Chapter 4 — Bare-Metal Startup, Linker Scripts, and Interrupt Handling

### example_01 — Minimal linker script and startup

This example implements the entire MCU startup from scratch, without the `cortex-m-rt` crate. It manually places a reset vector in the `.vector_table` linker section, initialises the `.data` and `.bss` segments in inline assembly, then configures GPIO and SysTick by writing directly to memory-mapped I/O addresses via raw pointers. The onboard LED on PC13 (active-low) toggles once per SysTick tick, revealing what the runtime crate does automatically under the hood.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_01/NUCLEO_STM32F103RBT6)

### example_02 — Manually creating a vector table and exception handlers

This example extends the bare-metal startup from example_01 by manually constructing the full Cortex-M exception vector table in a `startup` module.
Handlers for NMI, HardFault, MemManage, BusFault, UsageFault, SVCall, and PendSV are provided; each prints a diagnostic message via semihosting before halting. The example shows what `cortex-m-rt` generates automatically, making the underlying mechanism visible. **A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_02/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_02/NUCLEO_STM32F103RBT6)

### example_03 — Simplifying exception and interrupt handling with a run-time crate

This example replaces the manual vector table from example_02 with the `#[exception]` macro provided by `cortex-m-rt`. The SysTick timer fires every second and flips an `AtomicBool` flag. The main loop polls the flag in WFI sleep and prints an alternating message via semihosting, demonstrating the idiomatic `cortex-m-rt` approach to safe exception handler registration. **A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_03/NUCLEO_STM32F103RBT6)

### example_04 — Safe shared access using `cortex_m::interrupt::Mutex`

This example demonstrates how to safely share a variable between the main execution context and an interrupt handler using `cortex_m::interrupt::Mutex<Cell<u32>>`. A monotonic seconds counter is stored in a `static` and incremented by the `SysTick` handler every second. All accesses go through `interrupt::free()` critical sections, preventing data races on Cortex-M. The current count is printed via semihosting on each tick.
**A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_04/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_04/NUCLEO_STM32F103RBT6)

### example_05 — Sharing peripherals with mutex and interior mutability

This example extends the Mutex pattern to a full peripheral by moving the `SYST` timer into a `static Mutex<RefCell<Option<SYST>>>` after configuration. Both the main loop and the `SysTick` exception handler borrow it inside `interrupt::free()` critical sections. The handler reads the current value register (CVR) and prints it via semihosting; the main loop detects 30-second
intervals and prints elapsed time.
**A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_05/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_05/NUCLEO_STM32F103RBT6)

---

## Chapter 5 — Hardware Abstraction: From Raw MMIO to HAL

### example_01 — Raw-pointer MMIO LED blink

This is the chapter-5 baseline: a bare-metal LED blinker written entirely with raw pointer MMIO, using no HAL or PAC. The firmware initialises data and BSS sections, enables GPIOC via RCC, and configures PC13 as a push-pull output.
SysTick is polled (not interrupt-driven) to produce a 1-second toggle period.
The goal is to establish a reference point before showing how PAC and HAL crates simplify the same task in the subsequent examples.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_01/NUCLEO_STM32F103RBT6)

### example_02 — Type-safe MMIO via PAC

This example replaces the raw `*mut u32` writes from example_01 with the type-safe register API of the `stm32f1` Peripheral Access Crate (PAC). GPIO and RCC are configured through the PAC's `modify()` and `write()` closures, eliminating raw pointer casts while producing the same result: PC13 (active-low) toggled every second via SysTick.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_02/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_02/NUCLEO_STM32F103RBT6)

### example_03 — 1 Hz LED blink using stm32f1xx-hal

This example demonstrates the top HAL layer of the embedded-Rust stack. The `stm32f1xx-hal` crate configures the RCC, takes ownership of the GPIOC peripheral, and returns a typed push-pull output pin for PC13 (active-low). A HAL `Delay` provider built from SysTick abstracts the timing. The result is a concise, readable 1 Hz blink loop that hides all register-level detail, showing the contrast with example_01 (raw pointers) and example_02 (PAC).

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_03/NUCLEO_STM32F103RBT6)

### example_04 — SSD1306 OLED display over I2C

This example drives an SSD1306 OLED display using the `ssd1306` platform-agnostic driver crate over I2C1. The firmware probes the display, initialises it in terminal mode, and writes `"SSD1306 found!"` and `"Hello world!"` to the screen.
I2C1 is configured on PB6 (SCL) and PB7 (SDA). The example also introduces a custom `PrintStr` trait to work around a defect in the `ssd1306` crate's `fmt::Write` implementation, demonstrating how to patch third-party driver limitations without forking.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_04/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_04/NUCLEO_STM32F103RBT6)

### example_05 — Inline and global assembly for MMIO and timing

This example demonstrates three ways to embed assembly in embedded-Rust firmware.
A `global_asm!()` block defines a counted delay loop. `core::arch::asm!()` is used inline to manipulate `PRIMASK` (disabling interrupts) and to toggle the PC13 LED by writing directly to the GPIOC BSRR register. `cortex_m::asm::nop()` provides short NOP-based busy-wait delays. The example shows how to mix Rust and assembly for performance-critical or hardware-specific code paths while still leveraging the HAL for initial GPIO configuration.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_05/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_05/NUCLEO_STM32F103RBT6)

---

## Chapter 6 — Debugging, Logging, and Profiling

### example_01 — Semihosting with `hprintln!()`

This example demonstrates semihosting output on the STM32F103. The firmware prints `"Hello from semihosting"` once via the `hprintln!()` macro from the `cortex-m-semihosting` crate, then spins in an infinite loop. No peripherals are configured; all I/O is handled by the debugger on the host side.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_01/NUCLEO_STM32F103RBT6)

### example_02 — UART logging with `writeln!()`

This example demonstrates standalone UART output. The firmware configures USART1 on PA9 (TX) and PA10 (RX) at 115 200 baud, writes `"UART log message"` once via the `writeln!()` macro, then spins in an infinite loop. No debugger is required after flashing; the message appears on any serial terminal connected to the UART pins.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_02/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_02/NUCLEO_STM32F103RBT6)

### example_03 — ITM logging with `iprintln!()`

This example demonstrates ITM (Instrumentation Trace Macrocell) output. The firmware writes `"ITM log message"` to ITM stimulus port 0 via the `iprintln!()` macro from the `cortex-m` crate, then spins in an infinite loop.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_03/NUCLEO_STM32F103RBT6)

### example_04 — RTT logging with `rtt-target`

This example demonstrates Real-Time Transfer (RTT) output. The firmware initialises an RTT channel with `rtt_init_print!()`, writes `"RTT log message"` via `rprintln!()`, then spins in an infinite loop. RTT works by placing a control block in RAM that the debug probe reads in the background over SWD while the firmware runs — no extra hardware pins are required beyond the standard SWD connection (SWDIO, SWCLK, GND) already used for flashing.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_04/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_04/NUCLEO_STM32F103RBT6)

### example_05 — Structured logging with `defmt`

This example demonstrates `defmt` output. The firmware logs one message at the `INFO` level via `defmt::info!()` once per second, repeating indefinitely.
`defmt` is a binary logging framework: instead of formatting strings on the device, it emits a compact token that references a format string stored in the ELF binary. The host tool reconstructs the message by reading both the token stream and the ELF symbol table, making each log call cheaper and keeping the binary smaller than a `core::fmt`-based approach.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_05/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_05/NUCLEO_STM32F103RBT6)

### example_06 — Configurable fault handlers and HardFault diagnosis

This example demonstrates three complementary fault-handling techniques. Configurable fault handlers (`MemoryManagement`, `BusFault`, `UsageFault`) each route to their own `#[exception]` handler, which calls `panic!()` so the panic handler can log the fault via RTT. A custom `#panic_handler]` routes panic
messages through RTT so they appear in the probe-rs console. A HardFault handler with full register decode prints the CPU register snapshot from `ExceptionFrame`, then reads and decodes HFSR and CFSR (including the UFSR, BFSR, and MMFSR sub-fields), and prints the faulting address from MMFAR or BFAR when the  address-valid bits are set. A deliberate fault is injected on every boot to drive the demonstration.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_06/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_06/NUCLEO_STM32F103RBT6)

### example_07 — GDB + OpenOCD interactive debugging

This example demonstrates the full GDB + OpenOCD debugging workflow, including an automated `.gdbinit` that connects to the target, flashes the binary, and stops at `main` ready for interactive use.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_07/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_07/NUCLEO_STM32F103RBT6)

### example_08 — DWT cycle-counter profiling

This example demonstrates cycle-accurate timing measurement using the Cortex-M Data Watchpoint and Trace (DWT) unit. The firmware enables the DWT cycle counter, times two `nop`-loop tasks of different lengths, and reports the elapsed cycles over RTT.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_08/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_08/NUCLEO_STM32F103RBT6)

### example_09 — SysTick-based cycle measurement

This example demonstrates SysTick-based timing as a fallback for Cortex-M cores that lack a DWT cycle counter (Cortex-M0 / M0+). The firmware configures SysTick as a free-running down-counter, measures `do_work()` by reading the counter before and after, and reports the elapsed cycles over RTT. The Blue Pill is a Cortex-M3 and does have DWT, so the DWT approach from example_08 is preferable on this hardware; this example demonstrates the SysTick technique that would be used on Cortex-M0 / M0+ / M23 parts, and the same code runs unchanged on a Cortex-M3.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_09/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_09/NUCLEO_STM32F103RBT6)

---

## Chapter 7 — Managing Stack and Heap in Resource-Constrained Systems

### example_01 — Dynamic allocation with `embedded-alloc` (LLFF heap)

This example enables the `alloc` crate in a `no_std` firmware by installing the `embedded-alloc` LLFF (Last-Level First-Fit) heap as the global allocator. A 4 KiB backing store is carved out of RAM using a `static mut [MaybeUninit<u8>]` array and handed to the allocator before any allocation is attempted. The firmware then exercises `Box::new` (allocates a `u32` on the heap, logs its value, and drops it back) and `Vec::push` (builds a three-element vector), both logged via `defmt`. The example shows the minimum wiring needed to bring the full `alloc` API online in a bare-metal Rust project.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_01/NUCLEO_STM32F103RBT6)

### example_02 — Buddy-system allocator with `buddy_system_allocator`

This example replaces the LLFF heap from example_01 with a buddy-system allocator from the `buddy_system_allocator` crate. The allocator is parameterised by an ORDER constant (32) that sets the maximum block size to 2^32 bytes; allocations are rounded up to the nearest power of two in exchange for bounded worst-case timing and predictable fragmentation behaviour. The same
`Box::new` and `Vec::push` workload from example_01 is repeated so the two allocator strategies can be compared directly.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_02/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_02/NUCLEO_STM32F103RBT6)

### example_03 — Heap-allocated event log with `Vec<String>`

This example demonstrates a practical use of the heap: accumulating a runtime event log as a `Vec<String>`. A `record_event()` helper formats each entry with `format!()` and appends it to a heap-allocated vector of heap-allocated strings. 

Three events are logged (UART, I2C, GPIO), and the completed log is printed via `defmt`. The example illustrates that idiomatic heap usage in `no_std` firmware looks exactly like standard Rust once the global allocator is installed.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_03/NUCLEO_STM32F103RBT6)

### example_04 — Fixed-capacity collections with `heapless`

This example demonstrates heap-free, fixed-capacity data structures from the `heapless` crate as a drop-in alternative to `Vec` and `String`. A `Vec<u8,LINE_LEN>` accumulates incoming bytes and a `String<RESP_LEN>` holds the response — both sized entirely at compile time with no allocator required. A `feed_byte()` function processes a simulated UART byte stream: it dispatches `HELLO` and `VERSION` commands to `handle_command()`, returns `ERR unknown` for unrecognised input, and detects buffer overflow when a line exceeds `LINE_LEN` bytes without a newline. All responses are logged via `defmt`.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_04/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_04/NUCLEO_STM32F103RBT6)

### example_05 — Stack-only collections with `arrayvec`

This example demonstrates the `arrayvec` crate's `ArrayVec<T, N>` and `ArrayString<N>` types, which store their elements entirely on the stack with no heap involvement. Both panicking and fallible APIs are shown: `push` / `push_str` panic on overflow (suitable for code paths that are statically known to be in-bounds), while `try_push` / `try_push_str` return a `Result` for input-driven paths where overflow is a real possibility. A `try_push_str` that would exceed the 32-byte `ArrayString` capacity is demonstrated, showing that the string is left unchanged after a failed attempt.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_05/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_05/NUCLEO_STM32F103RBT6)

### example_06 — Hybrid stack-or-heap collections with `tinyvec` and `smallvec`

This example demonstrates two hybrid collection crates that store a fixed number of elements inline (on the stack) and spill to the heap only when that capacity is exceeded. `TinyVec<[T; N]>` requires `T: Default` and is fully safe;
`SmallVec<[T; N]>` uses `unsafe` internally to remove the `Default` requirement.
Both are initialised with 8 inline elements (no heap), and a ninth push triggers the spill in each case. The `is_heap()` / `spilled()` methods are logged before and after the spill to make the transition visible. A global `embedded-alloc` heap is installed to service the spill allocations.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_06/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_06/NUCLEO_STM32F103RBT6)

### example_07 — Stack-overflow detection with `flip-link`

This example demonstrates the `flip-link` linker wrapper, which rearranges the RAM layout so the stack occupies the bottom of RAM rather than the top. In the default layout a stack overflow silently overwrites `.data` and `.bss`; with `flip-link` the overflow hits unmapped memory first, generating a deterministic bus fault. The firmware starts infinite recursion (512 bytes per frame via a
`MaybeUninit` buffer and a volatile write), logging the current depth via `defmt` on every frame. A `static mut DEVICE_ID` is placed in `.data` to make the corruption contrast visible: without `flip-link` the RTT stream cuts off early and `DEVICE_ID` is overwritten; with `flip-link` the log continues cleanly until the bus fault halts execution at a deterministic address.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_07/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_07/NUCLEO_STM32F103RBT6)

### example_08 — Stack-usage measurement with canary painting

This example demonstrates a manual stack-painting technique for measuring peak stack usage without an RTOS or a debugger. `paint_stack()` fills free RAM from `_stack_bottom` up to just below the current stack pointer with a 0xDEADBEEF canary pattern. After calling `work_shallow()` (256-byte frame) and `work_deep()` (512-byte frame that calls `work_shallow()`), `unused_stack_words()` scans from the bottom upward and counts surviving canary words. The difference gives the peak stack consumption of each call chain. Output is sent via semihosting (`hprintln!`) so no RTT ring buffer occupies the unpainted region and skews the measurement. `#[inline(never)]` is applied to both work functions to prevent the optimiser from merging their stack frames.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH07/example_08/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH07/example_08/NUCLEO_STM32F103RBT6)

### example_09 — MPU stack guard on the STM32F3DISCOVERY

This example configures the Cortex-M4 Memory Protection Unit (MPU) as a hardware stack guard on the STM32F303VCT6. A 32-byte no-access execute-never region (MPU region 0) is installed at `_stack_bottom`, the lowest valid stack address defined in `memory.x`. Any read, write, or instruction fetch within that region triggers a MemManage fault (or HardFault if MemManage is not separately
enabled), converting a silent stack overflow into a deterministic, debugger-visible fault. The MPU is enabled with `PRIVDEFENA` so all other memory regions retain their default access permissions. DSB and ISB barriers ensure the configuration takes effect before the next memory access. The STM32F3DISCOVERY is used here because its Cortex-M4 core includes the MPU; the Blue Pill's
Cortex-M3 also has an MPU but this example targets the Discovery board.

**Available on:**
[STM32F3DISCOVERY (STM32F303VCT6)](CH07/example_09/stm32f3discovery_STM32F303VCT6)
