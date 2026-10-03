fn main() {
    println!("cargo:rerun-if-changed=c/callback_lib.c");
    println!("cargo:rerun-if-changed=c/callback_lib.h");

    cc::Build::new()
        .file("c/callback_lib.c")
        .include("c")
        .compile("callback_lib");
}
