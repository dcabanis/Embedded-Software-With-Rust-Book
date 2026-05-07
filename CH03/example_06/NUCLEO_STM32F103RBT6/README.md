# example_06  UART echo on USART2 via ST-LINK VCP on the NUCLEO-F103RB

This example configures USART2 at 115200 baud, 8N1 and implements a byte echo
loop using the `nb::block!()` macro for blocking I/O. On the NUCLEO-F103RB,
USART2 (PA2 TX / PA3 RX) is routed through the on-board ST-LINK to a virtual
COM port (VCP) that appears as `/dev/ttyACM0` on Linux. No external USB-to-UART
adapter is needed. Characters typed in a serial terminal are echoed back by the
firmware.

## UART pin wiring

| Signal | Pin | Note                   |
|--------|-----|------------------------|
| TX     | PA2 | Routed to ST-LINK VCP  |
| RX     | PA3 | Routed to ST-LINK VCP  |

## Build

```sh
cargo build
```

## Run

Open a serial terminal in a separate terminal window first:

```sh
picocom -b 115200 -d 8 -p 1 /dev/ttyACM0
```

Then flash and run the firmware:

```sh
cargo run
```

Characters typed in picocom are echoed back by the firmware.
