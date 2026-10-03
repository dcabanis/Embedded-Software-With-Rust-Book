fn main() {
    println!("cargo:rerun-if-changed=c/device_status.c");
    println!("cargo:rerun-if-changed=c/device_status.h");

    // Compiles c/device_status.c for the firmware's own target (picked up
    // from the TARGET env var Cargo sets) and links the resulting static
    // library automatically.
    cc::Build::new()
        .file("c/device_status.c")
        .include("c")
        .compile("device_status");
}
