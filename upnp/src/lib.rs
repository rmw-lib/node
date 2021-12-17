use anyhow::Result;
use async_std::{
  channel::{bounded, Receiver},
  task::{sleep, spawn},
};
use igd::aio::search_gateway;
use igd::AddPortError::{self, PortInUse};
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum UpnpError {
  #[error("not ipv4")]
  NotIpv4,
}

pub fn daemon(
  name: String,
  port: u16,
  duration: u32,
) -> Receiver<Result<(u16, Ipv4Addr, SocketAddrV4), String>> {
  let (s, r) = bounded(1);

  spawn(async move {
    if false {
      return Ok::<(), anyhow::Error>(());
    };

    let mut local_ip = Ipv4Addr::UNSPECIFIED;
    let mut pre_gateway = SocketAddrV4::new(local_ip, 0);
    let duration = duration + 9;
    let seconds = Duration::from_secs(duration.into());
    let mut ext_port = port;
    let mut retryed = 0;
    loop {
      match upnp(&name, port, ext_port, duration).await {
        Ok((gateway, ip)) => {
          if ip != local_ip || gateway != pre_gateway {
            retryed = 0;
            local_ip = ip;
            pre_gateway = gateway;
            s.send(Ok((ext_port, ip, gateway))).await?;
          }
        }
        Err(err) => {
          local_ip = Ipv4Addr::UNSPECIFIED;
          {
            let err = err.root_cause();
            if retryed < 9 {
              if let Some(PortInUse) = err.downcast_ref::<AddPortError>() {
                retryed += 1;
                if ext_port == 65535 {
                  ext_port = 1025;
                } else {
                  ext_port += 1;
                }
                continue;
              }
            }
          }
          retryed = 0;
          s.send(Err(err.to_string())).await?;
        }
      }
      sleep(seconds).await;
    }
  });
  r
}

pub async fn upnp(
  name: &str,
  port: u16,
  ext_port: u16,
  duration: u32,
) -> Result<(SocketAddrV4, Ipv4Addr)> {
  let gateway = search_gateway(Default::default()).await?;
  let gateway_addr = gateway.addr;
  let stream = TcpStream::connect(gateway_addr)?;
  let addr = stream.local_addr()?;
  drop(stream);
  if let IpAddr::V4(ip) = addr.ip() {
    gateway
      .add_port(
        igd::PortMappingProtocol::UDP,
        ext_port,
        SocketAddrV4::new(ip, port),
        duration,
        name,
      )
      .await?;
    Ok((gateway_addr, ip))
  } else {
    Err(UpnpError::NotIpv4.into())
  }
}
