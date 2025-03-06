# 生成2套proto

## 优化 protobuf 3 生成的 js 文件大小

### 摇树优化

#### 编码和解码函数的拆分

#### js类的数组化

#### protobuf结构体的数组化

##### protobuf中单字段结构体的优化

### 函数调用压缩

`console.error`

- [esbuild 不支持空函数压缩](https://github.com/evanw/esbuild/issues/290#issuecomment-665420487)
- [terser 可以删除空函数调用](https://github.com/terser/terser/issues/263#issuecomment-462577953)

### 类型复用
