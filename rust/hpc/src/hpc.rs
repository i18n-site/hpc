use std::future::Future;

use aok::Result;
use pb_jelly::ClosedProtoEnum;
use req_::Req;

pub trait Hpc {
  type Func: TryFrom<u32> + ClosedProtoEnum + Copy;

  fn run(req: &Req, func: Self::Func, args: &[u8]) -> impl Future<Output = Result<Vec<u8>>> + Send;

  #[allow(async_fn_in_trait)]
  async fn run_with_log(req: &Req, func: Self::Func, args: &[u8]) -> Result<Vec<u8>> {
    let cost = cost_time::start();
    let r = Self::run(req, func, args).await;
    let name = func.name();
    let cost = cost.sec();
    match r {
      Ok(r) => {
        println!("{name} {cost}s");
        Ok(r)
      }
      Err(err) => {
        eprintln!("❌ {name} {err} {cost}s : {:?}", args);
        Err(err)
      }
    }
  }
}
