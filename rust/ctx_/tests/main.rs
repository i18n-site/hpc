use axum::{body::Body, extract::Request};
use aok::{OK, Result};
use tokio::time::{Duration, sleep};
use ctx_::{Cookie, Ctx};
use http::Method;
use tracing::info;

#[static_init::constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  let request = Request::builder()
    .method(Method::POST)
    .uri("/my/path")
    .header("cookie", "session_id=12345; user_id=67890")
    .body(Body::empty())?;

  {
    let req: Ctx = request.into();
    let req = &req;

    async {
      sleep(Duration::from_secs(2)).await;
      let cookie: &Cookie = ctx_::sync::get(req);
      info!("{}", cookie);
    }
    .await;

    async {
      sleep(Duration::from_secs(1)).await;
      let cookie: &Cookie = ctx_::sync::get(req);
      info!("{}", cookie);
    }
    .await;
  }
  info!("done");
  OK
}
