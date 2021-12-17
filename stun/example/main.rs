use async_std::net::UdpSocket;
use rmw_stun::external_addr;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Instant};

#[async_std::main]
async fn main() {
  let udp = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0))
    .await
    .unwrap();
  //let server = "not.exist.stun:3478";
  let server = "stun.cablenet-as.net:3478";
  let timeout = 3;
  let start = Instant::now();
  let addr = external_addr(&udp, server, timeout).await;
  let duration = start.elapsed();

  println!("external addr {:?} cost {:?}", addr, duration);
}
