http only 的 cookie

b 浏览器id
r refresh 这个过期时间短 每7天过期一次 如果没有，都重新设置一次b防止浏览器id过期
这个做成全局的中间件 一直检查


u 用户id cookie
  然后 hpc 用js读取请求头的content_type 如果是 # 表示未登录， 否则就是用户id

  cookie -> sessionStorage 这样刷新还是同一个用户

  这样的好处是可以同时登录多个不同用户

不用 localstorage 的原因是为了子域名也能登录


