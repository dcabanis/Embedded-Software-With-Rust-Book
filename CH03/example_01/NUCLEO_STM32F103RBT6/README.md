# List of commands

## Build 

cargo build

## Start the on-chip debugger (separate terminal)

openocd -f interface/stlink.cfg -f target/stm32f1x.cfg -c "init" -c "halt" -c "arm semihosting enable"

## Flash and debug

cargo run

## GDB commands 

step

next

continue

Ctrl+c

quit

## Clean 

cargo clean
