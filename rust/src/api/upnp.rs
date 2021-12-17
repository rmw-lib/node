use async_std::task;
use neon::prelude::*;
use rmw_bytes::Bytes;

pub fn daemon(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let name = cx.argument::<JsString>(0)?.value(&mut cx);
  let port = cx.argument::<JsNumber>(1)?.value(&mut cx) as u16;
  let duration = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;

  let resolve = cx.argument::<JsFunction>(3)?.root(&mut cx);
  let reject = cx.argument::<JsFunction>(4)?.root(&mut cx);
  let channel = cx.channel();
  let recver = rmw_upnp::daemon(name, port, duration);

  channel.send(move |mut cx| {
    let this = cx.undefined();
    let resolve = resolve.into_inner(&mut cx);
    let reject = reject.into_inner(&mut cx);
    task::block_on(async move {
      while let Ok(r) = recver.recv().await {
        match r {
          Ok(r) => {
            let ip = JsArrayBuffer::external(&mut cx, r.1.octets());
            let gateway = JsArrayBuffer::external(&mut cx, r.2.to_bytes());
            let args = [
              cx.number(r.0).upcast::<JsValue>(),
              ip.upcast::<JsValue>(),
              gateway.upcast::<JsValue>(),
            ];
            resolve.call(&mut cx, this, args).unwrap();
          }
          Err(r) => {
            let args = [cx.string(r.to_string())];
            reject.call(&mut cx, this, args)?;
          }
        }
      }
      return Ok::<(), neon::result::Throw>(());
    })?;
    Ok(())
  });
  Ok(cx.undefined())
}
