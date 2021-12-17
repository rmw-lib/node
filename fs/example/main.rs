use anyhow::Result;
use async_std::channel::bounded;
use async_std::net::UdpSocket;
use futures::join;
use log::error;
use rmw_fs::{config_get, grpc, init, udp, upnp};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[async_std::main]
async fn main() -> Result<()> {
  init::init();
  let port = config_get!(udp, port, || {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.local_addr().unwrap().port()
  });

  let (s, r) = bounded(8);

  match UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)).await {
    Ok(udp) => {
      // https://github.com/tokio-rs/tokio/discussions/3755

      let _ = join!(
        udp::init(udp, r),
        grpc::init(
          s,
          config_get!(grpc, port, || SocketAddr::new(
            IpAddr::V4([127, 0, 0, 1].into()),
            9981
          ))
        ),
        upnp::init(port),
      );
    }
    Err(err) => {
      error!("{}", err);
      std::process::exit(1);
    }
  }

  Ok(())
}
