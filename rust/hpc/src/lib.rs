pub use icall::CodeBody;

mod hpc;
pub use hpc::Hpc;

mod call;
pub use call::{call_err, run};

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
