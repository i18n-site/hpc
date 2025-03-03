use aok::Result;
use axum::{Router, body::Body, routing::get};
use cookie_b::BrowserIdLayer;
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
  let app = Router::new().route("/", get(handler)).layer(BrowserIdLayer);

  // 首次请求，应该设置新的 browser_id cookie
  let req = Request::builder()
    .uri("/")
    .header("host", "127.0.0.1")
    .body(Body::empty())?;

  let res = app.clone().oneshot(req).await?;

  // 获取所有的 Set-Cookie 头
  let cookies: Vec<_> = res
    .headers()
    .get_all(header::SET_COOKIE)
    .iter()
    .filter_map(|v| v.to_str().ok())
    .collect();

  assert!(!cookies.is_empty(), "应该设置至少一个 cookie");

  // 提取 browser_id
  let browser_id = cookies[0]
    .split(';')
    .next()
    .unwrap()
    .trim_start_matches("b=");

  assert!(!browser_id.is_empty(), "browser_id 不能为空");

  // 第二次请求，使用已有的 browser_id
  let req = Request::builder()
    .uri("/")
    .header("host", "127.0.0.1") // 设置请求的host为127.0.0.1
    .header(header::COOKIE, format!("b={};r=", browser_id))
    .body(Body::empty())?;

  let res = app.oneshot(req).await?;

  // 验证第二次请求没有设置新的 cookie
  assert!(res.headers().get(header::SET_COOKIE).is_none());

  Ok(())
}
