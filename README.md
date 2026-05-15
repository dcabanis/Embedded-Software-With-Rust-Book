# Code Examples

Companion source code organised by chapter.
Each example lives inside a board-specific sub-folder — clone the repo and
build only the variant that matches your hardware.

## Contents

- [Chapter 1 — Peripheral Singletons and Interrupts](#chapter-1--peripheral-singletons-and-interrupts)
- [Chapter 2 — Tooling, Templates, and Simulation](#chapter-2--tooling-templates-and-simulation)
- [Chapter 3 — Project Structure and the Embedded-Rust Ecosystem](#chapter-3--project-structure-and-the-embedded-rust-ecosystem)
- [Chapter 4 — Bare-Metal Startup, Linker Scripts, and Interrupt Handling](#chapter-4--bare-metal-startup-linker-scripts-and-interrupt-handling)
- [Chapter 5 — Hardware Abstraction: From Raw MMIO to HAL](#chapter-5--hardware-abstraction-from-raw-mmio-to-hal)
- [Chapter 6 — Debugging, Logging, and Profiling](#chapter-6--debugging-logging-and-profiling)

---

## Chapter 1 — Peripheral Singletons and Interrupts

### example_01 — The singleton pattern for peripherals

This example demonstrates the Cortex-M singleton pattern for peripheral access.
The firmware acquires ownership of `CorePeripherals` via
`cortex_m::Peripherals::take()`, configures the SysTick timer for a 1-second
periodic interrupt at 8 MHz, and parks the CPU in WFI in the main loop. A
`#[exception]`-annotated `SysTick` handler fires on each tick, illustrating how
`cortex-m-rt` places exception handlers in the correct vector table slots.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH01/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH01/example_01/NUCLEO_STM32F103RBT6)

---

## Chapter 2 — Tooling, Templates, and Simulation

### example_01 — Using a project template with cargo generate

This is the minimal "hello world" template for an STM32F103 project. The
firmware prints `"Fly like a bird!"` once via the `hprintln!()` semihosting
macro and then loops forever. No peripherals are configured; all I/O is handled
by the attached debugger on the host side. **Without an active debugger the
firmware will hang at the semihosting call.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH02/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH02/example_01/NUCLEO_STM32F103RBT6)

### example_02 — Hello world with Knurling

This is a Knurling-based project template that demonstrates `defmt` structured
logging over RTT. Multiple binary targets are provided: `hello` prints a
greeting, `levels` shows all defmt log levels controlled by the `DEFMT_LOG`
environment variable, `format` demonstrates the `Format` derive macro for custom
struct formatting, `bitfield` shows bitfield extraction syntax for reading
register fields, `panic` triggers a defmt panic, and `overflow` exhausts the
stack with a recursive Ackermann function to demonstrate stack-overflow
detection.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH02/example_02/knurling_bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH02/example_02/knurling_NUCLEO_STM32F103RBT6)

### example_03 — Using QEMU for running examples

This example runs on QEMU instead of physical hardware, targeting the STM32F100
(Cortex-M3) chip model. The firmware prints `"Fly like a bird!"` via
semihosting, then enters an infinite loop. It demonstrates how to validate
embedded Rust code on a host machine without a development board, using QEMU's
ARM system emulation as the execution environment.

**Available on:**
[QEMU STM32F100](CH02/example_03/QEMU_STM32F100)

### example_04 — LED blink in PICSimLab simulation

This example targets the PICSimLab simulator rather than physical hardware. The
firmware configures the STM32F103C8T6 system clock to 72 MHz via the HSE PLL,
initialises GPIO PC13 as a push-pull output, and toggles the LED every 500 ms
using a SysTick-based delay. It is intended to be run inside PICSimLab's Blue
Pill board workspace, demonstrating how to develop and test firmware without a
physical device.

**Available on:**
[PICSimLab (STM32F103C8T6)](CH02/example_04/PICSimLab_STM32F103C8T6)

---

## Chapter 3 — Project Structure and the Embedded-Rust Ecosystem

### example_01 — Typical embedded Rust project layout

This is the chapter-3 starting-point project. The firmware takes STM32 device
peripherals via `stm32f1xx_hal::pac::Peripherals::take()` and prints
`"Hello, world!"` through semihosting. The purpose is to show the canonical
embedded-Rust project structure, `#![no_std]` / `#![no_main]`, runtime entry
point, peripheral singleton, and a HAL import, before adding any real hardware
interaction. **A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_01/NUCLEO_STM32F103RBT6)

### example_02 — Conditional compile with features

This example demonstrates compile-time feature selection for the panic handler.
Two mutually exclusive features are defined in `Cargo.toml`: the default
`panic-halt` feature links `panic-halt`, and the `semihosting` feature links
`panic-semihosting` plus `cortex-m-semihosting`. A `#[cfg]` compile-time check
enforces that exactly one feature is active. The firmware then triggers a panic
so the chosen strategy can be observed in practice.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_02/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_02/NUCLEO_STM32F103RBT6)

### example_03 — Using cortex-m crates together

This example uses both the `cortex-m` and `cortex-m-rt` crates together to
configure SysTick and register its exception handler. The firmware programs
SysTick for a 1-second reload interval at 8 MHz.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_03/NUCLEO_STM32F103RBT6)

