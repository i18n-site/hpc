use std::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};

use tower::{Layer, Service};
use rand::{RngCore, SeedableRng, rngs::StdRng};
use axum::{
  body::Body,
  http::{Request, Response},
};

#[derive(Clone)]
pub struct BrowserIdLayer;

pub fn browser_id_layer() -> BrowserIdLayer {
  BrowserIdLayer::new()
}

impl Default for BrowserIdLayer {
  fn default() -> Self {
    Self::new()
  }
}

impl BrowserIdLayer {
  pub fn new() -> Self {
    Self
  }
}

fn browser_id() -> String {
  let mut rng = StdRng::from_rng(&mut rand::rng());
  let mut bytes = [0u8; 16];
  rng.fill_bytes(&mut bytes);
  ub64::b64e(bytes)
}

impl<S> Layer<S> for BrowserIdLayer {
  type Service = BrowserIdService<S>;

  fn layer(&self, service: S) -> Self::Service {
    BrowserIdService { inner: service }
  }
}

#[derive(Clone)]
pub struct BrowserIdService<S> {
  inner: S,
}

impl<S, B> Service<Request<B>> for BrowserIdService<S>
where
  S: Service<Request<B>, Response = Response<Body>> + Send + 'static,
  S::Future: Send + 'static,
  B: Send + 'static,
{
  type Response = S::Response;
  type Error = S::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(cx)
  }

  fn call(&mut self, mut request: Request<B>) -> Self::Future {
    let has_browser_id = request
      .headers()
      .get("cookie")
      .and_then(|cookie| cookie.to_str().ok())
      .map(|cookie| cookie.contains("b="))
      .unwrap_or(false);

    let browser_id = if !has_browser_id {
      let id = browser_id();
      let cookie = format!("b={}", id);
      request
        .headers_mut()
        .insert("cookie", cookie.parse().unwrap());
      Some(id)
    } else {
      None
    };

    let future = self.inner.call(request);

    Box::pin(async move {
      let mut response = future.await?;
      if let Some(id) = browser_id {
        let cookie = format!("b={}; Path=/; HttpOnly", id);
        response
          .headers_mut()
          .insert("set-cookie", cookie.parse().unwrap());
      }
      Ok(response)
    })
  }
}
