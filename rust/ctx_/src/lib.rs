#![feature(let_chains)]
#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]

use std::any::TypeId;

use bumpalo::Bump;
use dashmap::DashMap;
use parking_lot::Mutex;
use http::request::Parts;

pub struct Ctx {
  pub req: Parts,
  /*
   * 使用 Bump 分配器管理 req 生命周期内的内存。
   * Uses Bump allocator to manage memory within the req lifecycle.
   */
  pub bump: Mutex<Bump>,
  /*
   * 缓存提取器 (Extractor) 的结果。
   * 使用 TypeId 作为键，Bump 分配内存的指针作为值。
   * Caches Extractor results.
   * Uses TypeId as key and pointer to Bump-allocated memory as value.
   */
  pub cache: DashMap<TypeId, usize>,
}

impl Ctx {
  pub fn has<T: 'static>(&self) -> bool {
    self.cache.contains_key(&TypeId::of::<T>())
  }
}

impl From<Parts> for Ctx {
  fn from(req: Parts) -> Self {
    Self {
      req,
      bump: Default::default(),
      cache: DashMap::new(),
    }
  }
}

mod r#async;
pub use r#async::{Extract, get};
pub mod sync;

#[cfg(feature = "cookie")]
mod cookie;
#[cfg(feature = "cookie")]
pub use cookie::Cookie;

#[cfg(feature = "user")]
mod user;
#[cfg(feature = "user")]
pub use user::User;

#[cfg(feature = "set_header")]
mod set_header;
#[cfg(feature = "set_header")]
pub use set_header::{SET_COOKIE, SetHeader};

#[cfg(feature = "captcha")]
mod captcha;
#[cfg(feature = "captcha")]
pub use captcha::{Captcha, captcha};
