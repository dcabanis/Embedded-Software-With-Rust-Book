# List of commands

## Build 

cargo build

## flash and run

./run_openOCD.sh 

Alternatively:

cargo run

## Board configuration (NUCLEO-F103RB)

- probe-rs chip: `STM32F103RBTx`
- Linker memory: `FLASH = 128K`, `RAM = 20K`

## Clean 

cargo clean
