use std::any::TypeId;

use crate::Ctx;

pub trait Extract: Sized {
  fn from_ctx(ctx: &super::Ctx) -> Self;
}

pub fn get<T: Extract + 'static>(ctx: &Ctx) -> &T {
  let ptr = *ctx.cache.entry(TypeId::of::<T>()).or_insert_with(|| {
    let bump = ctx.bump.lock();
    let ptr = bump.alloc(T::from_ctx(ctx));
    ptr as *const T as usize
  });

  unsafe { &*(ptr as *const T) }
}
