use std::fmt;

use pb_jelly::Message;

use crate::Bin;

impl fmt::Display for Bin {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Bin {:?}", self.inner)
  }
}

impl std::error::Error for Bin {}

pub fn bin<T>(bin: impl AsRef<[u8]>) -> anyhow::Result<T> {
  Err(
    Bin {
      inner: bin.as_ref().into(),
    }
    .into(),
  )
}
