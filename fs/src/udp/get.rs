use async_std::fs;
use async_std::fs::File;
use async_std::net::SocketAddrV4;
use async_std::prelude::Future;
use hashbrown::{HashMap, HashSet};
use log::error;
use std::collections::BTreeMap;
use std::num::Wrapping;
use std::path::{Path, PathBuf};
use vob::Vob;

#[derive(Debug)]
pub struct Get {
  file: File,
  bit: Vob,
}

#[derive(Debug)]
pub enum Task {
  Size,
  Get,
}

#[derive(Debug, Default)]
pub struct State {
  id: Wrapping<u32>,
  utime: u64,
  src: HashSet<Box<[u8]>>,
  expire: BTreeMap<u64, Task>,
}

#[derive(Debug, Default)]
pub struct GetBox {
  expire: BTreeMap<u64, SocketAddrV4>,
  ing: HashMap<SocketAddrV4, State>,
}

impl GetBox {
  pub async fn add<Send: Future>(&self, addr: SocketAddrV4, src: &[u8], out: &Path, send: Send) {
    let mut dir: PathBuf = out.into();
    dir.pop();
    send.await;
    match fs::create_dir_all(dir).await {
      Ok(_) => {}
      Err(err) => {
        error!("{:?}", err);
      }
    }
  }
}

/*
#[derive(Debug)]
pub struct Get {
  file: File,
  /*
  bit: Vob,
  offset: usize,
  pos: usize,
  */
}
#[derive(Debug)]
enum EnumTask {
  Get(Get),
}

struct Task {
  task: EnumTask,
  utime: u64,
}

struct AddrTaskState {
  id: Wrapping<u32>,
  runing: u32,
}

type AddrTask = HashMap<SocketAddr, HashMap<u32, Task>>;
type Ing = BTreeMap<u64, Task>;
type AddrGetSrc = HashMap<SocketAddr, HashSet<Box<[u8]>>>;
use vob::Vob;

  let mut addr_get_src = AddrGetSrc::new();
  let mut ing = Ing::new();

            macro_rules! insert {
              ($set:ident) => {
                /*
                let task = Task {
                  task: Get
                }
                */
                println!("{:?}", get.0);
                send!(Get);
                $set.insert(msg);
              };
            }

if let Some(set) = addr_get_src.get_mut(&addr) {
              if let Some(_) = set.get(&msg) {
                println!("get task exist {:?}", msg);
                continue;
              } else {
                insert!(set);
              }
            } else {
              let mut set = HashSet::new();
              insert!(set);
              addr_get_src.insert(addr, set);
            }
*/
