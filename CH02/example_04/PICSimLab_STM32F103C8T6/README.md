# List of commands

## Build

Release build:

```sh
cargo build --release
```

Debug build:

```sh
cargo build
```

## Run with PICSimLab

Default run (builds release, creates workspace, launches PICSimLab, opens log terminal when available):

```sh
./run_picsimlab.sh
```

Show script help:

```sh
./run_picsimlab.sh --help
```

## Environment overrides

Override PICSimLab executable path/command:

```sh
PICSIMLAB_BIN=$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh
```

Override terminal application used for log window:

```sh
TERMINAL_APP=gnome-terminal ./run_picsimlab.sh
```

Use both overrides together:

```sh
PICSIMLAB_BIN=/opt/picsimlab/picsimlab TERMINAL_APP=xfce4-terminal ./run_picsimlab.sh
```

## Notes

- If no supported terminal is found, PICSimLab still starts, but the log window is skipped.
