use aok::{Error, Result};
use axum::{
  body,
  http::{HeaderMap, StatusCode},
};
use futures::stream::{FuturesOrdered, StreamExt};
use icall::{BinLi, Call, CallLi};
use pb_jelly::{ClosedProtoEnum, Message};
use tracing::warn;

pub trait Hpc {
  type Func: TryFrom<i32> + ClosedProtoEnum + Copy;

  fn run(
    func: Self::Func,
    args: impl AsRef<[u8]> + Send,
  ) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;
}

pub type CodeBody = (StatusCode, Box<[u8]>);

fn res(code: StatusCode, body: impl AsRef<[u8]>) -> CodeBody {
  (code, body.as_ref().into())
}

fn func_err(func: impl ClosedProtoEnum, err: impl Into<Error>) -> CodeBody {
  let err = err.into();

  if err.is::<apierr::Response>() {
    let r = err.downcast::<apierr::Response>().unwrap();
    return res(
      StatusCode::from_u16(r.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
      r.body,
    );
  }
  rt_err(
    StatusCode::INTERNAL_SERVER_ERROR,
    format!("{} {err}", func.name()),
  )
}

fn rt_err(code: StatusCode, err: impl std::fmt::Display) -> CodeBody {
  let err = format!("{err}");
  warn!("{err}");
  res(code, err)
}

fn bad_request(err: impl std::fmt::Display) -> CodeBody {
  rt_err(StatusCode::BAD_REQUEST, err)
}

fn miss_func(func: i32) -> CodeBody {
  bad_request(format!("miss func {}", func))
}

async fn one<H: Hpc>(body: &[u8]) -> CodeBody {
  match Call::deserialize_from_slice(body) {
    Ok(Call { func, args }) => match H::Func::try_from(func) {
      Ok(func) => match H::run(func, args).await {
        Ok(bin) => res(StatusCode::OK, bin),
        Err(err) => func_err(func, err),
      },
      Err(_) => miss_func(func),
    },
    Err(err) => bad_request(err),
  }
}

async fn batch<H: Hpc, const BATCH_LIMIT: usize>(body: &[u8]) -> CodeBody {
  match CallLi::deserialize_from_slice(body) {
    Ok(CallLi { call_li }) => {
      let len = call_li.len();
      if len > BATCH_LIMIT {
        return res(
          StatusCode::PAYLOAD_TOO_LARGE,
          format!("call li len {len} > {BATCH_LIMIT}"),
        );
      }

      let mut futures_ordered = FuturesOrdered::new();
      let call_li_len = call_li.len();
      let mut func_li = Vec::with_capacity(call_li_len);

      for Call { func, args } in call_li {
        match H::Func::try_from(func) {
          Ok(func) => {
            func_li.push(func);
            futures_ordered.push_back(H::run(func, args));
          }
          Err(_) => {
            return miss_func(func);
          }
        }
      }

      let mut bin_li = Vec::with_capacity(call_li_len);
      let mut pos = 0;

      while let Some(result) = futures_ordered.next().await {
        match result {
          Ok(bin) => bin_li.push(bin),
          Err(err) => {
            return func_err(func_li[pos], err);
          }
        }
        pos += 1;
      }

      let bin_li = BinLi { bin_li };

      (StatusCode::OK, bin_li.serialize_to_vec().into())
    }
    Err(err) => bad_request(err),
  }
}

pub async fn run<H: Hpc, const BATCH_LIMIT: usize>(
  headers: HeaderMap,
  body: body::Bytes,
) -> CodeBody {
  if let Some(content_type) = headers.get("content-type") {
    if content_type == "1" {
      return one::<H>(&body).await;
    } else if content_type == "b" {
      return batch::<H, BATCH_LIMIT>(&body).await;
    }
  }
  res(StatusCode::BAD_REQUEST, "miss content-type")
}

// #[axum::debug_handler]
// pub async fn hpc(headers: HeaderMap, body: body::Bytes) -> CodeBody {
//   if let Some(content_type) = headers.get("content-type") {
//     if content_type == "1" {
//       return one::<C>(&body).await;
//     } else if content_type == "b" {
//       return batch::<C>(&body).await;
//     }
//   }
//   res(StatusCode::BAD_REQUEST, "miss content-type")
// }
