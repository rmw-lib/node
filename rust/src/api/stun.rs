use crate::util::addr_to_bytes::ToBytes;
use crate::Try;
use async_std::net::UdpSocket;
use async_std::task::spawn;
use neon::prelude::*;
use std::net::{IpAddr, SocketAddr};

pub fn external_addr(mut cx: FunctionContext) -> JsResult<JsPromise> {
  let server = cx.argument::<JsString>(0)?.value(&mut cx);
  let port = cx.argument::<JsNumber>(1)?.value(&mut cx) as u16;

  let ip = match cx.argument_opt(2) {
    Some(v) => v.downcast_or_throw::<JsString, _>(&mut cx)?.value(&mut cx),
    None => "0.0.0.0".to_string(),
  };
  let timeout = match cx.argument_opt(3) {
    Some(v) => v.downcast_or_throw::<JsNumber, _>(&mut cx)?.value(&mut cx) as u64,
    None => 3,
  };

  let ip = Try!(cx, ip.parse::<IpAddr>());

  let (deferred, promise) = cx.promise();
  let channel = cx.channel();
  spawn(async move {
    let udp = UdpSocket::bind(SocketAddr::new(ip, port)).await.unwrap();
    macro_rules! done {
      ($addr:ident) => {
        channel.send(move |mut cx| {
          let addr = JsArrayBuffer::external(&mut cx, $addr.to_bytes());
          deferred.resolve(&mut cx, addr);
          Ok(())
        })
      };
    }
    match rmw_stun::external_addr(&udp, server, timeout).await {
      Ok(addr) => match addr {
        SocketAddr::V6(addr) => {
          done!(addr);
        }
        SocketAddr::V4(addr) => {
          done!(addr);
        }
      },
      Err(err) => {
        channel.send(move |mut cx| {
          let err = cx.string(err.to_string());
          deferred.reject(&mut cx, err);
          Ok(())
        });
      }
    }
  });

  Ok(promise)
}