### example_04 — embedded-hal used with the LSM303AGR accelerometer

This example demonstrates platform-agnostic hardware driver usage via the
`embedded-hal` I2C trait. The firmware configures I2C1, initialises an LSM303AGR
accelerometer in Normal mode at 50 Hz ODR using the `lsm303agr` driver crate,
and toggles the onboard LED on PC13 whenever the X-axis acceleration exceeds a
threshold. It shows that the same high-level driver works across STM32 families
because it depends only on the `embedded-hal` I2C abstraction.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_04/bluepill_STM32F103C8T6) ·
[STM32F3DISCOVERY (STM32F303)](CH03/example_04/Discovery_STM32F303) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_04/NUCLEO_STM32F103RBT6)

### example_05 — Platform-agnostic driver with embedded-hal

This example demonstrates writing MCU-agnostic driver logic using the
`embedded-hal` `OutputPin` trait. A generic `blink_led()` function accepts any
`OutputPin` and toggles it in a loop. Because the Blue Pill's onboard LED on
PC13 is active-low, an `ActiveLow` new-type wrapper is introduced to invert the
pin polarity, letting the generic function operate correctly without knowing
about board-specific wiring.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_05/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_05/NUCLEO_STM32F103RBT6)

### example_06 — UART echo on USART1

This example configures USART1 at 115 200 baud, 8N1 and implements a byte echo
loop using the `nb::block!()` macro for blocking I/O. Bytes received on PA10
(RX) are immediately re-transmitted on PA9 (TX). A USB-to-UART adapter and a
terminal emulator such as `picocom` are required on the host side to send and
see the echoed characters.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH03/example_06/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH03/example_06/NUCLEO_STM32F103RBT6)

---

## Chapter 4 — Bare-Metal Startup, Linker Scripts, and Interrupt Handling

### example_01 — Minimal linker script and startup

This example implements the entire MCU startup from scratch, without the
`cortex-m-rt` crate. It manually places a reset vector in the `.vector_table`
linker section, initialises the `.data` and `.bss` segments in inline assembly,
then configures GPIO and SysTick by writing directly to memory-mapped I/O
addresses via raw pointers. The onboard LED on PC13 (active-low) toggles once
per SysTick tick, revealing what the runtime crate does automatically under the
hood.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_01/NUCLEO_STM32F103RBT6)

### example_02 — Manually creating a vector table and exception handlers

This example extends the bare-metal startup from example_01 by manually
constructing the full Cortex-M exception vector table in a `startup` module.
Handlers for NMI, HardFault, MemManage, BusFault, UsageFault, SVCall, and
PendSV are provided; each prints a diagnostic message via semihosting before
halting. The example shows what `cortex-m-rt` generates automatically, making
the underlying mechanism visible. **A debugger must be attached to see the
semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_02/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_02/NUCLEO_STM32F103RBT6)

### example_03 — Simplifying exception and interrupt handling with a run-time crate

This example replaces the manual vector table from example_02 with the
`#[exception]` macro provided by `cortex-m-rt`. The SysTick timer fires every
second and flips an `AtomicBool` flag. The main loop polls the flag in WFI sleep
and prints an alternating message via semihosting, demonstrating the idiomatic
`cortex-m-rt` approach to safe exception handler registration. **A debugger must
be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_03/NUCLEO_STM32F103RBT6)

