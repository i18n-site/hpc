use std::{borrow::Borrow, fmt};

use pb_jelly::Message;
use sonic_rs::{Serialize, to_string};

use crate::{Code, CodeBody, CodeLi, State};

impl fmt::Display for CodeLi {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "CodeLi {:?}", self.li)?;
    Ok(())
  }
}

pub fn code_li() -> CodeLi {
  Default::default()
}

impl std::error::Error for CodeLi {}

impl CodeLi {
  pub fn push(&mut self, code: u32) {
    self.li.push(code);
  }
  pub fn throw(&mut self) -> anyhow::Result<()> {
    if self.li.is_empty() {
      return Ok(());
    }
    Err(
      CodeLi {
        li: std::mem::take(&mut self.li),
      }
      .into(),
    )
  }
}

#[macro_export]
macro_rules! err_code_li {
  ($this:ident) => {
    let mut err_code_li = $crate::code_li();
    macro_rules! err {
      ($mod:ident $code:ident) => {
        err_code_li.push($this::err::$mod::$code);
      };
      () => {
        err_code_li.throw()?;
      };
    }
  };
}
