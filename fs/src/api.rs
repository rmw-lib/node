use crate::udp::cmd::{Cmd, Get};
use crate::udp::r#type::CmdV4;
use anyhow::Result;
use async_std::channel;

use async_std::net::SocketAddr;
use async_std::net::ToSocketAddrs;
use log::error;
use rmw_utf8::encode;
use std::path::PathBuf;

type Sender = channel::Sender<CmdV4>;

pub struct Api {
  pub sender: Sender,
}

impl Api {
  pub async fn send<IntoCmd: Into<Cmd>, Addr: std::fmt::Debug + ToSocketAddrs>(
    &self,
    addr: Addr,
    cmd: IntoCmd,
    msg: &[u8],
  ) {
    let cmd: Cmd = cmd.into();
    match addr.to_socket_addrs().await {
      Ok(li) => {
        for addr in li {
          match addr {
            SocketAddr::V4(addr) => {
              self
                .sender
                .send((addr, cmd.clone(), msg.into()))
                .await
                .unwrap();
            }
            _ => {}
          }
        }
      }
      Err(err) => error!("{}", err),
    }
  }
}

impl From<Sender> for Api {
  fn from(sender: Sender) -> Api {
    Api { sender }
  }
}

impl Api {
  pub async fn get<IntoPathBuf: Into<PathBuf>, Addr: std::fmt::Debug + ToSocketAddrs>(
    &self,
    addr: Addr,
    url: &[u8],
    out: IntoPathBuf,
  ) -> Result<()> {
    let url = encode(url);
    self.send(addr, Get(out.into()), &url).await;
    Ok(())
  }
}
