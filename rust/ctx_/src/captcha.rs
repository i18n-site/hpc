use aok::Result;
use captcha_verify::captcha_verify;
use hpc_captcha::GenCaptcha;
use ih::CodeBody;

use crate::{Ctx, Extract};

pub struct Captcha(pub bool);

impl Extract for Captcha {
  async fn from_ctx(ctx: &Ctx) -> Result<Self> {
    Ok(Self(captcha_verify(&ctx.req.headers).await))
  }
}

pub async fn captcha<G: GenCaptcha>(
  ctx: &Ctx,
  captcha: &hpc_captcha::Captcha<G>,
) -> Result<(), CodeBody> {
  if let Ok::<&Captcha, _>(c) = crate::get(ctx).await {
    if c.0 {
      return Ok(());
    }
  }
  Err(captcha.get().await?)
}
