pub mod api;
pub mod config;
pub mod grpc;
pub mod init;
pub mod udp;
pub mod upnp;
pub mod var;

#[macro_export]
macro_rules! run {
  ($r:expr) => {
    match $r {
      Err(err) => {
        log::error!("{}", err);
      }
      _ => {}
    }
  };
}
