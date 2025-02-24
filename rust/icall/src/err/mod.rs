use pb_jelly::Message;

use crate::{Code, CodeBody, Json, State};

pub fn try_into(err: &anyhow::Error) -> Option<CodeBody> {
  if let Some(t) = err.downcast_ref::<Code>() {
    return Some((State::CODE, Default::default()));
  }

  if let Some(t) = err.downcast_ref::<Json>() {
    return Some((State::JSON, t.serialize_to_vec().into()));
  }
  None
}

#[cfg(feature = "err_code")]
pub mod code;

#[cfg(feature = "err_json")]
pub mod json;
