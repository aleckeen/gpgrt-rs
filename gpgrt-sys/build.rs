use std::env;
use std::path::PathBuf;

use gpgrt_src::Build;

fn main() {
    println!(
        "cargo:rerun-if-changed={}",
        gpgrt_src::source_dir().display()
    );

    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().parse().unwrap();

    let build = Build::new();
    build.build();
    build.check();
    let artifacts = build.install();
    artifacts.print_cargo_metadata();

    bindgen::builder()
        .header(artifacts.include_dir.join("gpgrt.h").display().to_string())
        .size_t_is_usize(true)
        .default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: true })
        .generate_comments(false)
        .allowlist_var("GPG.*")
        .allowlist_var("gpg.*")
        .allowlist_type("GPG.*")
        .allowlist_type("gpg.*")
        .allowlist_function("GPG.*")
        .allowlist_function("gpg.*")
        .generate()
        .unwrap()
        .write_to_file(manifest_dir.join("src/ffi.rs"))
        .unwrap();
}
