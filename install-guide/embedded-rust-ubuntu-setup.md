# Embedded Rust Development Environment on Ubuntu

A reproducible setup guide for building, flashing, and debugging embedded Rust projects on Ubuntu (24.04 / 26.04 LTS). Targets ARM Cortex-M boards such as the STM32F103C8T6 ("Blue Pill"), with optional tooling for PIC simulation and broader microcontroller work.

Run each section in order. Where two installation methods are listed (e.g. apt and snap), pick whichever your distribution supports best, you don't need both.

---

## 1. Base system utilities

A handful of general-purpose tools that the rest of this guide depends on.

```bash
sudo apt update
sudo apt install build-essential curl git libssl-dev
```

- `build-essential` — C compiler and headers, required by some `cargo install` targets.
- `curl` — used to fetch the rustup installer and probe-rs installer.
- `git` — for cloning the book examples and other repositories.
- `libssl-dev` — required to build `cargo-generate` from source.

Optional editors and viewers:

```bash
sudo apt install neovim firefox
sudo snap install --classic code        # VS Code
sudo snap install marktext              # Markdown editor for the book
```

---

## 2. Rust toolchain

Install rustup with the official one-line installer, then load it into the current shell:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustup update
```

Add the ARM Cortex-M compilation targets used by common boards:

```bash
rustup target add thumbv6m-none-eabi        # Cortex-M0 / M0+
rustup target add thumbv7m-none-eabi        # Cortex-M3 (e.g. STM32F103 "Blue Pill")
rustup target add thumbv7em-none-eabihf     # Cortex-M4F / M7F (hardware float)
```

Install the LLVM tools and the Cargo wrappers used to inspect binaries (`cargo size`, `cargo objdump`, etc.):

```bash
rustup component add llvm-tools
cargo install cargo-binutils
```

Two more Cargo helpers that come up in the book examples:

```bash
cargo install cargo-generate    # project templates from git repos
cargo install flip-link         # stack-overflow protection linker for embedded
cargo install itm               # decode ITM trace data from a file
```

---

## 3. ARM cross toolchain

GCC, binutils, and a multi-architecture GDB for ARM bare-metal targets:

```bash
sudo apt install gcc-arm-none-eabi binutils-arm-none-eabi gdb-multiarch
```

`gdb-multiarch` is the GDB binary you point at the `target/thumbv7m-none-eabi/debug/<bin>` ELF when debugging.

Optional — drop in the popular gdb-dashboard configuration:

```bash
wget -P ~ https://github.com/cyrus-and/gdb-dashboard/raw/master/.gdbinit
```

QEMU is useful for running examples without hardware:

```bash
sudo apt install qemu-system-arm
```

---

## 4. Flashing and on-chip debugging

You need at least one of these — most workflows use both `probe-rs` (modern, one-shot run-and-print) and OpenOCD + GDB (interactive debugging).

### 4.1 ST-Link tools (for STM32 boards)

```bash
sudo apt install stlink-tools stlink-gui
```

If apt cannot find a working package, snap has one:

```bash
sudo snap install stlink
```

Verify the probe is detected after plugging in your board:

```bash
lsusb            # look for an STMicroelectronics ST-Link entry
st-flash --help
```

### 4.2 OpenOCD

```bash
sudo apt install openocd
```

A minimal session for a Blue Pill via ST-Link looks like:

```bash
openocd -f interface/stlink.cfg -f target/stm32f1x.cfg \
        -c "init" -c "halt" -c "arm semihosting enable"
```

### 4.3 probe-rs

The modern, all-in-one alternative to OpenOCD. Install via the upstream script:

```bash
curl -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

You can then `cargo run` a Rust binary and have it flashed and its `defmt` / RTT output streamed automatically (configured per project in `.cargo/config.toml`).

---

## 5. Serial terminals

For UART output from the target board pick whichever you prefer; `picocom` is the lightest:

```bash
sudo apt install picocom minicom screen putty
```

Typical Blue Pill UART connection:

```bash
picocom -b 115200 /dev/ttyACM0
# or, for a USB-to-serial adapter:
picocom -b 115200 /dev/ttyUSB0
```

If you get a permission error, add yourself to the `dialout` group and log out / back in:

```bash
sudo usermod -aG dialout $USER
```

---

## 6. PIC simulation (optional)

Only needed if you want to run PIC microcontroller examples in PICSimLab.

Install runtime dependencies first:

```bash
sudo apt install libfuse2t64 curl file desktop-file-utils
```

Then install the PICSimLab `.deb` package downloaded from the [PICSimLab releases page](https://github.com/lcgamboa/picsimlab/releases). Pick the build that matches your Ubuntu version:

```bash
sudo dpkg -i PICSimLab_<version>_<date>_Ubuntu_<release>_amd64.deb
sudo apt --fix-broken install        # pulls in any missing dependencies
```

---

## 7. Miscellaneous helpers

A few small additions that came up during setup:

```bash
sudo apt install python3-pip python3-pygments    # syntax highlighting in some tools
```

---

## Summary checklist

| Component | Package(s) |
|---|---|
| Base utilities | `build-essential`, `curl`, `git`, `libssl-dev` |
| Rust | rustup, `thumbv6m/7m/7em` targets, `llvm-tools`, `cargo-binutils`, `cargo-generate`, `flip-link`, `itm` |
| ARM toolchain | `gcc-arm-none-eabi`, `binutils-arm-none-eabi`, `gdb-multiarch`, `qemu-system-arm` |
| Probes / flashing | `stlink-tools`, `openocd`, `probe-rs` |
| Serial terminal | `picocom` (and/or `minicom`, `screen`, `putty`) |
| PIC simulator (optional) | `libfuse2t64`, PICSimLab `.deb` |
| Editors (optional) | `neovim`, VS Code (snap), `marktext` (snap) |
