use crate::config_get;
use log::{info, warn};
use rmw_upnp::daemon;

pub async fn init(port: u16) {
  if config_get!(udp, upnp, || true) {
    let duration = 60;
    let recver = daemon("rmw upnp test".to_string(), port, duration);
    let mut upnp_success: i8 = 0;
    loop {
      if let Ok(r) = recver.recv().await {
        match r {
          Ok((ex_port, ip, gateway)) => {
            if upnp_success <= 0 {
              upnp_success = 1;
              info!("upnp {} -> {}:{}\ngateway {:?}", ex_port, ip, port, gateway);
            }
          }
          Err(err) => {
            if upnp_success >= 0 {
              upnp_success = -1;
              warn!("upnp error {:?}", err)
            }
          }
        }
      }
    }
  }
}
