use http::HeaderMap;
use xbin::concat;
use xkv::{R, fred::interfaces::KeysInterface};

pub async fn captcha_verify(headers: &HeaderMap) -> bool {
  if let Some(id) = headers.get("c") {
    if let Ok(id) = ub64::b64d(id) {
      let key = concat!(b"captcha:", id);
      if let Ok(Some(r)) = xerr::ok!(R.get::<Option<Vec<u8>>, _>(key).await) {
        xerr::log!(R.del::<(), _>(key).await);
        if r.is_empty() {
          return true;
        }
      }
    }
  }
  false
}
