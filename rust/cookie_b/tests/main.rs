use aok::Result;
use axum::{Router, body::Body, routing::get};
use cookie_b::browser_id_layer;
use http::{Request, header};
use tower::ServiceExt;

#[static_init::constructor(0)]
extern "C" fn init() {
  loginit::init()
}

async fn handler() -> &'static str {
  "Hello, World!"
}

#[tokio::test]
async fn test_browser_id_middleware() -> Result<()> {
  let app = Router::new()
    .route("/", get(handler))
    .layer(browser_id_layer());

  // 首次请求，应该设置新的 browser_id cookie
  let req = Request::builder().uri("/").body(Body::empty())?;
  let res = app.clone().oneshot(req).await?;

  let cookie = res
    .headers()
    .get(header::SET_COOKIE)
    .expect("应该设置 cookie");
  let cookie_str = cookie.to_str()?;
  assert!(cookie_str.starts_with("b="));
  assert!(cookie_str.contains("Path=/"));
  assert!(cookie_str.contains("HttpOnly"));

  dbg!(cookie_str);

  // 提取 browser_id
  let browser_id = cookie_str
    .split(';')
    .next()
    .unwrap()
    .trim_start_matches("b=");

  // 第二次请求，使用已有的 browser_id
  let req = Request::builder()
    .uri("/")
    .header(header::COOKIE, format!("b={}", browser_id))
    .body(Body::empty())?;
  let res = app.oneshot(req).await?;

  // 验证第二次请求没有设置新的 cookie
  assert!(res.headers().get(header::SET_COOKIE).is_none());

  Ok(())
}
