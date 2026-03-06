# List of commands

## Build 

cargo build --no-default-features --features semihosting

## flash and run

./run_openOCD.sh 

Alternatively (build flush and run with probe-rs):

cargo run --no-default-features --features semihosting

## Clean 

cargo clean