### example_04 — Safe shared access using `cortex_m::interrupt::Mutex`

This example demonstrates how to safely share a variable between the main
execution context and an interrupt handler using
`cortex_m::interrupt::Mutex<Cell<u32>>`. A monotonic seconds counter is stored
in a `static` and incremented by the `SysTick` handler every second. All
accesses go through `interrupt::free()` critical sections, preventing data races
on Cortex-M. The current count is printed via semihosting on each tick.
**A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_04/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_04/NUCLEO_STM32F103RBT6)

### example_05 — Sharing peripherals with mutex and interior mutability

This example extends the Mutex pattern to a full peripheral by moving the
`SYST` timer into a `static Mutex<RefCell<Option<SYST>>>` after configuration.
Both the main loop and the `SysTick` exception handler borrow it inside
`interrupt::free()` critical sections. The handler reads the current value
register (CVR) and prints it via semihosting; the main loop detects 30-second
intervals and prints elapsed time.
**A debugger must be attached to see the semihosting output.**

**Available on:**
[Blue Pill (STM32F103C8T6)](CH04/example_05/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH04/example_05/NUCLEO_STM32F103RBT6)

---

## Chapter 5 — Hardware Abstraction: From Raw MMIO to HAL

### example_01 — Raw-pointer MMIO LED blink

This is the chapter-5 baseline: a bare-metal LED blinker written entirely with
raw pointer MMIO, using no HAL or PAC. The firmware initialises data and BSS
sections, enables GPIOC via RCC, and configures PC13 as a push-pull output.
SysTick is polled (not interrupt-driven) to produce a 1-second toggle period.
The goal is to establish a reference point before showing how PAC and HAL crates
simplify the same task in the subsequent examples.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_01/NUCLEO_STM32F103RBT6)

### example_02 — Type-safe MMIO via PAC

This example replaces the raw `*mut u32` writes from example_01 with the
type-safe register API of the `stm32f1` Peripheral Access Crate (PAC). GPIO and
RCC are configured through the PAC's `modify()` and `write()` closures,
eliminating raw pointer casts while producing the same result: PC13 (active-low)
toggled every second via SysTick.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_02/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_02/NUCLEO_STM32F103RBT6)

### example_03 — 1 Hz LED blink using stm32f1xx-hal

This example demonstrates the top HAL layer of the embedded-Rust stack. The
`stm32f1xx-hal` crate configures the RCC, takes ownership of the GPIOC
peripheral, and returns a typed push-pull output pin for PC13 (active-low). A
HAL `Delay` provider built from SysTick abstracts the timing. The result is a
concise, readable 1 Hz blink loop that hides all register-level detail, showing
the contrast with example_01 (raw pointers) and example_02 (PAC).

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_03/NUCLEO_STM32F103RBT6)

### example_04 — SSD1306 OLED display over I2C

This example drives an SSD1306 OLED display using the `ssd1306` platform-agnostic
driver crate over I2C1. The firmware probes the display, initialises it in
terminal mode, and writes `"SSD1306 found!"` and `"Hello world!"` to the screen.
I2C1 is configured on PB6 (SCL) and PB7 (SDA). The example also introduces a
custom `PrintStr` trait to work around a defect in the `ssd1306` crate's
`fmt::Write` implementation, demonstrating how to patch third-party driver
limitations without forking.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_04/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_04/NUCLEO_STM32F103RBT6)

### example_05 — Inline and global assembly for MMIO and timing

This example demonstrates three ways to embed assembly in embedded-Rust firmware.
A `global_asm!()` block defines a counted delay loop. `core::arch::asm!()` is
used inline to manipulate `PRIMASK` (disabling interrupts) and to toggle the
PC13 LED by writing directly to the GPIOC BSRR register. `cortex_m::asm::nop()`
provides short NOP-based busy-wait delays. The example shows how to mix Rust and
assembly for performance-critical or hardware-specific code paths while still
leveraging the HAL for initial GPIO configuration.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH05/example_05/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH05/example_05/NUCLEO_STM32F103RBT6)

---

## Chapter 6 — Debugging, Logging, and Profiling

### example_01 — Semihosting with `hprintln!()`

This example demonstrates semihosting output on the STM32F103. The firmware
prints `"Hello from semihosting"` once via the `hprintln!()` macro from the
`cortex-m-semihosting` crate, then spins in an infinite loop. No peripherals
are configured; all I/O is handled by the debugger on the host side.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_01/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_01/NUCLEO_STM32F103RBT6)

