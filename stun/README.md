<!-- 本文件由 ./readme.make.md 自动生成，请不要直接修改此文件 -->

# rmw-stun

get external ip port via stun server

## use example

```
use async_std::net::UdpSocket;
use rmw_stun::external_addr;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Instant};

#[async_std::main]
async fn main() {
  let udp = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0))
    .await
    .unwrap();
  //let server = "not.exist.stun:3478";
  let server = "stun.cablenet-as.net:3478";
  let timeout = 3;
  let start = Instant::now();
  let addr = external_addr(&udp, server, timeout).await;
  let duration = start.elapsed();

  println!("external addr {:?} cost {:?}", addr, duration);
}
```

output as below

```
external addr Ok(54.177.127.37:40143) cost 402.976326ms
```

## 关于

本项目隶属于**人民网络([rmw.link](//rmw.link))** 代码计划。

![人民网络](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)