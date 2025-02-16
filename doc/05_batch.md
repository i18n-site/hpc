# 批量调用

```
Captcha 0.373s
Captcha 0.336s
0.373s
PUT / 20
```

因为是并发执行，总耗时是耗时最长函数的耗时

## auto batch

类redis客户端的auto pipeline
