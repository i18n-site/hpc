use anyhow::Result;
use pb_jelly::Message;

use crate::{Bin, Code, CodeBody, Json, State};

pub fn try_into(err: anyhow::Error) -> Result<CodeBody> {
  if let Some(t) = err.downcast_ref::<State>() {
    return Ok((*t, Default::default()));
  }

  if let Some(t) = err.downcast_ref::<Code>() {
    return Ok((State::CODE, Default::default()));
  }

  if let Some(t) = err.downcast_ref::<Json>() {
    return Ok((State::JSON, t.inner.as_bytes().into()));
  }

  match err.downcast::<Bin>() {
    Ok(t) => Ok((State::BIN, t.inner)),
    Err(err) => Err(err),
  }
}

#[cfg(feature = "err_code_li")]
pub mod code_li;

#[cfg(feature = "err_code")]
pub mod code;

#[cfg(feature = "err_json")]
pub mod json;

#[cfg(feature = "err_bin")]
pub mod bin;

pub mod state;
