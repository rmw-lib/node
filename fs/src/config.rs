use lazy_static::lazy_static;
use log::error;
use rmw_str::Str;
use std::{fs, path::PathBuf};

lazy_static! {
  pub static ref ROOT: PathBuf = crate::var::dir::ROOT.clone().join("conf");
}

pub fn get<T: Str>(file: &str, init: fn() -> T) -> T {
  let path = ROOT.clone().join(file);
  let _init = || {
    let r = init();
    let mut dir = path.clone();
    dir.pop();
    fs::create_dir_all(dir).unwrap();
    fs::write(&path, &r.encode()).unwrap();
    r
  };

  match fs::read_to_string(&path) {
    Ok(txt) => {
      let buf = txt.trim();
      match T::decode(buf) {
        Ok(r) => {
          if buf != txt {
            fs::write(&path, &buf).unwrap();
          }
          r
        }
        Err(err) => {
          error!("{}", err);
          _init()
        }
      }
    }
    Err(_) => _init(),
  }
}

#[macro_export]
macro_rules! config_get {
  ($dir:tt, $file:tt, $init:expr) => {
    $crate::config::get(concat!(stringify!($dir), '/', stringify!($file)), $init)
  };
}
