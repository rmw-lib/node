use anyhow::Result;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

pub trait Str {
  fn encode(&self) -> Box<[u8]>;
  fn decode(val: &[u8]) -> Result<Self>
  where
    Self: Sized;
}

impl Str for Box<[u8]> {
  fn encode(&self) -> Box<[u8]> {
    self.clone()
  }
  fn decode(val: &[u8]) -> Result<Self> {
    Ok(val.into())
  }
}

impl Str for bool {
  fn encode(&self) -> Box<[u8]> {
    if *self { "1" } else { "0" }.as_bytes().into()
  }

  fn decode(val: &[u8]) -> Result<Self> {
    if val.len() > 0 {
      if val[0] == '0' as u8 {
        return Ok(false);
      }
      return Ok(true);
    }
    Ok(true)
  }
}

#[macro_export]
macro_rules! ImplStr {
  ($cls:ident) => {
    impl Str for $cls {
      fn encode(&self) -> Box<[u8]> {
        self.to_string().as_bytes().into()
      }

      fn decode(val: &[u8]) -> Result<Self> {
        Ok(String::from_utf8_lossy(val).parse()?)
      }
    }
  };
}

ImplStr!(i8);
ImplStr!(i16);
ImplStr!(i32);
ImplStr!(i64);
ImplStr!(isize);
ImplStr!(u8);
ImplStr!(u16);
ImplStr!(u32);
ImplStr!(u64);
ImplStr!(usize);
ImplStr!(SocketAddrV4);
ImplStr!(SocketAddrV6);
ImplStr!(SocketAddr);
ImplStr!(String);
