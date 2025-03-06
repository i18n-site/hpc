use std::{fmt, ops::Deref};

use append_only_vec::AppendOnlyVec;

use crate::{Ctx, sync::Extract};

pub const SET_COOKIE: &str = "Set-Cookie";

type HeaderLi = AppendOnlyVec<(Box<str>, Box<str>)>;

#[derive(Default)]
pub struct SetHeader(HeaderLi);

impl SetHeader {
  pub fn push(&self, key: impl AsRef<str>, val: impl AsRef<str>) {
    self.0.push((key.as_ref().into(), val.as_ref().into()));
  }
}

impl Deref for SetHeader {
  type Target = HeaderLi;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl fmt::Display for SetHeader {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "SetHeader: {:?}", self.0)
  }
}

impl fmt::Debug for SetHeader {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(self, f)
  }
}

impl Extract for SetHeader {
  fn from_ctx(_: &Ctx) -> Self {
    Default::default()
  }
}
