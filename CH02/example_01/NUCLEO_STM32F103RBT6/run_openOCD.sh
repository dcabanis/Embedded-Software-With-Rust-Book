#!/usr/bin/env bash
openocd \
  -f interface/stlink.cfg \
  -f target/stm32f1x.cfg \
  -c "transport select hla_swd" \
  -c "init" \
  -c "halt" \
  -c "arm semihosting enable" \
  -c "program target/thumbv7m-none-eabi/release/NUCLEO-blinky verify" \
  -c "reset run"
