use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use aok::Result;
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

mod hpc;
pub use hpc::{run, CodeBody, Hpc};
pub use icall::Call;

pub async fn srv(port: u16, router: Router) -> Result<()> {
  info!("http://0.0.0.0:{}", port);

  axum::serve(
    TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)).await?,
    router,
  )
  .await?;
  Ok(())
}
