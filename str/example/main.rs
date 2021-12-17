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
