TARGET  := thumbv7m-none-eabi
LIBDIR  := rust_lib/target/$(TARGET)/release
LIB     := $(LIBDIR)/librust_lib.a

CC      := arm-none-eabi-gcc
OBJCOPY := arm-none-eabi-objcopy
SIZE    := arm-none-eabi-size

CFLAGS  := -mcpu=cortex-m3 -mthumb -Wall -Wextra -O2 -ffreestanding -std=c11 \
           -ffunction-sections -fdata-sections -Wa,--noexecstack
LDFLAGS := -T stm32f103.ld -nostartfiles -nostdlib -Wl,--gc-sections \
           -L$(LIBDIR) -lrust_lib

.PHONY: all rust_lib clean flash

all: app.bin app.elf
	$(SIZE) app.elf

rust_lib:
	cd rust_lib && cargo build --release --target $(TARGET)

$(LIB): rust_lib

app.elf: main.c startup.s stm32f103.ld $(LIB)
	$(CC) $(CFLAGS) main.c startup.s $(LDFLAGS) -o app.elf

app.bin: app.elf
	$(OBJCOPY) -O binary app.elf app.bin

flash: app.elf
	./run_openocd.sh

clean:
	rm -f app.elf app.bin
	cd rust_lib && cargo clean
