target extended-remote :3333

# Demangle Rust symbols in disassembly output
set print asm-demangle on

# Prevent runaway backtraces, particularly inside fault handlers
set backtrace limit 32

# Break when an unhandled exception or interrupt fires
break DefaultHandler

# Break on HardFault (effective when the user overrides cortex-m-rt's default handler)
break HardFault

# Break on every Rust panic, regardless of which #[panic_handler] crate is in use
break rust_begin_unwind

# Break at program entry
break main

# Enable semihosting trap interception in OpenOCD
monitor arm semihosting enable

# Optional: ITM tracing — uncomment and set <CPU_CLOCK_HZ> to the target's
# actual core clock at the moment trace is captured. The value below is a
# placeholder. Note: ST-Link clones do not support SWO; a J-Link or
# ST-Link/V2-1 with SWO-capable firmware is required.
# monitor tpiu config internal itm.txt uart off <CPU_CLOCK_HZ>
# monitor itm port 0 on

# Flash the binary and run to the first breakpoint
load
continue
