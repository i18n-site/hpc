use icall::CodeBin;
use pb_jelly::Message;

use crate::Response;

#[repr(u32)]
pub enum ErrCode {
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

pub const PRECONDITION_FAILED: u16 = 412;

pub fn user() -> Result<(), Response> {
  Err(
    (
      PRECONDITION_FAILED,
      CodeBin {
        code: ErrCode::User as u32,
        bin: vec![],
      }
      .serialize_to_vec(),
    )
      .into(),
  )
}

pub fn captcha() -> Result<(), Response> {
  Err(
    (
      PRECONDITION_FAILED,
      CodeBin {
        code: ErrCode::User as u32,
        bin: todo!("生成验证码"),
      }
      .serialize_to_vec(),
    )
      .into(),
  )
}

pub fn code(err_code: u32) -> Result<(), Response> {
  Err(
    (
      PRECONDITION_FAILED,
      CodeBin {
        code: ErrCode::Code as u32,
        bin: icall::Code { code: err_code }.serialize_to_vec(),
      }
      .serialize_to_vec(),
    )
      .into(),
  )
}
