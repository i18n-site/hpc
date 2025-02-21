use hpc_captcha::{Captcha, GenCaptcha};
use icall::{BinLi, CodeBody, State};
use pb_jelly::Message;
use req_::Req;

use super::{call_err, miss_func, res};
use crate::Hpc;

pub async fn one<H: Hpc, G: GenCaptcha>(
  func_id: u32,
  args: &[u8],
  req: &Req,
  captcha: &mut Captcha<G>,
) -> CodeBody {
  match H::Func::try_from(func_id) {
    Ok(func) => match H::run_with_log(req, func, args).await {
      Ok(bin) => res(State::OK, {
        BinLi {
          state_li: vec![State::OK as u32],
          bin_li: vec![bin],
        }
        .serialize_to_vec()
      }),
      Err(err) => call_err::<H, _>(func, err, captcha).await,
    },
    Err(_) => miss_func(func_id),
  }
}
