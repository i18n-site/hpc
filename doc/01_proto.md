# hpc ( http protobuf call ) : 基于 http 和 protobuf 3 的远程调用协议 ( 开发中)

这是一个正在开发中的项目，我将在此分享一些我的开发进度。

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
] = await batch(()=>{
  func1(arg1,arg2)
  func2(arg)
  func3()
})
```

## 第一步: 简化 protobuf 生成的 js 代码

### 体积对比

在演示项目 [github/i18n-demo/protobuf-minify](https://github.com/i18n-demo/protobuf-minify?tab=readme-ov-file) 中 ，我对比了 [_.proto](https://github.com/i18n-demo/protobuf-minify/blob/main/proto/_.proto) 生成的 js 打包压缩后的体积:

| Filename | Size (bytes) | Zstd (bytes, %) | Brotli (bytes, %) | Gzip (bytes, %) |
|----------|--------------|-----------------|-------------------|-----------------|
| [protoscript](https://www.npmjs.com/package/protoscript) | 38445 | 6989 (449.74%) | 6605 (425.03%) | 7400 (476.19%) |
| [@3-/protoscript](https://www.npmjs.com/package/@3-/protoscript) | 4120 | 1651 (106.24%) | 1554 (100.00%) | 1688 (108.62%) |

### 摇树优化

#### 编码和解码函数的拆分

#### js类的数组化

#### protobuf结构体的数组化

##### protobuf中单字段结构体的优化

### 函数调用压缩

`console.error`

* [esbuild 不支持空函数压缩](https://github.com/evanw/esbuild/issues/290#issuecomment-665420487)
* [terser 可以删除空函数调用](https://github.com/terser/terser/issues/263#issuecomment-462577953)
