use lazy_static::lazy_static;
use std::{env, path::PathBuf};

lazy_static! {
  pub static ref ROOT: PathBuf = {
    if let Ok(root) = env::var("RMW_ROOT") {
      root.into()
    } else {
      let mut rmw = match dirs::home_dir() {
        Some(root) => root,
        None => {
          let mut dir = env::current_exe().unwrap();
          dir.pop();
          dir
        }
      };
      rmw.push(".rmw");
      rmw
    }
  };
}
