use crate::udp::cmd::Cmd;
use async_std::net::SocketAddrV4;

pub type CmdV4 = (SocketAddrV4, Cmd, Box<[u8]>);
