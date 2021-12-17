use rmw_upnp::daemon;
use std::process::exit;

#[async_std::main]
async fn main() {
  let port = 23087;
  let duration = 60;
  let r = daemon("rmw upnp test".to_string(), port, duration);
  loop {
    println!("{:?}", r.recv().await);
    exit(0);
  }
}
