use aok::Error;
use hpc_captcha::{Captcha, GenCaptcha};
use icall::{CodeBody, State};
use tracing::warn;

pub async fn call_err<G: GenCaptcha>(
  func: &str,
  err: impl Into<Error>,
  captcha: &Captcha<G>,
  get_args: impl FnOnce() -> String,
) -> CodeBody {
  let err = err.into();

  if let Some(r) = icall::err::try_into(&err) {
    if r.0 == State::CAPTCHA {
      match captcha.get().await {
        Ok(bin) => return (State::CAPTCHA, bin),
        Err(err) => return rt_err(State::MIDDLEWARE_ERROR, format!("captcha {err}")),
      }
    }
    return r;
  }

  tracing::error!("{func} {} {err} ", get_args());
  rt_err(State::CALL_ERROR, format!("{func} {err}"))
}

pub fn rt_err(code: State, err: impl std::fmt::Display) -> CodeBody {
  let err = format!("{err}");
  warn!("{err}");
  (code, err.as_bytes().into())
}

pub fn miss_func(func: u32) -> CodeBody {
  rt_err(State::MISS_FUNC, format!("{func}"))
}

pub fn args_invalid(func: &str) -> CodeBody {
  rt_err(State::ARGS_INVALID, func)
}
