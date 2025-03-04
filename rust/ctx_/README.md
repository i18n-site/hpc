# ctx_

Use arena memory allocation bumpalo

Extract message form headers only once for one req ( support async / sync )

使用 arena 内存分配器 bumpalo

一个请求只提取一次消息, 支持异步 / 同步


```rust
use aok::{OK, Result};
use tokio::time::{Duration, sleep};
use ctx_::{Cookie, Ctx};
use http::{Method, Request};
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
    .body(())?;

  let (request, _) = request.into_parts();

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
