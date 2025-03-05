use std::{borrow::Borrow, fmt};

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
  pub fn set<V: Serialize>(&mut self, key: impl AsRef<str>, val: impl Borrow<V>) {
    if let Ok(key) = xerr::ok!(to_string(key.as_ref())) {
      if let Ok(val) = xerr::ok!(to_string(val.borrow())) {
        let inner = &mut self.inner;
        inner.push(if inner.is_empty() { '{' } else { ',' });
        self.inner += &key;
        self.inner.push(':');
        self.inner += &val;
      }
    }
  }

  pub fn end(mut self) -> anyhow::Result<()> {
    if self.inner.is_empty() {
      return Ok(());
    }
    self.inner.push('}');
    Err(self)?
  }
}

impl<K: AsRef<str>, V: Serialize> FnMut<(K, V)> for Json {
  extern "rust-call" fn call_mut(&mut self, args: (K, V)) -> Self::Output {
    self.set(args.0, args.1);
  }
}

impl<K: AsRef<str>, V: Serialize> FnOnce<(K, V)> for Json {
  type Output = ();
  extern "rust-call" fn call_once(self, _args: (K, V)) -> Self::Output {
    unimplemented!()
  }
}
