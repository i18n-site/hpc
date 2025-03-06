use futures::stream::{FuturesOrdered, StreamExt};
use hpc_captcha::{Captcha, GenCaptcha};
use ih::{BinLi, State};
use ctx_::Ctx;

use crate::Hpc;

pub async fn batch<H: Hpc, const BATCH_LIMIT: usize, G: GenCaptcha>(
  func_id_li: Vec<u32>,
  args_li: Vec<Vec<u8>>,
  req: &Ctx,
  captcha: &Captcha<G>,
) -> BinLi {
  let len = func_id_li.len();
  let mut state_li = Vec::with_capacity(len);
  let mut bin_li = Vec::with_capacity(len);

  if len > BATCH_LIMIT {
    state_li.push(State::BATCH_LIMIT.into());
    bin_li.push(format!("CallLi {len} > {BATCH_LIMIT}").into());
    return BinLi { state_li, bin_li };
  }

  let mut futures_ordered = FuturesOrdered::new();
  let mut func_li = Vec::with_capacity(len);

  for (func_id, args) in func_id_li.into_iter().zip(args_li.iter()) {
    match H::Func::try_from(func_id) {
      Ok(func) => {
        func_li.push(func);
        futures_ordered.push_back(H::run_with_log(req, func, args, captcha));
      }
      Err(_) => {
        state_li.push(State::MISS_FUNC.into());
        bin_li.push(format!("miss func {func_id}").into());
      }
    }
  }

  while let Some(result) = futures_ordered.next().await {
    state_li.push(result.0.into());
    bin_li.push(result.1);
  }

  BinLi { state_li, bin_li }
}
