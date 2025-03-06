use axum::{
  body::Body,
  extract::Request,
  http::{HeaderValue, Method, header},
  middleware::Next,
  response::IntoResponse,
};

pub async fn set(req: Request<Body>, next: Next) -> impl IntoResponse {
  let origin = req
    .headers()
    .get(header::ORIGIN)
    .map(|i| i.to_str().map(|s| s.to_owned()));

  let is_options = req.method() == Method::OPTIONS;
  let mut res = if is_options {
    let mut res = "".into_response();
    res.headers_mut().insert(
      header::ACCESS_CONTROL_MAX_AGE,
      HeaderValue::from_static("9999999"),
    );
    res
  } else {
    next.run(req).await
  };

  let headers = res.headers_mut();

  headers.remove(header::CONTENT_TYPE);

  if let Some(Ok(origin)) = origin {
    headers.insert(
      header::ACCESS_CONTROL_ALLOW_ORIGIN,
      HeaderValue::from_str(&origin).unwrap(),
    );
  }

  headers.insert(
    header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
    HeaderValue::from_static("true"),
  );

  headers.insert(
    header::ACCESS_CONTROL_ALLOW_METHODS,
    HeaderValue::from_static("PUT"),
  );

  headers.insert(
    header::ACCESS_CONTROL_ALLOW_HEADERS,
    HeaderValue::from_static("content-type,c"),
  );

  res
}
