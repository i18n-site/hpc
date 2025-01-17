use std::ops::{AddAssign, ControlFlow, FromResidual};

use icall::CodeBin;
use num_traits::cast::ToPrimitive;
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
  CodeMsgLi = 4,
}

pub const PRECONDITION_FAILED: u16 = 412;

pub type ErrResponse = Result<(), Response>;

pub fn rt_proto(code: ErrCode, msg: impl Message) -> ErrResponse {
  rt(code, msg.serialize_to_vec())
}

pub fn rt(code: ErrCode, bin: Vec<u8>) -> ErrResponse {
  Err(
    (
      PRECONDITION_FAILED,
      CodeBin {
        code: code as _,
        bin,
      }
      .serialize_to_vec(),
    )
      .into(),
  )
}

pub fn user() -> ErrResponse {
  rt(ErrCode::User, vec![])
}

pub fn code<N: ToPrimitive>(code: N) -> ErrResponse {
  if let Some(code) = code.to_u32() {
    if code > 0 {
      return rt_proto(ErrCode::Code, icall::Code { code });
    }
  }
  Ok(())
}

#[derive(Default, Debug)]
pub struct CodeMsgLi {
  pub code_li: Vec<u32>,
  pub msg_li: Vec<String>,
}

impl<N: ToPrimitive, S: Into<String>> AddAssign<(N, S)> for CodeMsgLi {
  fn add_assign(&mut self, rhs: (N, S)) {
    if let Some(code) = rhs.0.to_u32() {
      self.code_li.push(code);
      self.msg_li.push(rhs.1.into());
    }
  }
}

pub fn code_msg_li() -> CodeMsgLi {
  CodeMsgLi::default()
}

impl std::ops::Try for CodeMsgLi {
  type Output = ();
  type Residual = CodeMsgLi;

  fn branch(self) -> ControlFlow<Self::Residual> {
    if self.code_li.is_empty() {
      return ControlFlow::Continue(());
    }
    return ControlFlow::Break(self);
  }

  fn from_output((): ()) -> Self {
    CodeMsgLi::default()
  }
}

impl FromResidual<CodeMsgLi> for CodeMsgLi {
  fn from_residual(r: CodeMsgLi) -> Self {
    r
  }
}

impl<T> FromResidual<CodeMsgLi> for Result<T, aok::Error> {
  fn from_residual(r: CodeMsgLi) -> Self {
    let response: Response = (
      PRECONDITION_FAILED,
      icall::CodeMsgLi {
        code_li: r.code_li,
        msg_li: r.msg_li,
      }
      .serialize_to_vec(),
    )
      .into();
    Err(response.into())
  }
}

#[derive(Default, Debug)]
pub struct CodeLi(pub Vec<u32>);

impl<N: ToPrimitive> AddAssign<N> for CodeLi {
  fn add_assign(&mut self, rhs: N) {
    if let Some(code) = rhs.to_u32() {
      self.0.push(code);
    }
  }
}

pub fn code_li() -> CodeLi {
  CodeLi::default()
}

impl std::ops::Try for CodeLi {
  type Output = ();
  type Residual = CodeLi;

  fn branch(self) -> ControlFlow<Self::Residual> {
    if self.0.is_empty() {
      return ControlFlow::Continue(());
    }
    return ControlFlow::Break(self);
  }

  fn from_output((): ()) -> Self {
    CodeLi::default()
  }
}

impl FromResidual<CodeLi> for CodeLi {
  fn from_residual(r: CodeLi) -> Self {
    r
  }
}

impl<T> FromResidual<CodeLi> for Result<T, aok::Error> {
  fn from_residual(r: CodeLi) -> Self {
    let response: Response = (
      PRECONDITION_FAILED,
      icall::CodeLi { li: r.0 }.serialize_to_vec(),
    )
      .into();
    Err(response.into())
  }
}

// pub fn captcha() -> Result<(), Response> {
//   Err(
//     (
//       PRECONDITION_FAILED,
//       CodeBin {
//         code: ErrCode::User as u32,
//         bin: icall::Captcha {}.serialize_to_vec(),
//       }
//       .serialize_to_vec(),
//     )
//       .into(),
//   )
// }
