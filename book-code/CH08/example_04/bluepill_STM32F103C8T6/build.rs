// Automates *bindgen* in build.rs, against the vendored jsmn
// JSON tokenizer.
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=c/wrapper.h");
    println!("cargo:rerun-if-changed=c/jsmn_impl.c");
    println!("cargo:rerun-if-changed=c/jsmn/jsmn.h");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cc::Build::new()
        .file("c/jsmn_impl.c")
        .include(format!("{manifest_dir}/c"))
        // Explicit rather than relying on the default, so the built
        // library and the bindgen invocation below can't drift apart if
        // a future toolchain changes its default.
        .flag("-fshort-enums")
        .compile("jsmn");

    // Parse the headers for the *target*, not the build host -- Cargo
    // provides the triple in the TARGET variable.
    let target = env::var("TARGET").unwrap();

    let bindings = bindgen::Builder::default()
        .header("c/wrapper.h")
        .clang_arg(format!("-I{manifest_dir}/c"))
        .clang_arg(format!("--target={target}"))
        // GCC's AAPCS-based ARM targets shrink an `enum` to the smallest
        // type that fits by default; libclang, which bindgen
        // runs headers through, does not assume that on its own. Without
        // this flag, jsmntype_t would be bound as a 4-byte c_uint even
        // though arm-none-eabi-gcc compiles jsmn_impl.c with a 1-byte
        // enum, a real instance of the mismatch.
        .clang_arg("-fshort-enums")
        .use_core()
        .ctypes_prefix("core::ffi")
        .allowlist_type("jsmntok_t")
        .allowlist_type("jsmntype_t")
        .allowlist_type("jsmn_parser")
        .allowlist_function("jsmn_init")
        .allowlist_function("jsmn_parse")
        .generate()
        .expect("Unable to generate jsmn bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("jsmn_bindings.rs"))
        .expect("Couldn't write jsmn bindings!");
}
