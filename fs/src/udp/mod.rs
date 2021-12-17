pub mod cmd;
pub mod get;
pub mod r#type;

use crate::run;
use crate::var::dir::ROOT;
use anyhow::Result;
use async_std::channel::Receiver;
use async_std::net::SocketAddr;
use async_std::sync::Arc;
use async_std::{fs::File, net::UdpSocket};
use bytes::{BufMut, BytesMut};
use cmd::{Cmd, CmdU8};
use futures::join;
use log::{error, info};
use r#type::CmdV4;
use rmw_utf8::decode;
use std::fs;

// https://stackoverflow.com/questions/3928766/how-big-is-an-ip-packet-frame-including-headers
const MTU: usize = 1472;
//const MSL: u64 = 30;
// 1472-8(加密校验码)-1(字节头)-4(文件编号)
// bytes 1489920

pub async fn init(udp: UdpSocket, recver: Receiver<CmdV4>) -> Result<()> {
  info!("udp {:?}", udp.local_addr()?);

  // addr : task_id utime
  // addr : task_id -> File
  //let addr_ing = HashMap::new()
  //let li: LinkedList<Vec<()>> = LinkedList::new();

  let udp = Arc::new(udp);
  let udp_s = udp.clone();
  let get_box = get::GetBox::default();

  let _ = join!(
    async move {
      while let Ok((addr, cmd, msg)) = recver.recv().await {
        let msg = &*msg;
        macro_rules! send {
          ($cmd:ident) => {
            async {
              let mut buf = BytesMut::with_capacity(1472);
              buf.put_u8(CmdU8::$cmd.into());
              buf.put(msg);
              info!("{:?} {:?} {:?}", addr, cmd, msg);
              run!(udp_s.send_to(&buf, addr).await);
            }
          };
        }
        match cmd {
          Cmd::Get(ref get) => get_box.add(addr, msg, &get.0, send!(Get)).await,
        }
      }
    },
    _udp(udp)
  );
  Ok(())

  /*
  let connecting = Cache::<SocketAddrV4, u64>::new();
    connecting.monitor(2, 1, *MSL / 3 + Duration::from_secs(1), &|kvli| {
        //msl秒内有过成功的连接
        if kvli.len() > 0 && (now::sec() - unsafe { CONNECTED_TIME }) <= *EXPIRE {
          for (addr, site_id) in kvli {
            println!("expire ip {:?} v {:?}", addr, site_id);
          }
        }
      }),
  */
}

async fn _udp(udp: Arc<UdpSocket>) -> Result<()> {
  let mut root = ROOT.clone();
  root.push("mnt");
  fs::create_dir_all(&root).unwrap();

  let mut buf = BytesMut::with_capacity(MTU);
  unsafe { buf.set_len(MTU) };

  while let Ok((n, peer)) = udp.recv_from(&mut buf).await {
    if n > 0 {
      let data = &buf[1..n];
      if let Ok(cmd) = buf[0].try_into() {
        match cmd {
          CmdU8::Get => {
            let fp = decode(data);
            info!("get {}", fp);
            match File::open(&root.clone().join(&fp)).await {
              Ok(file) => {
                info!("{:?}", file);
              }
              Err(err) => {
                error!("{} {:?}", fp, err)
              }
            }
          }
          _ => {
            info!("{} {:?} : {:?}", peer, cmd, data);
          }
        }
      }
      continue;
    }
    info!("{} {} : {:?}", n, peer, buf);
    //udp.send_to(&buf[..n], &peer).await?;
  }

  Ok(())
}
