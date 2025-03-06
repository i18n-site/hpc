use std::{
  marker::PhantomData,
  sync::atomic::{AtomicBool, Ordering},
};

use aok::Result;
use ih::{CodeBody, State};

pub trait GenCaptcha {
  fn get() -> impl Future<Output = Result<Vec<u8>>> + Send;
}

pub struct Captcha<G: GenCaptcha> {
  exist: AtomicBool,
  _g: PhantomData<G>,
}

impl<G: GenCaptcha> Default for Captcha<G> {
  fn default() -> Self {
    Self::new()
  }
}

impl<G: GenCaptcha> Captcha<G> {
  pub fn new() -> Self {
    Captcha {
      exist: AtomicBool::new(false),
      _g: PhantomData,
    }
  }

  pub async fn get(&self) -> Result<CodeBody, CodeBody> {
    if self.exist.swap(true, Ordering::SeqCst) {
      Ok((State::CAPTCHA, vec![]))
    } else {
      match G::get().await {
        Ok(bin) => Ok((State::CAPTCHA, bin)),
        Err(err) => Err((State::MIDDLEWARE_ERROR, format!("captcha {err}").into())),
      }
    }
  }
}
