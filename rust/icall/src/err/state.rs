use std::fmt;

use pb_jelly::Message;

use crate::{CodeBody, State};

impl fmt::Display for State {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "State::{:?}", self)
  }
}

impl std::error::Error for State {}

pub fn captcha<T>() -> anyhow::Result<T> {
  Err(State::CAPTCHA.into())
}