### example_02 — UART logging with `writeln!()`

This example demonstrates standalone UART output. The firmware configures
USART1 on PA9 (TX) and PA10 (RX) at 115 200 baud, writes `"UART log message"`
once via the `writeln!()` macro, then spins in an infinite loop. No debugger is
required after flashing; the message appears on any serial terminal connected to
the UART pins.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_02/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_02/NUCLEO_STM32F103RBT6)

### example_03 — ITM logging with `iprintln!()`

This example demonstrates ITM (Instrumentation Trace Macrocell) output. The
firmware writes `"ITM log message"` to ITM stimulus port 0 via the `iprintln!()`
macro from the `cortex-m` crate, then spins in an infinite loop.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_03/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_03/NUCLEO_STM32F103RBT6)

### example_04 — RTT logging with `rtt-target`

This example demonstrates Real-Time Transfer (RTT) output. The firmware
initialises an RTT channel with `rtt_init_print!()`, writes `"RTT log message"`
via `rprintln!()`, then spins in an infinite loop. RTT works by placing a
control block in RAM that the debug probe reads in the background over SWD while
the firmware runs — no extra hardware pins are required beyond the standard SWD
connection (SWDIO, SWCLK, GND) already used for flashing.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_04/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_04/NUCLEO_STM32F103RBT6)

### example_05 — Structured logging with `defmt`

This example demonstrates `defmt` output. The firmware logs one message at the
`INFO` level via `defmt::info!()` once per second, repeating indefinitely.
`defmt` is a binary logging framework: instead of formatting strings on the
device, it emits a compact token that references a format string stored in the
ELF binary. The host tool reconstructs the message by reading both the token
stream and the ELF symbol table, making each log call cheaper and keeping the
binary smaller than a `core::fmt`-based approach.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_05/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_05/NUCLEO_STM32F103RBT6)

### example_06 — Configurable fault handlers and HardFault diagnosis

This example demonstrates three complementary fault-handling techniques.
Configurable fault handlers (`MemoryManagement`, `BusFault`, `UsageFault`) each
route to their own `#[exception]` handler, which calls `panic!()` so the panic
handler can log the fault via RTT. A custom `#[panic_handler]` routes panic
messages through RTT so they appear in the probe-rs console. A HardFault handler
with full register decode prints the CPU register snapshot from `ExceptionFrame`,
then reads and decodes HFSR and CFSR (including the UFSR, BFSR, and MMFSR
sub-fields), and prints the faulting address from MMFAR or BFAR when the
address-valid bits are set. A deliberate fault is injected on every boot to
drive the demonstration.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_06/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_06/NUCLEO_STM32F103RBT6)

### example_07 — GDB + OpenOCD interactive debugging

This example demonstrates the full GDB + OpenOCD debugging workflow, including
an automated `.gdbinit` that connects to the target, flashes the binary, and
stops at `main` ready for interactive use.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_07/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_07/NUCLEO_STM32F103RBT6)

### example_08 — DWT cycle-counter profiling

This example demonstrates cycle-accurate timing measurement using the Cortex-M
Data Watchpoint and Trace (DWT) unit. The firmware enables the DWT cycle
counter, times two `nop`-loop tasks of different lengths, and reports the elapsed
cycles over RTT.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_08/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_08/NUCLEO_STM32F103RBT6)

### example_09 — SysTick-based cycle measurement

This example demonstrates SysTick-based timing as a fallback for Cortex-M cores
that lack a DWT cycle counter (Cortex-M0 / M0+). The firmware configures SysTick
as a free-running down-counter, measures `do_work()` by reading the counter
before and after, and reports the elapsed cycles over RTT. The Blue Pill is a
Cortex-M3 and does have DWT, so the DWT approach from example_08 is preferable
on this hardware; this example demonstrates the SysTick technique that would be
used on Cortex-M0 / M0+ / M23 parts, and the same code runs unchanged on a
Cortex-M3.

**Available on:**
[Blue Pill (STM32F103C8T6)](CH06/example_09/bluepill_STM32F103C8T6) ·
[NUCLEO-F103RB (STM32F103RBT6)](CH06/example_09/NUCLEO_STM32F103RBT6)
