use std::{
  marker::PhantomData,
  sync::atomic::{AtomicBool, Ordering},
};

use aok::Result;

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

  pub async fn get(&self) -> Result<Vec<u8>> {
    if self.exist.swap(true, Ordering::SeqCst) {
      Ok(vec![])
    } else {
      G::get().await
    }
  }
}
