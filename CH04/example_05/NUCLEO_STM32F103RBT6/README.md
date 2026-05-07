# example_05  Sharing a peripheral with Mutex<RefCell<Option<>>> on the NUCLEO-F103RB

This example extends the Mutex pattern to a full peripheral by moving the
`SYST` timer into a `static Mutex<RefCell<Option<SYST>>>` after configuration. Both the main loop and the `SysTick` exception handler borrow it inside `interrupt::free()` critical sections. The handler reads the current value register (CVR) and prints it via semihosting; the main loop detects 30-second intervals and prints elapsed time. 
**A debugger must be attached to see the
semihosting output.**

## Build

```sh
cargo build
```

## Run  Path A: probe-rs

```sh
cargo run
```

## Run  Path B: OpenOCD

```sh
./run_openOCD.sh
```
