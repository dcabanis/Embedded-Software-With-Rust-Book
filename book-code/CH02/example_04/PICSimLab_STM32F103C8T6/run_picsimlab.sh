#!/bin/sh

PICSIMLAB_DEFAULT_BIN="/usr/bin/picsimlab"
PICSIMLAB_BIN="${PICSIMLAB_BIN:-$PICSIMLAB_DEFAULT_BIN}"
TERMINAL_APP="${TERMINAL_APP:-}"
ELF_PATH="target/thumbv7m-none-eabi/release/app"

print_usage() {
    cat <<EOF
Usage: ./run_picsimlab.sh [--help|-h]

Options:
  -h, --help  Show this help

Environment:
  PICSIMLAB_BIN  Override PICSimLab executable path/command
  TERMINAL_APP   Override terminal emulator path/command
EOF
}

while [ $# -gt 0 ]; do
    case "$1" in
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            echo "\033[1;91mUnknown option: $1\033[0m"
            print_usage
            exit 1
            ;;
    esac
    shift
done

find_terminal() {
    if [ -n "$TERMINAL_APP" ]; then
        if [ -x "$TERMINAL_APP" ]; then
            echo "$TERMINAL_APP"
            return 0
        fi
        if command -v "$TERMINAL_APP" >/dev/null 2>&1; then
            command -v "$TERMINAL_APP"
            return 0
        fi
        echo "Warning: TERMINAL_APP='$TERMINAL_APP' was set but not found. Auto-detecting instead." >&2
    fi

    for candidate in x-terminal-emulator xfce4-terminal gnome-terminal konsole kitty alacritty mate-terminal lxterminal xterm; do
        if command -v "$candidate" >/dev/null 2>&1; then
            command -v "$candidate"
            return 0
        fi
    done

    return 1
}

resolve_picsimlab() {
    if [ -x "$PICSIMLAB_BIN" ]; then
        echo "$PICSIMLAB_BIN"
        return 0
    fi

    if command -v "$PICSIMLAB_BIN" >/dev/null 2>&1; then
        command -v "$PICSIMLAB_BIN"
        return 0
    fi

    if command -v picsimlab >/dev/null 2>&1; then
        command -v picsimlab
        return 0
    fi

    for candidate in \
        /usr/local/bin/picsimlab \
        /opt/picsimlab/picsimlab \
        "$HOME/Applications/PICSimLab.AppImage" \
        "$HOME/bin/PICSimLab.AppImage"
    do
        if [ -x "$candidate" ]; then
            echo "$candidate"
            return 0
        fi
    done

    return 1
}

launch_log_terminal() {
    terminal_bin="$1"
    log_cmd="watch tail -n 15 ~/.picsimlab/picsimlab_log0.txt"

    case "$(basename "$terminal_bin")" in
        gnome-terminal)
            "$terminal_bin" --title="LogWatch" -- sh -c "$log_cmd"
            ;;
        xfce4-terminal)
            "$terminal_bin" --title="LogWatch" --command="sh -c '$log_cmd'"
            ;;
        mate-terminal)
            "$terminal_bin" --title="LogWatch" -- sh -c "$log_cmd"
            ;;
        konsole)
            "$terminal_bin" --new-tab -p tabtitle="LogWatch" -e sh -c "$log_cmd"
            ;;
        alacritty)
            "$terminal_bin" -t "LogWatch" -e sh -c "$log_cmd"
            ;;
        kitty)
            "$terminal_bin" --title "LogWatch" sh -c "$log_cmd"
            ;;
        lxterminal)
            "$terminal_bin" --title="LogWatch" -e sh -c "$log_cmd"
            ;;
        xterm|x-terminal-emulator)
            "$terminal_bin" -T "LogWatch" -e sh -c "$log_cmd"
            ;;
        *)
            "$terminal_bin" -e sh -c "$log_cmd"
            ;;
    esac
}

spawn_log_terminal() {
    terminal_bin="$1"
    launch_log_terminal "$terminal_bin" >/dev/null 2>&1 &
}

# Kill any existing PICSimLab process without matching this script name
pkill -9 -x picsimlab 2>/dev/null
pkill -9 -x PICSimLab.AppImage 2>/dev/null

# Start with a fresh PICSimLab instance (best effort)
if [ -d "$HOME/.picsimlab" ]; then
    if [ -w "$HOME/.picsimlab" ]; then
        rm -rf "$HOME/.picsimlab" 2>/dev/null || \
            echo "\033[1;93mWarning: could not fully clean ~/.picsimlab (continuing).\033[0m"
    else
        echo "\033[1;93mWarning: ~/.picsimlab is not writable; skipping cleanup.\033[0m"
    fi
fi

# Removing previous compilation run's binary
rm -f app.bin

# Removing previous workspace
rm -f virtual_board.pzw

# Build the project
cargo build --release

# Testing if the compilation was sucessful
if [ $? -ne 0 ];

then
     echo "\033[1;91mCargo build failed see previous error messages\033[0m"
     echo "\033[1;91mPlease fix the code before proceeding with PicSimLab\033[0m"
     exit 1
fi

# Convering the ELF file into a binary
arm-none-eabi-objcopy -O binary "$ELF_PATH" app.bin

# Refresh the picsimlab workspace
cp app.bin picsimlab_workspace/mdump_Blue_Pill_stm32f103c8t6.bin
zip -r virtual_board.pzw picsimlab_workspace

echo "\033[0;32mPICSimLab Workspace file created\033[0m"
echo "\033[0;32mLaunching PICSimLab with app.bin\033[0m"

PICSIMLAB_EXECUTABLE="$(resolve_picsimlab)"
if [ $? -ne 0 ] || [ -z "$PICSIMLAB_EXECUTABLE" ]; then
    echo "\033[1;91mCould not find PICSimLab executable.\033[0m"
    echo "\033[1;91mSet PICSIMLAB_BIN to the full path (or command name), e.g.:\033[0m"
    echo "\033[1;91mPICSIMLAB_BIN=\$HOME/Applications/PICSimLab.AppImage ./run_picsimlab.sh\033[0m"
    exit 1
fi

TERMINAL_EXECUTABLE="$(find_terminal)"

# Start a new instance of PICSimLab
"$PICSIMLAB_EXECUTABLE" virtual_board.pzw &

# Launch a terminal that tails the log, using a custom window title when supported
if [ -n "$TERMINAL_EXECUTABLE" ]; then
    spawn_log_terminal "$TERMINAL_EXECUTABLE"
else
    echo "\033[1;93mNo supported terminal application found. Skipping log viewer window.\033[0m"
fi
