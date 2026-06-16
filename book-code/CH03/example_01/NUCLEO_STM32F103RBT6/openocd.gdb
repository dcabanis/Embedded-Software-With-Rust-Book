target extended-remote :3333
monitor reset halt
load
monitor reset halt
b main
continue
