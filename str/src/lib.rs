use anyhow::Result;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

pub trait Str {
  fn encode(&self) -> String;
  fn decode(val: &str) -> Result<Self>
  where
    Self: Sized;
}

impl Str for bool {
  fn encode(&self) -> String {
    if *self { "1" } else { "0" }.into()
  }

  fn decode(val: &str) -> Result<Self> {
    if let Some(i) = val.chars().next() {
      if i == '0' {
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
      fn encode(&self) -> String {
        self.to_string()
      }

      fn decode(val: &str) -> Result<Self> {
        Ok(val.parse()?)
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
