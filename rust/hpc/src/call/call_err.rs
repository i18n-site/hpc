use aok::Error;
use hpc_captcha::{Captcha, GenCaptcha};
use icall::{CodeBody, State};
use pb_jelly::ClosedProtoEnum;
use tracing::warn;

use super::res;
use crate::Hpc;

#[derive(Debug)]
struct CallErr {
  pub func: &'static str,
  pub args: Vec<String>,
  pub err: String,
}

impl std::fmt::Display for CallErr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}({}) {}", self.func, self.args.join(","), self.err)
  }
}

impl std::error::Error for CallErr {}

pub async fn call_err<H: Hpc, G: GenCaptcha>(
  func: H::Func,
  err: impl Into<Error>,
  captcha: &mut Captcha<G>,
) -> CodeBody {
  let err = err.into();

  if let Some(r) = icall::err::try_into(&err) {
    if r.0 == State::CAPTCHA {
      match captcha.get().await {
        Ok(bin) => return res(State::CAPTCHA, bin),
        Err(err) => return rt_err(State::MIDDLEWARE_ERROR, format!("captcha {err}")),
      }
    }
    return r;
  }

  if let Some(r) = err.downcast_ref::<CallErr>() {
    tracing::error!("{r}");
    return res(State::CALL_ERROR, format!("{} {}", r.func, r.err));
  }

  let name = func.name();
  tracing::error!("{name} {err}");
  rt_err(State::CALL_ERROR, format!("{name} {err}"))
}

fn rt_err(code: State, err: impl std::fmt::Display) -> CodeBody {
  let err = format!("{err}");
  warn!("{err}");
  res(code, err)
}

pub fn miss_func(func: u32) -> CodeBody {
  rt_err(State::MISS_FUNC, format!("{func}"))
}
