<!-- 本文件由 ./readme.make.md 自动生成，请不要直接修改此文件 -->

# rmw-bytes

to bytes and from bytes

## use example

```
use anyhow::Result;
use rmw_bytes::Bytes;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

fn main() -> Result<()> {
  let x: u16 = 12345;
  let bytes = x.encode();
  dbg!(u16::decode(&bytes)?, bytes);

  let x: bool = false;
  let bytes = x.encode();
  dbg!(bool::decode(&bytes)?, bytes);

  let x = SocketAddrV6::new(
    Ipv6Addr::new(1, 2, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf),
    8080,
    0,
    0,
  );
  let bytes = x.encode();
  dbg!(SocketAddrV6::decode(&bytes)?, bytes);

  let x = SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 8080);
  let bytes = x.encode();
  dbg!(SocketAddrV4::decode(&bytes)?, bytes);

  Ok(())
}
```

output as below

```

```

## 关于

本项目隶属于**人民网络([rmw.link](//rmw.link))** 代码计划。

![人民网络](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)