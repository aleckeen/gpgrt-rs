use std::env;
use std::path::{Path, PathBuf};

use ffi_tools::{Artifacts, Project};

pub fn source_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("gpgrt")
}

pub struct Build {
    project: Project,
    install_dir: PathBuf,
    static_lib: Option<bool>,
    shared_lib: Option<bool>,
    doc: Option<bool>,
}

impl Build {
    pub fn new() -> Self {
        let out_dir: PathBuf = env::var("OUT_DIR").unwrap().parse().unwrap();
        let build_dir = out_dir.join("gpgrt-build");
        let src_dir = build_dir.join("src");
        let install_dir = build_dir.join("install");
        let mut project = Project::new("gpgrt", &source_dir());
        project.cp_src(&src_dir);
        project.autogen();
        Self {
            project,
            install_dir,
            static_lib: None,
            shared_lib: None,
            doc: None,
        }
    }

    pub fn src_dir<P: AsRef<Path>>(&mut self, new_src_dir: P) {
        self.project.mv_src(new_src_dir)
    }

    pub fn install_dir<P: AsRef<Path>>(&mut self, new_install_dir: P) {
        self.install_dir = new_install_dir.as_ref().to_path_buf();
    }

    pub fn enable_static(&mut self, enable: bool) {
        self.static_lib = Some(enable);
    }

    pub fn enable_shared(&mut self, enable: bool) {
        self.shared_lib = Some(enable);
    }

    pub fn enable_doc(&mut self, enable: bool) {
        self.doc = Some(enable);
    }

    pub fn build(&self) {
        let mut configure = self.project.configure();
        if let Some(enable) = self.static_lib {
            if enable {
                configure.enable("static");
            } else {
                configure.disable("static");
            }
        }
        if let Some(enable) = self.shared_lib {
            if enable {
                configure.enable("shared");
            } else {
                configure.disable("shared");
            }
        }
        if let Some(enable) = self.doc {
            if enable {
                configure.enable("doc");
            } else {
                configure.disable("doc");
            }
        }
        configure.prefix(&self.install_dir);
        configure.configure();
        self.project.make();
    }

    pub fn check(&self) {
        self.project.check();
    }

    pub fn install(self) -> Artifacts {
        self.project.install();
        let bin_dir = self.install_dir.join("bin");
        let include_dir = self.install_dir.join("include");
        let lib_dir = self.install_dir.join("lib");
        Artifacts {
            install_dir: self.install_dir,
            bin_dir,
            include_dir,
            lib_dir,
            libs: vec!["gpg-error"],
        }
    }
}
