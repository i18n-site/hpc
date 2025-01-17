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
pub enum Err {
  // 验证码错误
  Captcha = 0,
  // 当前用户未登录
  User = 1,
  // 权限不足
  Role = 2,
  Code = 3,
  CodeLi = 4,
  CodeMsg = 5,
  CodeMsgLi = 6,
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
