use std::future::Future;

use hpc_captcha::{Captcha, GenCaptcha};
use icall::CodeBody;
use pb_jelly::ClosedProtoEnum;
use req_::Req;

pub trait Hpc {
  type Func: TryFrom<u32> + ClosedProtoEnum + Copy;

  fn run<G: GenCaptcha>(
    req: &Req,
    func: Self::Func,
    args: &[u8],
    captcha: &Captcha<G>,
  ) -> impl Future<Output = CodeBody> + Send;

  fn run_with_log<G: GenCaptcha>(
    req: &Req,
    func: Self::Func,
    args: &[u8],
    captcha: &Captcha<G>,
  ) -> impl Future<Output = CodeBody> {
    async move {
      let cost = cost_time::start();
      let r = Self::run(req, func, args, captcha).await;
      let name = func.name();
      let cost = cost.sec();
      println!("{name} {cost}s");
      r
    }
  }
}
