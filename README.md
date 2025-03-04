# 相关仓库

[css](https://github.com/i18n-site/css) [deno](https://github.com/i18n-site/deno) [hpc](https://github.com/i18n-site/hpc) [mod](https://github.com/i18n-site/mod) [pug](https://github.com/i18n-site/pug) [srv](https://github.com/i18n-site/srv) [state](https://github.com/i18n-site/state) [web](https://github.com/i18n-site/web)

# hpc

hpc ( http protobuf call ) : 基于 http 和 protobuf 3 的远程调用协议 ( 开发中)。

目前处于概念原型演示阶段，可以运行示例，支持更多类型还需要进一步完善。

如果有任何问题，欢迎在此邮件列表中讨论： https://groups.google.com/u/1/g/hpcsrv

## 序言

协议设计的初衷是基于后端`rust`代码自动生成前端`js`函数。

比如 `rust` 代码有如下函数：

```rust
pub struct User {
  pub id: u64,
  pub name: String,
  pub ico: Box<[u8]>,
}

pub async fn info(id: u64) -> Result<User> {
  … …
}
```

那么将生成的前端 `js` 函数如下:

```
export const info = async (id) => {
  … …
  return [
    id,
    name,
    ico
  ]
}
```

同时支持在一个请求中批量调用多个函数，比如:

```js
const [
  func1_result,
  func2_result,
  func3_result,
] = await batch(() => {
  func1(arg1, arg2);
  func2(arg);
  func3();
});
```

## 优化 protobuf 生成的 js 代码

前后端协议是基于http的protobuf。

我基于 [protoscript](https://www.npmjs.com/package/protoscript) 二次开发了
[@3-/protoscript](https://www.npmjs.com/package/@3-/protoscript)，优化 `.proto`
文件生成的 js 代码。

### protobuf 3 生成的 js 文件大小: 减少 76%

在演示项目
[github/i18n-demo/protobuf-minify](https://github.com/i18n-demo/protobuf-minify?tab=readme-ov-file)
中，我对比了
[_.proto](https://github.com/i18n-demo/protobuf-minify/blob/main/proto/_.proto)
生成的 js 打包压缩后的体积(减少了 76%):

| Filename                                                         | Size (bytes) | Zstd (bytes, %) | Brotli (bytes, %) | Gzip (bytes, %) |
| ---------------------------------------------------------------- | ------------ | --------------- | ----------------- | --------------- |
| [protoscript](https://www.npmjs.com/package/protoscript)         | 38445        | 6989 (449.74%)  | 6605 (425.03%)    | 7400 (476.19%)  |
| [@3-/protoscript](https://www.npmjs.com/package/@3-/protoscript) | 4120         | 1651 (106.24%)  | 1554 (100.00%)    | 1688 (108.62%)  |

虽然格式解析js引入了几KB开销，但
[protobuf数据gz压缩后的大小约为json的70%](https://nilsmagnus.github.io/post/proto-json-sizes/)，这种开销会在数据传输中节省回来。

## 演示项目

### 代码库

- [hpc](https://github.com/i18n-site/hpc) 代码生成器
- [mod](https://github.com/i18n-site/mod) 网站的功能模块
- [hpc_srv](https://github.com/i18n-site/hpc_srv) 从功能模块生成http服务

### 开发环境准备

1. 安装 [mise](https://github.com/jdx/mise) ，编辑 `~/.zshrc` 或者 `~/.bashrc`
   配置命令行环境
1. 并进入目录，运行 `mise trust`
1. 安装 [bun](https://github.com/oven-sh/bun)
1. 安装 [rust](https://www.rust-lang.org/tools/install)

Linux 请安装 [mold](https://github.com/rui314/mold) 到 `/usr/bin/mold`

### 克隆代码库

```
mkdir -p demo && cd demo

clone(){
  git clone --depth 1 git@github.com:i18n-site/$1.git
  cd $1
  mise trust
  [ -f package.json ] && bun i
  cd ..
}

clone hpc
clone mod
clone hpc_srv
cd hpc_srv/gen/js
bun i
cd ../..
```

### 运行数据库

运行演示需要一个数据库，可以用 [kvrocks](https://github.com/apache/kvrocks) 或者
redis (任选其一) ，用如下命令启动:

#### 启动 kvrocks

```
docker run -d --rm \
  --name xkv_test_db \
  -p 6666:6666 apache/kvrocks \
  --bind 0.0.0.0 \
  --resp3-enabled yes --requirepass xxx
```

#### 启动 redis

```
docker run -d --rm \
  --name xkv_test_db \
  -e REDIS_PASSWORD=xxx \
  -p 6666:6379 \
  redis
```

#### 配置环境变量

创建 `hpc_srv/conf/r.sh` , 内容类似下面:

```
R_PORT=6666
R_NODE="127.0.0.1:$R_PORT"
R_PASSWORD=xxx
R_RESP=3
R_USER=
```

### 生成文件

```
cd hpc_srv
./rust2proto.local.sh
```

### 运行服务器

`hpc_srv` 目录下运行

```
./hpc_srv.sh
```

### 运行测试脚本

`hpc_srv` 目录下运行

```
./gen/js/test.coffee
```

### 生成的代码

代码根据 `hpc_srv/mod/src/lib.rs` 暴露的包生成，其代码如下:

```rs
pub mod auth {
  pub use auth_::*;
}

pub mod captcha {
  pub use captcha_::captcha;
}
```

这里 `pub mod auth` 会生成 `hpc_srv/gen/proto/auth.proto` 如下:

```
/**
 * ## auth_
 * - signup_mail(SignupMailArgs)
 * - signup_mail_verify(SignupMailVerifyArgs) → SignupMailVerify
 */

syntax = "proto3";
package auth;

message SignupMailArgs {
  string address = 1;
  string password = 2;
}

message SignupMailVerifyArgs {
  string address = 1;
  string code = 2;
}

enum SignupMailVerify {
  OK = 0;
  INVALID_ADDRESS = 1;
  PASSWORD_TOO_SHORT = 2;
}
```

生成的js函数如下, `gen/_hpc.rs`

```
/// AUTO GEN BY rust2proto , DO NOT EDIT

use aok::{anyhow, Result};
use pb::Func;
use pb_jelly::Message;
use r#mod::*;

pub struct Hpc;

impl hpc::Hpc for Hpc {
  type Func = Func;
  async fn run(func: Func, args: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let args = args.as_ref();

    Ok(match func {
      Func::None => return Ok(vec![]),

      Func::AuthSignupMail => {
        let args = pb::auth::SignupMailArgs::deserialize_from_slice(args)?;
        auth::signup_mail(&args.address,&args.password).await?;
        vec![]
      }
      Func::AuthSignupMailVerify => {
        let args = pb::auth::SignupMailVerifyArgs::deserialize_from_slice(args)?;
        let r = auth::signup_mail_verify(&args.address,&args.code).await?;
        match (r as i32).try_into() {
          Ok::<pb::auth::SignupMailVerify, _>(r) => r.serialize_to_vec(),
          Err(err) => return Err(anyhow!(format!("pb::auth::SignupMailVerify Invalid: {err}"))),
        }
      }

      Func::Captcha => {
        let r = captcha::captcha().await?;
        pb::captcha::Captcha {
          id: r.id.into(),
          img: r.img.into(),
          tip: r.tip.into()
        }.serialize_to_vec()
      }

    })
  }
}
```

同时生成调用的`hpc_srv/gen/js/api.js`如下

```
import {
  T3Encode,
  T4Decode,T5Decode,
  T0Encode, // CallEncode,
  T1Encode, //CallLiEncode,
  T2Decode // BinLiDecode
} from './_.pb.js'

import ireq from '@3-/ireq'

const [
  _setUrl,
  _reqNoArgs,
  _req,
  _batch
] = ireq(
  // err catch
  (code, res)=>{
    console.log('TODO err catch', code)
  },
  T0Encode, T1Encode, T2Decode,
)

export const batch = _batch;
export const setUrl = _setUrl;

export const authSignupMail = (address /* str */,password /* str */)=>_req(1,T3Encode([address,password]))
export const authSignupMailVerify = (address /* str */,code /* str */)=>_req(2,T3Encode([address,code]),T4Decode/* SIGNUP_MAIL_VERIFY:enum */)
export const captcha = _reqNoArgs(3,T5Decode/* id:Box<[u8]>,img:Box<[u8]>,tip:Box<[u8]> */)
```

## rust 编码技巧

参考 [xkv](https://crates.io/crates/xkv)
可以创建一个全局的数据库连接，代码示例如下:

```rust
use aok::{Result, OK};
use tracing::info;
use xkv::{fred::interfaces::KeysInterface, R};

async fn test_redis() -> Result<()> {
  let key = "xkvtest1";
  let val = "abc";

  R!(del key);

  let v: bool = R.exists(key).await?;
  assert_eq!(false, v);

  let v: Option<String> = R.get(key).await?;
  info!("get {key} = {:?}", v);
  assert_eq!(v, None);

  R!(set key, val, None, None, false);

  let v: Option<String> = R.get(key).await?;
  info!("get {key} = {:?}", v);
  assert_eq!(v, Some(val.into()));

  R!(del key);

  OK
}

#[tokio::main]
async fn main() -> Result<()> {
  // 仅在程序启动的main函数中调用一次
  static_::init().await?;
  test_redis().await?;
  OK
}
```
