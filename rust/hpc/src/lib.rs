mod hpc;
pub use hpc::{Hpc, run};
pub use icall::CodeBody;

#[derive(Debug)]
pub struct CallErr {
  pub func: &'static str,
  pub args: Vec<String>,
  pub err: String,
}

impl std::fmt::Display for CallErr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}({}) {}", self.func, self.args.join(","), self.err)
  }
}

impl std::error::Error for CallErr {}

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
