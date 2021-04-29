use std::env;
use std::path::PathBuf;

use libgpg_error_src::Build;

fn main() {
    println!(
        "cargo:rerun-if-changed={}",
        libgpg_error_src::source_dir().display()
    );

    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().parse().unwrap();

    let mut build = Build::new();
    let artifacts = build.build();

    bindgen::builder()
        .header(
            artifacts
                .include_dir()
                .join("gpgrt.h")
                .display()
                .to_string(),
        )
        .size_t_is_usize(true)
        .use_core()
        .generate()
        .unwrap()
        .write_to_file(manifest_dir.join("src/ffi.rs"))
        .unwrap();

    artifacts.print_cargo_metadata();
}
