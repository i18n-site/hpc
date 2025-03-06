use aok::{OK, Result};
use ih::State;
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  info!("{}", State::CAPTCHA);
  OK
}
