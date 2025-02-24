use std::any::TypeId;

use crate::Req;

pub trait Extract: Sized {
  fn from_req(req: &super::Req) -> Self;
}

pub fn get<T: Extract + 'static>(req: &Req) -> &T {
  let ptr = *req.cache.entry(TypeId::of::<T>()).or_insert_with(|| {
    let bump = req.bump.lock();
    let ptr = bump.alloc(T::from_req(req));
    ptr as *const T as usize
  });

  unsafe { &*(ptr as *const T) }
}
