use crate::Try;
use blake3::Hasher;
use neon::prelude::*;
use std::fs::File;
use std::io::copy;

pub fn file(mut cx: FunctionContext) -> JsResult<JsBuffer> {
  let path: String = cx.argument::<JsString>(0)?.value(&mut cx);

  let mut file = Try!(cx, File::open(path));
  let mut hasher = Hasher::new();
  Try!(cx, copy(&mut file, &mut hasher));
  let hash: [u8; 32] = *hasher.finalize().as_bytes();
  let r = JsBuffer::external(&mut cx, hash);
  Ok(r)
}
