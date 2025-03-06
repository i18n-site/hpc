# ih : interface proto for hpc

```rust
syntax = "proto3";

message CallLi
{
  repeated uint32 func_li = 1;
  repeated bytes args_li = 2;
}

message BinLi
{
  repeated uint32 state_li = 1;
  repeated bytes bin_li = 2;
}

enum State {
  OK = 0;

  JSON = 1;
  CODE = 2;
  CODE_LI = 3;
  BIN = 4;

  CAPTCHA = 10;
  NEED_SIGNIN = 11;
  NO_PERMISSION = 12;

  MISS_FUNC = 100;
  ARGS_INVALID = 101;
  BATCH_LIMIT = 102;
  CALL_ERROR = 103;
  MIDDLEWARE_ERROR = 104;
}

message Code
{
  uint32 inner = 1;
}

message CodeLi
{
  repeated uint32 li = 1;
}

message Json
{
  string inner = 1;
}

message Bin
{
  bytes inner = 1;
}

message Captcha
{
  bytes id = 1;
  bytes img = 2;
  bytes tip = 3;
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
