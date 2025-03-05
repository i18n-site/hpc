use hpc_captcha::{Captcha, GenCaptcha};
use ih::BinLi;
use ctx_::Ctx;

use super::miss_func;
use crate::Hpc;

pub async fn one<H: Hpc, G: GenCaptcha>(
  func_id: u32,
  args: &[u8],
  req: &Ctx,
  captcha: &mut Captcha<G>,
) -> BinLi {
  let r = match H::Func::try_from(func_id) {
    Ok(func) => H::run_with_log(req, func, args, captcha).await,
    Err(_) => miss_func(func_id),
  };
  BinLi {
    state_li: vec![r.0 as u32],
    bin_li: vec![r.1],
  }
}
