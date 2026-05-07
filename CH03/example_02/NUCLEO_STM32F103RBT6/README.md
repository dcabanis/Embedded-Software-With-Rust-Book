# example_02  Cargo feature flags for panic strategy on the NUCLEO-F103RB

This example demonstrates compile-time feature selection for the panic handler.
Two mutually exclusive features are defined in `Cargo.toml`: the default
`panic-halt` feature links `panic-halt`, and the `semihosting` feature links
`panic-semihosting` plus `cortex-m-semihosting`. A `#[cfg]` compile-time check
enforces that exactly one feature is active. The firmware then triggers a panic
so the chosen strategy can be observed in practice.

## Build

Default build (halt on panic, no debugger needed):

```sh
cargo build
```

Build with semihosting panic (shows message via debugger):

```sh
cargo build --no-default-features --features semihosting
```

## Run — Path A: probe-rs

```sh
cargo run --no-default-features --features semihosting
```

## Run — Path B: OpenOCD

```sh
./run_openOCD.sh
```

## Expected output

```sh
Semihosting I/O is enabled.                                                                                          
panicked at src/main.rs:36:5:                                                                                        
This will halt (panic-halt) or print via semihosting (semihosting feature).                                          
Firmware exited unexpectedly: Breakpoint(Unknown)                                                                    
Core 0                                                                                                               
    Frame 0: __bkpt @ 0x08000f60 inline                                                                              
       ./asm/lib.rs:48:1                                                                                             
    Frame 1: __bkpt @ 0x0000000008000f60                                                                             
       ./asm/lib.rs:51:17                                                                                            
    Frame 2: panic @ 0x08000496                                                                                      
       /home/username/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/panic-semihosting-0.6.0/src/lib.rs:84:15  
```