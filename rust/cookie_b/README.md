# cookie_b

```rust
#![feature(let_chains)]

use std::{
  fmt::Debug,
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};

use axum::{
  body::Body,
  http::{Request, Response},
};
use rand::{RngCore, SeedableRng, rngs::StdRng};
use set_cookie::SET_COOKIE;
use tower::{Layer, Service};

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

impl Debug for Browser {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&format!(
      "Browser:{}(renew:{})",
      ub64::b64e(self.bin),
      self.renew
    ))
  }
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

    let mut cookie_u = None;

    for i in cookie_li {
      let i = i.trim_start();
      if let Some(b) = i.strip_prefix("b=") {
        cookie_browser_bin = Some(b.to_owned());
      } else if i == COOKIE_REFRESH {
        no_refresh = false;
      } else if let Some(i) = i.strip_prefix("u=") {
        cookie_u = Some(i.to_owned());
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
      && let Ok(host) = xerr::ok!(header_host::tld(headers))
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
        let cookie = set_cookie::new(host);
        // r如果没了就自动给b续期，防止b过期消失(chrome的cookie最长有效期400天)
        for val in [
          cookie.set_max("b", ub64::b64e(browser_bin)),
          cookie.set("r", "", 999999),
        ] {
          if let Ok(val) = xerr::ok!(val.parse()) {
            headers.append(SET_COOKIE, val);
          }
        }
        if let Some(cookie_u) = cookie_u {
          let cookie_u = cookie.set_max_for_js("u", cookie_u);
          if let Ok(cookie_u) = xerr::ok!(cookie_u.parse()) {
            headers.append(SET_COOKIE, cookie_u);
          }
        }

        Ok(response)
      })
    } else {
      Box::pin(future)
    }
  }
}
```

## About

This project is an open-source component of
[i18n.site ⋅ Internationalization Solution](https://i18n.site).

- [i18 : MarkDown Command Line Translation Tool](https://i18n.site/i18)

  The translation perfectly maintains the Markdown format.

  It recognizes file changes and only translates the modified files.

  The translated Markdown content is editable; if you modify the original text
  and translate it again, manually edited translations will not be overwritten
  (as long as the original text has not been changed).

- [i18n.site : MarkDown Multi-language Static Site Generator](https://i18n.site/i18n.site)

  Optimized for a better reading experience

## 关于

本项目为 [i18n.site ⋅ 国际化解决方案](https://i18n.site) 的开源组件。

- [i18 : MarkDown命令行翻译工具](https://i18n.site/i18)

  翻译能够完美保持 Markdown 的格式。能识别文件的修改，仅翻译有变动的文件。

  Markdown
  翻译内容可编辑；如果你修改原文并再次机器翻译，手动修改过的翻译不会被覆盖（如果这段原文没有被修改）。

- [i18n.site : MarkDown多语言静态站点生成器](https://i18n.site/i18n.site)
  为阅读体验而优化。
