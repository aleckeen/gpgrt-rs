use std::ffi::CStr;

pub struct Error(gpgrt_sys::gpg_err_code_t);

impl Error
{
  pub fn error_string(&self) -> &str
  {
    let ptr = unsafe { gpgrt_sys::gpg_strerror(self.0) };
    unsafe { CStr::from_ptr(ptr) }.to_str().unwrap()
  }
}

impl std::fmt::Debug for Error
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    f.debug_struct("Error")
      .field("error_code", &self.0)
      .field("error_msg", &self.error_string())
      .finish()
  }
}

impl std::fmt::Display for Error
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    f.write_str(&format!(
      "gpg returned with an error code of {}: {}",
      self.0,
      self.error_string()
    ))
  }
}

impl std::error::Error for Error {}
