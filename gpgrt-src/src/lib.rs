use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn source_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("gpgrt")
}

pub struct Build {
    out_dir: Option<PathBuf>,
}

impl Default for Build {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Artifacts {
    install_dir: PathBuf,
    include_dir: PathBuf,
    lib_dir: PathBuf,
    bin_dir: PathBuf,
    libs: Vec<&'static str>,
}

impl Build {
    pub fn new() -> Self {
        Self {
            out_dir: env::var_os("OUT_DIR").map(|s| PathBuf::from(s).join("gpgrt-build")),
        }
    }

    pub fn out_dir<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn build(&mut self) -> Artifacts {
        let out_dir = self.out_dir.as_ref().expect("OUT_DIR is not set");
        let build_dir = out_dir.join("build");
        let install_dir = out_dir.join("install");

        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).unwrap();
        }
        if install_dir.exists() {
            fs::remove_dir_all(&install_dir).unwrap();
        }

        let inner_dir = build_dir.join("src");
        fs::create_dir_all(&inner_dir).unwrap();
        cp_r(&source_dir(), &inner_dir);

        let mut autogen = Command::new("./autogen.sh");
        autogen.current_dir(&inner_dir);
        run_command(autogen, "generating configure script using autogen.sh");

        let mut configure = Command::new("./configure");
        configure.current_dir(&inner_dir);
        configure.args(&[
            &format!("--prefix={}", install_dir.display()),
            "--enable-static=yes",
            "--enable-shared=no",
            "--enable-doc=no",
        ]);
        run_command(configure, "configuring gpgrt");

        let mut make = Command::new("make");
        make.current_dir(&inner_dir);
        run_command(make, "building gpgrt");

        let mut make_check = Command::new("make");
        make_check.arg("check");
        make_check.current_dir(&inner_dir);
        run_command(make_check, "checking gpgrt");

        let mut make_install = Command::new("make");
        make_install.arg("install");
        make_install.current_dir(&inner_dir);
        run_command(make_install, "installing gpgrt");

        let lib_dir = install_dir.join("lib");
        let bin_dir = install_dir.join("bin");
        let include_dir = install_dir.join("include");
        Artifacts {
            install_dir,
            lib_dir,
            bin_dir,
            include_dir,
            libs: vec!["gpg-error"],
        }
    }
}

impl Artifacts {
    pub fn install_dir(&self) -> &Path {
        &self.install_dir
    }

    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    pub fn bin_dir(&self) -> &Path {
        &self.bin_dir
    }

    pub fn libs(&self) -> &[&str] {
        &self.libs
    }

    pub fn print_cargo_metadata(&self) {
        println!("cargo:rustc-link-search=native={}", self.lib_dir.display());
        for &lib in self.libs.iter() {
            println!("cargo:rustc-link-lib=static={}", lib);
        }
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
    }
}

fn cp_r(src: &Path, dst: &Path) {
    for f in fs::read_dir(src).unwrap() {
        let f = f.unwrap();
        let path = f.path();
        let name = path.file_name().unwrap();

        if name.to_str() == Some(".git") {
            continue;
        }

        let dst = dst.join(name);
        if f.file_type().unwrap().is_dir() {
            fs::create_dir_all(&dst).unwrap();
            cp_r(&path, &dst);
        } else {
            let _ = fs::remove_file(&dst);
            fs::copy(&path, &dst).unwrap();
        }
    }
}

fn run_command(mut command: Command, desc: &str) {
    println!("running {:?}", command);
    let status = command.status().unwrap();
    if !status.success() {
        panic!(
            "\n\
             \n\
             Error: {}:\n\
                 Command: {:?}\n\
                 Exit status: {}\n\
             \n\
             \n",
            desc, command, status
        );
    }
}
