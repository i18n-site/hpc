pub use icall::CodeBody;

mod hpc;
pub use hpc::Hpc;

mod call;
pub use call::{
  call_err::{args_invalid, call_err},
  run,
};

pub type Result<T> = std::result::Result<T, CodeBody>;

pub fn args_decode<T: pb_jelly::Message>(args: &[u8], name: &str) -> Result<T> {
  if let Ok(args) = T::deserialize_from_slice(args) {
    return Ok(args);
  }
  Err(args_invalid(name))
}

#[cfg(feature = "srv")]
pub async fn srv(port: u16, router: axum::Router) -> aok::Result<()> {
  use std::net::{IpAddr, Ipv4Addr, SocketAddr};

  use tokio::net::TcpListener;
  use tracing::info;

  info!("http://0.0.0.0:{}", port);

  if let Ok(tcp) =
    xerr::ok!(TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)).await)
  {
    axum::serve(tcp, router).await?;
  }
  Ok(())
}
