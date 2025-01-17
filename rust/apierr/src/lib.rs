#![feature(try_trait_v2)]

use std::{error::Error, fmt};

#[derive(Debug)]
pub struct Response {
  pub code: u16,
  pub body: Vec<u8>,
}

impl From<(u16, Vec<u8>)> for Response {
  fn from((code, body): (u16, Vec<u8>)) -> Self {
    Response { code, body }
  }
}

impl From<u16> for Response {
  fn from(code: u16) -> Self {
    Response { code, body: vec![] }
  }
}

impl fmt::Display for Response {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Response {} : {:?}", self.code, self.body)
  }
}

impl Error for Response {}

#[cfg(feature = "err")]
mod err;

#[cfg(feature = "err")]
pub use err::*;

// pub fn user() -> Result<(), Response> {
//   Err(Response {
//     code: PRECONDITION_FAILED,
//     body: b"".into(),
//   })
// }
//
// User,
// Captcha,
// Form {
//   err_li: Vec<(String, u64)>
// },
