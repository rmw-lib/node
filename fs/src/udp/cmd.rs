use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum CmdU8 {
  Get,
  Data,
  Time,
  Meta,
}

macro_rules! IntoCmd {
  ($cls:ident) => {
    impl From<$cls> for Cmd {
      fn from(i: $cls) -> Cmd {
        Cmd::$cls(i)
      }
    }
  };
}

#[derive(Clone, Debug)]
pub struct Get(pub PathBuf);

#[derive(Clone, Debug)]
pub enum Cmd {
  Get(Get),
}

IntoCmd!(Get);
