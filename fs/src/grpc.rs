use crate::{api::Api, run};
use anyhow::Result;
use async_std::net::SocketAddr;
use log::info;
use proto::fs_server::{Fs, FsServer};
use proto::{Empty, Req};
use tonic::{transport::Server, Request, Response, Status};

pub mod proto {
  tonic::include_proto!("proto");
}

pub struct FsImpl {
  pub api: Api,
}

#[tonic::async_trait]
impl Fs for FsImpl {
  async fn get(&self, req: Request<Req>) -> Result<Response<Empty>, Status> {
    //info!("> {:?} {:?}", request.remote_addr(), request);

    let req = req.into_inner();
    let reply = Empty {};
    run!(self.api.get(req.addr, req.src.as_bytes(), req.out,).await);
    //message: format!("Hello {}!", request.into_inner().name),
    Ok(Response::new(reply))
  }
}

pub async fn init<IntoApi: Into<Api>>(api: IntoApi, addr: SocketAddr) -> Result<()> {
  let fs = FsServer::new(FsImpl { api: api.into() });

  #[cfg(feature = "grpc-http")]
  let tip = format!("\nhttp://{}", addr);

  #[cfg(not(feature = "grpc-http"))]
  let tip = "";

  info!("grpc\ntcp://{}{}", addr, tip);

  #[cfg(feature = "grpc-http")]
  macro_rules! fs {
    ($fs:expr) => {
      tonic_web::enable($fs)
    };
  }

  #[cfg(feature = "grpc-http")]
  macro_rules! srv {
    ($srv:expr) => {
      $srv.accept_http1(true)
    };
  }

  #[cfg(not(feature = "grpc-http"))]
  macro_rules! fs {
    ($fs:expr) => {
      $fs
    };
  }

  #[cfg(not(feature = "grpc-http"))]
  macro_rules! srv {
    ($srv:expr) => {
      $srv
    };
  }

  srv!(Server::builder())
    .add_service(fs!(fs))
    .serve(addr)
    .await?;
  //    let fs = tonic_web::enable(fs);
  //    let mut srv = srv.accept_http1(true);
  //    tip = format!("{}\nhttp://{}", tip, addr);
  //    serve!(srv, fs);
  //  #[cfg(not(feature = "grpc-http"))]

  Ok(())
}
