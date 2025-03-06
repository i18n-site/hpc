use std::str::FromStr;

use axum::{
  body,
  body::to_bytes,
  extract::Request,
  http::{HeaderName, HeaderValue, StatusCode},
  response::{IntoResponse, Response},
};
use ctx_::{Ctx, SetHeader};
use hpc_captcha::Captcha;
use ih::{BinLi, CallLi, State};
use pb_jelly::Message;

use super::{batch, one};

pub async fn run<const BATCH_LIMIT: usize, Hpc: crate::Hpc, GenCaptcha: hpc_captcha::GenCaptcha>(
  request: Request,
) -> Response {
  let (parts, body) = request.into_parts();
  let ctx: Ctx = parts.into();
  match to_bytes(body, usize::MAX).await {
    Ok(body) => {
      let mut captcha = Captcha::<GenCaptcha>::new();

      let bin_li: BinLi = match CallLi::deserialize_from_slice(&body) {
        Ok(CallLi { func_li, args_li }) => match func_li.len() {
          0 => return (StatusCode::BAD_REQUEST, "no func call").into_response(),
          1 => one::<Hpc, _>(func_li[0], &args_li[0], &ctx, &mut captcha).await,
          _ => {
            let cost = cost_time::start();
            let bin_li = batch::<Hpc, BATCH_LIMIT, _>(func_li, args_li, &ctx, &captcha).await;
            println!("{}s", cost.sec());
            bin_li
          }
        },
        Err(err) => super::bin_li(State::ARGS_INVALID, err.to_string()),
      };

      let mut response =
        (StatusCode::OK, body::Bytes::from(bin_li.serialize_to_vec())).into_response();
      if ctx.has::<SetHeader>() {
        let response_header = response.headers_mut();
        let set_header: &SetHeader = ctx_::sync::get(&ctx);
        for i in set_header.iter() {
          if let Ok(header_name) = xerr::ok!(HeaderName::from_str(&i.0)) {
            if let Ok(header_val) = xerr::ok!(HeaderValue::from_str(&i.1)) {
              response_header.append(header_name, header_val);
            }
          }
        }
      }

      response
    }
    Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
  }
}
