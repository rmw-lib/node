<!-- 本文件由 ./readme.make.md 自动生成，请不要直接修改此文件 -->

# rmw-upnp

upnp port map daemon

## use example

```
use rmw_upnp::daemon;
use std::process::exit;

#[async_std::main]
async fn main() {
  let port = 23087;
  let duration = 60;
  let r = daemon("rmw upnp test".to_string(), port, duration);
  loop {
    println!("{:?}", r.recv().unwrap());
    exit(0);
  }
}
```

output as below

```
Ok((23087, 172.16.0.123, 172.16.0.1:1900))
```

## 关于

本项目隶属于**人民网络([rmw.link](//rmw.link))** 代码计划。

![人民网络](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)