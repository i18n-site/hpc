use aok::Result;
use captcha_verify::captcha_verify;
use hpc_captcha::GenCaptcha;
use icall::CodeBody;

use crate::{Extract, Req};

pub struct Captcha(pub bool);

impl Extract for Captcha {
  async fn from_req(req: &Req) -> Result<Self> {
    Ok(Self(captcha_verify(&req.headers).await))
  }
}

pub async fn captcha<G: GenCaptcha>(
  req: &Req,
  captcha: &hpc_captcha::Captcha<G>,
) -> Result<(), CodeBody> {
  if let Ok::<&Captcha, _>(c) = crate::get(req).await {
    if c.0 {
      return Ok(());
    }
  }
  Err(captcha.get().await?)
}
