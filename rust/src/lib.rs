mod api;

#[macro_export]
macro_rules! Try {
  ( $cx:ident, $func:expr ) => {
    $func.or_else(|err| $cx.throw_error(err.to_string()))?
  };
}

use neon::prelude::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  macro_rules! def {
    (  $mod:ident, $name:ident ) => {
      cx.export_function(
        concat!(stringify!($mod), '_', stringify!($name)),
        api::$mod::$name,
      )?
    };
  }
  def!(blake3, file);
  def!(upnp, daemon);
  //  def!(stun, external_addr);
  Ok(())
}
