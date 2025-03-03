use std::fmt;

use pb_jelly::Message;
use sonic_rs::{Serialize, to_string};

use crate::{Code, CodeBody, Json, State};

impl fmt::Display for Json {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "Json {}", self.inner)?;
    Ok(())
  }
}

impl std::error::Error for Json {}

pub fn json() -> Json {
  Default::default()
}

impl Json {
  pub fn set<V: Serialize>(
    &mut self,
    key: impl AsRef<str>,
    val: impl AsRef<V>,
  ) -> sonic_rs::Result<()> {
    let inner = &mut self.inner;
    inner.push(if inner.is_empty() { '{' } else { ',' });
    self.inner += &to_string(key.as_ref())?;
    self.inner.push(':');
    self.inner += &to_string(val.as_ref())?;
    Ok(())
  }

  pub fn end(mut self) -> anyhow::Result<()> {
    if self.inner.is_empty() {
      return Ok(());
    }
    self.inner.push('}');
    Err(self)?
  }
}
