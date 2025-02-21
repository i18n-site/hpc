use std::{future::Future, str::FromStr};

use aok::{Error, Result};
use axum::{
  body,
  http::{HeaderMap, HeaderName, HeaderValue, StatusCode, header::CONTENT_TYPE},
  response::{IntoResponse, Response},
};
use futures::stream::{FuturesOrdered, StreamExt};
use icall::{BinLi, CallLi, CodeBody, State};
use pb_jelly::{ClosedProtoEnum, Message};
use req_::{Req, SetHeader};
use tracing::warn;

use crate::{CallErr, Captcha, GenCaptcha};

pub trait Hpc {
  type Func: TryFrom<u32> + ClosedProtoEnum + Copy;

  fn run(req: &Req, func: Self::Func, args: &[u8]) -> impl Future<Output = Result<Vec<u8>>> + Send;

  #[allow(async_fn_in_trait)]
  async fn run_with_log(req: &Req, func: Self::Func, args: &[u8]) -> Result<Vec<u8>> {
    let cost = cost_time::start();
    let r = Self::run(req, func, args).await;
    let name = func.name();
    let cost = cost.sec();
    match r {
      Ok(r) => {
        println!("{name} {cost}s");
        Ok(r)
      }
      Err(err) => {
        eprintln!("❌ {name} {err} {cost}s : {:?}", args);
        Err(err)
      }
    }
  }
}

fn res(code: State, body: impl AsRef<[u8]>) -> CodeBody {
  (code, body.as_ref().into())
}

async fn call_err<H: Hpc, G: GenCaptcha>(
  func: H::Func,
  err: impl Into<Error>,
  captcha: &mut Captcha<G>,
) -> CodeBody {
  let err = err.into();

  if let Some(r) = icall::err::try_into(&err) {
    if r.0 == State::CAPTCHA {
      match captcha.get().await {
        Ok(bin) => return res(State::CAPTCHA, bin),
        Err(err) => return rt_err(State::MIDDLEWARE_ERROR, format!("captcha {err}")),
      }
    }
    return r;
  }

  if let Some(r) = err.downcast_ref::<CallErr>() {
    tracing::error!("{r}");
    return res(State::CALL_ERROR, format!("{} {}", r.func, r.err));
  }

  let name = func.name();
  tracing::error!("{name} {err}");
  rt_err(State::CALL_ERROR, format!("{name} {err}"))
}

fn rt_err(code: State, err: impl std::fmt::Display) -> CodeBody {
  let err = format!("{err}");
  warn!("{err}");
  res(code, err)
}

fn miss_func(func: u32) -> CodeBody {
  rt_err(State::MISS_FUNC, format!("{func}"))
}
// Ok(Call { func, args }) =>

async fn one<H: Hpc, G: GenCaptcha>(
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

async fn batch<H: Hpc, const BATCH_LIMIT: usize, G: GenCaptcha>(
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

pub async fn run<H: Hpc, const BATCH_LIMIT: usize, G: GenCaptcha>(
  headers: HeaderMap,
  body: body::Bytes,
) -> Response {
  let req: Req = headers.into();

  let code_body;

  let mut captcha = Captcha::<G>::new();

  #[allow(clippy::never_loop)]
  loop {
    if let Some(content_type) = req.headers.get(CONTENT_TYPE) {
      if content_type == "h" {
        code_body = match CallLi::deserialize_from_slice(&body) {
          Ok(CallLi { func_li, args_li }) => match func_li.len() {
            0 => return (StatusCode::BAD_REQUEST, "no func call").into_response(),
            1 => one::<H, _>(func_li[0], &args_li[0], &req, &mut captcha).await,
            _ => {
              let cost = cost_time::start();
              let code_body =
                batch::<H, BATCH_LIMIT, _>(func_li, args_li, &req, &mut captcha).await;
              println!("{}s", cost.sec());
              code_body
            }
          },
          Err(err) => res(State::ARGS_INVALID, err.to_string()),
        };
        break;
      }
    }
    return (StatusCode::BAD_REQUEST, "content-type != h").into_response();
  }

  let mut response = (StatusCode::OK, body::Bytes::from(code_body.1)).into_response();
  if req.has::<SetHeader>() {
    let response_header = response.headers_mut();
    let set_header: &SetHeader = req_::sync::get(&req);
    for i in set_header.iter() {
      if let Ok(header_name) = xerr::ok!(HeaderName::from_str(&i.0)) {
        if let Ok(header_val) = xerr::ok!(HeaderValue::from_str(&i.1)) {
          response_header.insert(header_name, header_val);
        }
      }
    }
  }

  response
}
