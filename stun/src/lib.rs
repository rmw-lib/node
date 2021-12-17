use anyhow::Result;
use async_std::io;
use async_std::net::UdpSocket;
use std::mem::MaybeUninit;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use stun::agent::TransactionId;
use stun::message::{Getter, Message, BINDING_REQUEST};
use stun::xoraddr::XorMappedAddress;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("addr not exist")]
  AddrNotExist,
  #[error("timeout")]
  Timeout,
}

pub async fn external_addr<T: ToSocketAddrs>(
  udp: &UdpSocket,
  server: T,
  timeout: u64,
) -> Result<SocketAddr> {
  let mut bin = Message::new();
  bin.build(&[
    Box::new(TransactionId::default()),
    Box::new(BINDING_REQUEST),
  ])?;
  let bin = bin.marshal_binary()?;
  let mut li = server.to_socket_addrs()?;
  if let Some(addr) = li.next() {
    udp.send_to(&bin, addr).await?;
    match io::timeout(Duration::from_secs(timeout), async move {
      loop {
        let mut buf: [u8; 1472] = unsafe { MaybeUninit::uninit().assume_init() };
        if let Ok((n, peer)) = udp.recv_from(&mut buf).await {
          if peer == addr {
            let mut xor_addr = XorMappedAddress::default();
            let mut msg = Message::new();
            if msg.unmarshal_binary(&buf[..n]).is_ok() && xor_addr.get_from(&msg).is_ok() {
              return Ok(SocketAddr::new(xor_addr.ip, xor_addr.port));
            }
          }
        }
      }
    })
    .await
    {
      Ok(r) => return Ok(r),
      _ => return Err(Error::Timeout.into()),
    }
  }
  Err(Error::AddrNotExist.into())
}
