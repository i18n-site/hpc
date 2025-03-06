use aok::Error;
use hpc_captcha::{Captcha, GenCaptcha};
use ih::{CodeBody, State};
use tracing::warn;

pub async fn call_err<G: GenCaptcha>(
  func: &str,
  err: impl Into<Error>,
  captcha: &Captcha<G>,
  get_args: impl FnOnce() -> String,
) -> Result<CodeBody, CodeBody> {
  match ih::err::try_into(err.into()) {
    Ok(r) => {
      if r.0 == State::CAPTCHA {
        return captcha.get().await;
      }
      Ok(r)
    }
    Err(err) => {
      tracing::error!("{func} {} {err} ", get_args());
      Err((State::CALL_ERROR, format!("{func} {err}").as_bytes().into()))
    }
  }
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
