use std::future::Future;

use hpc_captcha::{Captcha, GenCaptcha};
use ih::CodeBody;
use pb_jelly::ClosedProtoEnum;
use ctx_::Ctx;

pub trait Hpc {
  type Func: TryFrom<u32> + ClosedProtoEnum + Copy;

  fn run<G: GenCaptcha>(
    req: &Ctx,
    func: Self::Func,
    args: &[u8],
    captcha: &Captcha<G>,
  ) -> impl Future<Output = crate::Result<CodeBody>>;

  fn run_with_log<G: GenCaptcha>(
    req: &Ctx,
    func: Self::Func,
    args: &[u8],
    captcha: &Captcha<G>,
  ) -> impl Future<Output = CodeBody> {
    async move {
      let cost = cost_time::start();
      let r = match Self::run(req, func, args, captcha).await {
        Ok(r) => r,
        Err(err) => err,
      };
      let name = func.name();
      let cost = cost.sec();
      println!("{name} {cost}s");
      r
    }
  }
}
