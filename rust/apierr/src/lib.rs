use std::{error::Error, fmt};

pub const PRECONDITION_FAILED: u16 = 412;

#[derive(Debug)]
pub struct Response {
  pub code: u16,
  pub body: Vec<u8>,
}

impl fmt::Display for Response {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Response {} : {:?}", self.code, self.body)
  }
}

impl Error for Response {}

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
