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
