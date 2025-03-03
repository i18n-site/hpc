use aok::{OK, Result};
use http::{HeaderMap, HeaderName, HeaderValue};
use req_::{Cookie, Req};
use tokio::time::{Duration, sleep};
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let mut headers = HeaderMap::new();

  let header_array = [
    ("cookie", "session_id=12345; user_id=67890"),
    ("content-type", "application/json"),
  ];

  for (name, value) in &header_array {
    headers.insert(
      HeaderName::from_static(name),
      HeaderValue::from_static(value),
    );
  }

  {
    let req: Req = headers.into();
    let req = &req;

    async {
      sleep(Duration::from_secs(2)).await;
      let cookie: &Cookie = req_::sync::get(req);
      info!("{}", cookie);
    }
    .await;

    async {
      sleep(Duration::from_secs(1)).await;
      let cookie: &Cookie = req_::sync::get(req);
      info!("{}", cookie);
    }
    .await;
  }
  info!("done");
  OK
}
