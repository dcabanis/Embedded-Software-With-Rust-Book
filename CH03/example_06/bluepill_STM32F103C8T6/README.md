# example_06  UART echo on USART1 (PA9/PA10) on the Blue Pill

This example configures USART1 at 115200 baud, 8N1 and implements a byte echo
loop using the `nb::block!()` macro for blocking I/O. Bytes received on PA10
(RX) are immediately re-transmitted on PA9 (TX). A USB-to-UART adapter and a
terminal emulator such as `picocom` are required on the host side to send and
see the echoed characters.

## UART pin wiring

| Signal | Pin  |
|--------|------|
| TX     | PA9  |
| RX     | PA10 |

## Build

```sh
cargo build
```

## Run

Open a serial terminal in a separate terminal window first:

```sh
picocom -b 115200 -d 8 -p 1 /dev/ttyUSB0
```

Then flash and run the firmware:

```sh
cargo run
```

## Expected Output

Characters typed in picocom are echoed back by the firmware.

## Exiting picocom

```sh
Ctrl+a Ctrl+x
```
