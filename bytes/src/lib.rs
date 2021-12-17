use anyhow::Result;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

pub trait Bytes {
  fn encode(&self) -> Box<[u8]>;
  fn decode(buf: &[u8]) -> Result<Self>
  where
    Self: Sized;
}

impl Bytes for bool {
  fn encode(&self) -> Box<[u8]> {
    ([if *self { '1' } else { '0' } as u8]).into()
  }

  fn decode(buf: &[u8]) -> Result<Self> {
    if let Some(i) = buf.iter().next() {
      if *i == b'0' {
        return Ok(false);
      }
      return Ok(true);
    }
    Ok(true)
  }
}

impl Bytes for u16 {
  fn encode(&self) -> Box<[u8]> {
    self.to_le_bytes().into()
  }

  fn decode(buf: &[u8]) -> Result<Self> {
    Ok(u16::from_le_bytes(buf[..2].try_into()?))
  }
}

impl Bytes for SocketAddrV4 {
  fn encode(&self) -> Box<[u8]> {
    let o = self.ip().octets();
    let p = self.port().to_le_bytes();
    [o[0], o[1], o[2], o[3], p[0], p[1]].into()
  }
  fn decode(buf: &[u8]) -> Result<Self> {
    let li: [u8; 6] = buf.try_into()?;

    Ok(Self::new(
      [li[0], li[1], li[2], li[3]].into(),
      u16::from_le_bytes([li[4], li[5]]),
    ))
  }
}

impl Bytes for SocketAddrV6 {
  fn encode(&self) -> Box<[u8]> {
    let o = self.ip().octets();
    let p = self.port().to_le_bytes();
    [
      o[0], o[1], o[2], o[3], o[4], o[5], o[6], o[7], o[8], o[9], o[10], o[11], o[12], o[13],
      o[14], o[15], p[0], p[1],
    ]
    .into()
  }
  fn decode(buf: &[u8]) -> Result<Self> {
    let o: [u8; 18] = buf.try_into()?;
    Ok(Self::new(
      [
        o[0], o[1], o[2], o[3], o[4], o[5], o[6], o[7], o[8], o[9], o[10], o[11], o[12], o[13],
        o[14], o[15],
      ]
      .into(),
      u16::from_le_bytes([o[16], o[17]]),
      0,
      0,
    ))
  }
}

impl Bytes for SocketAddr {
  fn encode(&self) -> Box<[u8]> {
    match self {
      SocketAddr::V4(addr) => addr.encode(),
      SocketAddr::V6(addr) => addr.encode(),
    }
  }
  fn decode(buf: &[u8]) -> Result<Self> {
    Ok(if buf.len() == 18 {
      SocketAddr::V6(SocketAddrV6::decode(buf)?)
    } else {
      SocketAddr::V4(SocketAddrV4::decode(buf)?)
    })
  }
}
