use std::marker::PhantomData;

use aok::Result;

pub trait GenCaptcha {
  fn get() -> impl Future<Output = Result<Vec<u8>>> + Send;
}

pub struct Captcha<G: GenCaptcha> {
  exist: bool,
  _g: PhantomData<G>,
}

impl<G: GenCaptcha> Captcha<G> {
  pub fn new() -> Self {
    Captcha {
      exist: false,
      _g: PhantomData,
    }
  }

  pub async fn get(&mut self) -> Result<Vec<u8>> {
    if self.exist {
      return Ok(vec![]);
    }
    self.exist = true;
    G::get().await
  }
}
