use std::env;
use std::path::{Path, PathBuf};

use ffi_tools::{Artifacts, Project};

pub fn source_dir() -> PathBuf
{
  Path::new(env!("CARGO_MANIFEST_DIR")).join("gpgrt")
}

pub fn rerun_if_src_changed()
{
  println!("cargo:rerun-if-changed={}", source_dir().display());
}

pub struct Build
{
  project: Project,
  install_dir: PathBuf,
}

impl Default for Build
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl Build
{
  pub fn new() -> Self
  {
    let out_dir: PathBuf = env::var("OUT_DIR").unwrap().parse().unwrap();
    let build_dir = out_dir.join("gpgrt-build");
    let src_dir = build_dir.join("src");
    let install_dir = build_dir.join("install");
    let mut project = Project::new("gpgrt", &source_dir());
    project.cp_src(&src_dir);
    project.autogen();
    Self { project, install_dir }
  }

  pub fn src_dir<P: AsRef<Path>>(&mut self, new_src_dir: P)
  {
    self.project.mv_src(new_src_dir)
  }

  pub fn install_dir<P: AsRef<Path>>(&mut self, new_install_dir: P)
  {
    self.install_dir = new_install_dir.as_ref().to_path_buf();
  }

  pub fn build(&self)
  {
    let mut configure = self.project.configure();
    configure.enable("static");
    configure.disable("shared");
    configure.disable("doc");
    configure.prefix(&self.install_dir);
    configure.configure();
    self.project.make();
  }

  pub fn check(&self)
  {
    self.project.check();
  }

  pub fn install(self) -> Artifacts
  {
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
