use std::fmt;

use pb_jelly::Message;

use crate::{Code, CodeBody, State};

impl fmt::Display for Code {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Code {}", self.inner)
  }
}

impl std::error::Error for Code {}

pub fn code<T>(code: impl Into<u32>) -> anyhow::Result<T> {
  Err(Code { inner: code.into() }.into())
}
