use aok::Result;
use xbin::concat;
use cookie_b::Browser;

use crate::{Ctx, Extract};

pub struct User {
  pub id: u64,
  pub bin: Box<[u8]>,
}

impl Extract for User {
  async fn from_ctx(ctx: &Ctx) -> Result<Self> {
    let bin: Box<[u8]>;
    let id;

    let req = &ctx.req;

    if let Some(uid) = req.headers().get("content-type")
      && let Ok(uid) = uid.to_str()
      && uid != "#"
      && let Ok(uid_bin) = ub64::b64d(uid)
      && let Some(browser) = req.extensions().get::<Browser>()
    {
      bin = uid_bin.into();
      id = intbin::bin_u64(&bin);
      let key = concat!(b"u:", bin);
    } else {
      bin = Box::new([]);
      id = 0;
    };
    Ok(Self { id, bin })
  }
}
