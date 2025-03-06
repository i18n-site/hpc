use aok::Result;
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn init() {
  loginit::init()
}

// #[tokio::test]
// async fn test() -> Result<()> {
//   info!("{}", 123456);
//   OK
// }

#[test]
fn main() -> Result<()> {
  info!(">> test");
  Ok(())
}
