<!-- 本文件由 ./readme.make.md 自动生成，请不要直接修改此文件 -->

# rmw-str

to str and from str

## use example

```
use anyhow::Result;
use rmw_str::Str;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

fn main() -> Result<()> {
  let x: u16 = 12345;
  let str = x.encode();
  dbg!(u16::decode(&str)?, str);

  let x: bool = false;
  let str = x.encode();
  dbg!(bool::decode(&str)?, str);

  let x = SocketAddrV6::new(
    Ipv6Addr::new(1, 2, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf),
    8080,
    0,
    0,
  );
  let str = x.encode();
  dbg!(SocketAddrV6::decode(&str)?, str);

  let x = SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 8080);
  let str = x.encode();
  dbg!(SocketAddrV4::decode(&str)?, str);

  Ok(())
}
```

output as below

```

```

## 关于

本项目隶属于**人民网络([rmw.link](//rmw.link))** 代码计划。

![人民网络](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)