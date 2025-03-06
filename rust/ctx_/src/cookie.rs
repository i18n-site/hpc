use std::{collections::HashMap, fmt, ops::Deref};

use crate::{Ctx, sync::Extract};

pub struct Cookie(HashMap<String, String>);

impl Deref for Cookie {
  type Target = HashMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl fmt::Display for Cookie {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "cookie: {:?}", self.0)
  }
}

impl fmt::Debug for Cookie {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(self, f)
  }
}

impl Extract for Cookie {
  fn from_ctx(ctx: &Ctx) -> Self {
    use cookie::Cookie;

    let mut inner = HashMap::new();

    if let Some(cookie) = ctx.req.headers.get("cookie") {
      // dbg!(cookie);
      if let Ok(cookie) = xerr::ok!(cookie.to_str()) {
        for cookie in Cookie::split_parse(cookie) {
          if let Ok(cookie) = xerr::ok!(cookie) {
            let (name, value) = cookie.name_value();
            inner.insert(name.to_owned(), value.to_owned());
          }
        }
      }
    }
    Self(inner)
  }
}

// bumpalo not call drop
// impl Drop for Cookie {
//   fn drop(&mut self) {
//     info!("drop cookie");
//   }
// }
