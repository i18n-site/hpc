use futures::stream::{FuturesOrdered, StreamExt};
use hpc_captcha::{Captcha, GenCaptcha};
use icall::{BinLi, CodeBody, State};
use pb_jelly::Message;
use req_::Req;

use super::{call_err, miss_func, res};
use crate::Hpc;

pub async fn batch<H: Hpc, const BATCH_LIMIT: usize, G: GenCaptcha>(
  func_id_li: Vec<u32>,
  args_li: Vec<Vec<u8>>,
  req: &Req,
  captcha: &mut Captcha<G>,
) -> CodeBody {
  let len = func_id_li.len();
  if len > BATCH_LIMIT {
    return res(State::BATCH_LIMIT, format!("CallLi {len} > {BATCH_LIMIT}"));
  }

  let mut futures_ordered = FuturesOrdered::new();
  let mut func_li = Vec::with_capacity(len);

  for (func_id, args) in func_id_li.into_iter().zip(args_li.iter()) {
    match H::Func::try_from(func_id) {
      Ok(func) => {
        func_li.push(func);
        futures_ordered.push_back(H::run_with_log(req, func, args));
      }
      Err(_) => {
        return miss_func(func_id);
      }
    }
  }

  let mut bin_li = Vec::with_capacity(len);
  let mut state_li = Vec::with_capacity(len);
  let mut pos = 0;

  while let Some(result) = futures_ordered.next().await {
    match result {
      Ok(bin) => {
        state_li.push(State::OK as u32);
        bin_li.push(bin);
      }
      Err(err) => {
        let r = call_err::<H, _>(func_li[pos], err, captcha).await;
        state_li.push(r.0 as u32);
        bin_li.push(r.1.into());
      }
    }
    pos += 1;
  }

  let bin_li = BinLi { state_li, bin_li };

  (State::OK, bin_li.serialize_to_vec().into())
}
