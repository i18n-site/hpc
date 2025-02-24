use std::str::FromStr;

use axum::{
  body,
  http::{HeaderMap, HeaderName, HeaderValue, StatusCode, header::CONTENT_TYPE},
  response::{IntoResponse, Response},
};
use hpc_captcha::Captcha;
use icall::{BinLi, CallLi, State};
use pb_jelly::Message;
use req_::{Req, SetHeader};

use super::{batch, one};

pub async fn run<const BATCH_LIMIT: usize, Hpc: crate::Hpc, GenCaptcha: hpc_captcha::GenCaptcha>(
  headers: HeaderMap,
  body: body::Bytes,
) -> Response {
  let req: Req = headers.into();

  let mut captcha = Captcha::<GenCaptcha>::new();

  let bin_li: BinLi;

  #[allow(clippy::never_loop)]
  loop {
    if let Some(content_type) = req.headers.get(CONTENT_TYPE) {
      if content_type == "h" {
        bin_li = match CallLi::deserialize_from_slice(&body) {
          Ok(CallLi { func_li, args_li }) => match func_li.len() {
            0 => return (StatusCode::BAD_REQUEST, "no func call").into_response(),
            1 => one::<Hpc, _>(func_li[0], &args_li[0], &req, &mut captcha).await,
            _ => {
              let cost = cost_time::start();
              let bin_li = batch::<Hpc, BATCH_LIMIT, _>(func_li, args_li, &req, &captcha).await;
              println!("{}s", cost.sec());
              bin_li
            }
          },
          Err(err) => super::bin_li(State::ARGS_INVALID, err.to_string()),
        };
        break;
      }
    }
    return (StatusCode::BAD_REQUEST, "content-type != h").into_response();
  }

  let mut response = (StatusCode::OK, body::Bytes::from(bin_li.serialize_to_vec())).into_response();
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
