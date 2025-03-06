pub use ih::CodeBody;
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
mod header;

#[cfg(feature = "srv")]
pub async fn srv<H, T>(port: u16, router: axum::Router, path: &str, hpc: H) -> aok::Result<()>
where
  H: axum::handler::Handler<T, ()>,
  T: 'static,
{
  use std::net::{IpAddr, Ipv4Addr, SocketAddr};

  use axum::{Router, middleware, routing::put};
  use tokio::net::TcpListener;
  use tracing::info;

  let hpc_router: Router = Router::new()
    .route(path, put(hpc))
    .layer(middleware::from_fn(header::set))
    .layer(cookie_b::BrowserIdLayer);

  let router = axum_layer::layer(router.merge(hpc_router));

  info!("http://0.0.0.0:{}", port);

  if let Ok(tcp) =
    xerr::ok!(TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)).await)
  {
    axum::serve(tcp, router).await?;
  }
  Ok(())
}
