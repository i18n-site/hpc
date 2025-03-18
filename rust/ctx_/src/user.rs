use std::fmt::Debug;

use aok::{OK, Result};
use xkv::{
  R,
  fred::interfaces::{KeysInterface, SortedSetsInterface},
};
use xbin::concat;
use cookie_b::Browser;

use crate::{Ctx, Extract};

pub struct User {
  pub id: u64,
  pub bin: Box<[u8]>,
}

impl User {
  pub fn new(id: u64) -> Self {
    Self {
      id,
      bin: intbin::to_bin(id),
    }
  }
}

impl Debug for User {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("User:{}", self.id).as_str())
  }
}

/// https://developer.chrome.com/blog/cookie-max-age-expires?hl=zh-cn
pub const COOKIE_EXPIRE: i64 = 86400 * 400;

impl Extract for User {
  async fn from_ctx(ctx: &Ctx) -> Result<Self> {
    let bin: Box<[u8]>;
    let id;

    let req = &ctx.req;

    #[allow(clippy::never_loop)]
    loop {
      if let Some(uid) = req.headers.get("content-type")
        && let Ok(uid) = uid.to_str()
        && uid != "#"
        && let Ok(uid_bin) = ub64::b64d(uid)
        && let Some(browser) = req.extensions.get::<Browser>()
      {
        /*
          uid : ts

          ts 按上次登录时间时间排序
          ts 小于 0 为退出登录
        */
        let key = concat([b"bU:", &browser.bin[..]]);

        let score: Option<i64> = R.zscore(&key[..], &uid_bin[..]).await?;

        if let Some(score) = score
          && score > 0
        {
          bin = uid_bin.into();
          id = intbin::bin_u64(&bin);
          if browser.renew {
            tokio::spawn(async move {
              R!(expire & key[..], COOKIE_EXPIRE, None);
              OK
            });
          }
          break;
        }
      }
      bin = Box::new([]);
      id = 0;
      break;
    }

    Ok(Self { id, bin })
  }
}
