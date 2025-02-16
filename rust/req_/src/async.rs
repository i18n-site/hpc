use std::any::TypeId;

use aok::Result;
use tokio::sync::OnceCell;

use crate::Req;

pub trait Extract: Sized {
  fn from_req(req: &Req) -> impl Future<Output = Result<Self>>;
}

pub async fn get<T: Extract + 'static>(req: &Req) -> Result<&T> {
  let ptr = *req
    .cache
    .entry(TypeId::of::<T>())
    .or_insert_with(|| req.bump.lock().alloc(OnceCell::<T>::new()) as *const OnceCell<_> as usize);

  // # Safety:
  /*
   * - 只要 `Req` 实例存在，`bump` 分配器管理的内存就有效。
   * - As long as `Req` instance exists, memory managed by Bump allocator is valid.
   */
  let cell = unsafe { &*(ptr as *const OnceCell<T>) };

  match cell.get_or_try_init(|| T::from_req(req)).await {
    Ok(r) => Ok(r),
    Err(e) => {
      tracing::error!("{:?}", req.headers);
      Err(e)
    }
  }
}
