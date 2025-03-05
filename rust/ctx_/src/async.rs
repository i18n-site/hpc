use std::any::TypeId;

use ih::CodeBody;
use aok::Result;
use tokio::sync::OnceCell;

use crate::Ctx;

pub trait Extract: Sized {
  fn from_ctx(ctx: &Ctx) -> impl Future<Output = Result<Self>>;
}

pub async fn get<T: Extract + 'static>(ctx: &Ctx) -> Result<&T, CodeBody> {
  let ptr = *ctx
    .cache
    .entry(TypeId::of::<T>())
    .or_insert_with(|| ctx.bump.lock().alloc(OnceCell::<T>::new()) as *const OnceCell<_> as usize);

  // # Safety:
  /*
   * - 只要 `Ctx` 实例存在，`bump` 分配器管理的内存就有效。
   * - As long as `Ctx` instance exists, memory managed by Bump allocator is valid.
   */
  let cell = unsafe { &*(ptr as *const OnceCell<T>) };

  match cell.get_or_try_init(|| T::from_ctx(ctx)).await {
    Ok(r) => Ok(r),
    Err(e) => {
      tracing::error!("{:?}", ctx.req.headers);
      Err((ih::State::CALL_ERROR, e.to_string().as_bytes().into()))
    }
  }
}
