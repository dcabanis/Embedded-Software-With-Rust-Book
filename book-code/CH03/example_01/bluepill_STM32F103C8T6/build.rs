use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut file = File::create(out_dir.join("memory.x")).unwrap();

    writeln!(
        file,
        "MEMORY\n{{\n  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 64K\n  RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 20K\n}}"
    )
    .unwrap();

    println!("cargo:rustc-link-search={}", out_dir.display());
}
