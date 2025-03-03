# cookie_b

```rust
#![feature(let_chains)]

use std::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};

use header_host::header_host;
use tower::{Layer, Service};
use rand::{RngCore, SeedableRng, rngs::StdRng};
use axum::{
  body::Body,
  http::{HeaderMap, Request, Response},
};

#[derive(Clone)]
pub struct BrowserIdLayer;

fn gen_browser_id() -> [u8; 16] {
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
  pub id: [u8; 16],
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
    let mut cookie_browser_id = None;

    for i in cookie_li {
      if let Some(b) = i.strip_prefix("b=") {
        cookie_browser_id = Some(b.to_owned());
      } else if i == COOKIE_REFRESH {
        no_refresh = false;
      }
    }

    let no_browser_id = cookie_browser_id.is_none();
    let browser_id;

    #[allow(clippy::never_loop)]
    loop {
      if let Some(cookie_browser_id) = cookie_browser_id
        && let Ok(id) = ub64::b64d(&cookie_browser_id)
        && let Ok::<[u8; 16], _>(id) = id.try_into()
      {
        browser_id = id;
        break;
      }
      browser_id = gen_browser_id();
      break;
    }

    let host = if (no_browser_id || no_refresh)
      && let Ok(host) = xerr::ok!(header_host(headers))
    {
      Some(host.to_owned())
    } else {
      None
    };

    request.extensions_mut().insert(Browser { id: browser_id });

    let future = self.inner.call(request);

    if let Some(host) = host {
      Box::pin(async move {
        let mut response = future.await?;
        let mut cookie = Cookie {
          domain: xtld::host_tld(host),
          headers: response.headers_mut(),
        };
        // r如果没了就自动给b续期，防止b过期消失(chrome的cookie最长有效期400天)
        cookie.set("b", ub64::b64e(browser_id), 99999999);
        cookie.set("r", "", 999999);
        Ok(response)
      })
    } else {
      Box::pin(future)
    }
  }
}

pub struct Cookie<'a> {
  pub domain: String,
  pub headers: &'a mut HeaderMap,
}

impl Cookie<'_> {
  fn set(&mut self, key: impl AsRef<str>, val: impl AsRef<str>, max_age: u64) {
    let key = key.as_ref();
    let val = val.as_ref();
    let cookie = format!(
      "{key}={val};Max-Age={max_age};Domain={};Secure;HttpOnly;Path=/;Partitioned",
      self.domain
    );
    if let Ok(cookie) = cookie.parse() {
      self.headers.append("set-cookie", cookie);
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
