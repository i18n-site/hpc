#![feature(let_chains)]

use std::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};

use set_cookie::SET_COOKIE;
use header_host::header_host;
use tower::{Layer, Service};
use rand::{RngCore, SeedableRng, rngs::StdRng};
use axum::{
  body::Body,
  http::{Request, Response},
};

#[derive(Clone)]
pub struct BrowserIdLayer;

fn gen_browser_bin() -> [u8; 16] {
  let mut rng = StdRng::from_rng(&mut rand::rng());
  let mut bytes = [0u8; 16];
  rng.fill_bytes(&mut bytes);
  bytes
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

#[derive(Clone)]
pub struct Browser {
  pub bin: [u8; 16],
  pub renew: bool,
}

const COOKIE_REFRESH: &str = "r=";

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
    let headers = request.headers();
    let cookie_li = headers
      .get("cookie")
      .and_then(|cookie| cookie.to_str().ok())
      .unwrap_or("")
      .split(";");

    let mut no_refresh = true;
    let mut cookie_browser_bin = None;

    for i in cookie_li {
      let i = i.trim_start();
      if let Some(b) = i.strip_prefix("b=") {
        cookie_browser_bin = Some(b.to_owned());
      } else if i == COOKIE_REFRESH {
        no_refresh = false;
        if cookie_browser_bin.is_some() {
          break;
        }
      }
    }

    let no_browser_bin = cookie_browser_bin.is_none();
    let browser_bin;

    #[allow(clippy::never_loop)]
    loop {
      if let Some(cookie_browser_bin) = cookie_browser_bin
        && let Ok(id) = ub64::b64d(&cookie_browser_bin)
        && let Ok::<[u8; 16], _>(id) = id.try_into()
      {
        browser_bin = id;
        break;
      }
      browser_bin = gen_browser_bin();
      break;
    }

    let host = if (no_browser_bin || no_refresh)
      && let Ok(host) = xerr::ok!(header_host(headers))
    {
      Some(host.to_owned())
    } else {
      None
    };

    request.extensions_mut().insert(Browser {
      bin: browser_bin,
      renew: no_refresh && !no_browser_bin,
    });

    let future = self.inner.call(request);

    if let Some(host) = host {
      Box::pin(async move {
        let mut response = future.await?;
        let headers = response.headers_mut();
        let cookie = set_cookie::new(xtld::host_tld(host));
        // r如果没了就自动给b续期，防止b过期消失(chrome的cookie最长有效期400天)
        for val in [
          cookie.set_max("b", ub64::b64e(browser_bin)),
          cookie.set("r", "", 999999),
        ] {
          if let Ok(val) = xerr::ok!(val.parse()) {
            headers.append(SET_COOKIE, val);
          }
        }

        Ok(response)
      })
    } else {
      Box::pin(future)
    }
  }
}
