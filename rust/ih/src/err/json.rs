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

pub fn json() -> Json {
  Default::default()
}

impl std::error::Error for Json {}

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

  pub fn throw(&mut self) -> anyhow::Result<()> {
    if self.inner.is_empty() {
      return Ok(());
    }
    let mut err = Json {
      inner: std::mem::take(&mut self.inner),
    };
    err.inner.push('}');
    Err(err.into())
  }
}

// impl<K: AsRef<str>, V: Serialize> FnMut<(K, V)> for Json {
//   extern "rust-call" fn call_mut(&mut self, args: (K, V)) -> Self::Output {
//     self.set(args.0, args.1);
//   }
// }
//
// impl<K: AsRef<str>, V: Serialize> FnOnce<(K, V)> for Json {
//   type Output = ();
//   extern "rust-call" fn call_once(self, _args: (K, V)) -> Self::Output {
//     unimplemented!()
//   }
// }
//
//
// impl FnMut<()> for Json {
//   extern "rust-call" fn call_mut(&mut self, args: ()) -> Self::Output {
//     if self.inner.is_empty() {
//       return Ok(());
//     }
//     let mut err = Json {
//       inner: std::mem::take(&mut self.inner),
//     };
//     err.inner.push('}');
//     Err(err.into())
//   }
// }
//
// impl FnOnce<()> for Json {
//   type Output = anyhow::Result<()>;
//   extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
//     unimplemented!()
//   }
// }
