#![deny(clippy::all)]

use std::{
  net::{IpAddr, SocketAddr},
  sync::Arc,
};

use napi::bindgen_prelude::*;
use napi_derive::napi;

mod config;
mod socket;
mod stats;

#[napi]
pub enum SocketFamily {
  Ipv4,
  Ipv6,
}

#[napi]
pub struct Server {
  socket: Arc<socket::UdpSocket>,
  endpoint: quinn::Endpoint,
}

#[napi]
impl Server {
  #[napi(constructor)]
  pub fn new(config: &config::QuinnConfig, ip: String, port: u16) -> Result<Self> {
    let ip_addr = ip.parse::<IpAddr>().map_err(to_err)?;
    let socket_addr = SocketAddr::new(ip_addr, port);
    let socket = std::net::UdpSocket::bind(socket_addr)?;
    let socket = socket::UdpSocket::wrap_udp_socket(socket)?;
    let endpoint = quinn::Endpoint::new_with_abstract_socket(
      config.endpoint_config.clone(),
      Some(config.server_config.clone()),
      socket.clone(),
      Arc::new(quinn::TokioRuntime),
    )?;
    Ok(Self { socket, endpoint })
  }

  #[napi]
  pub async fn inbound_connection(&self) -> Result<Connection> {
    let incoming = match self.endpoint.accept().await {
      Some(incoming) => Ok(incoming),
      None => Err(to_err("server closed")),
    }?;
    let connection = incoming.await.map_err(to_err)?;
    Ok(Connection::new(connection))
  }

  #[napi]
  pub async unsafe fn abort(&mut self) {
    self.endpoint.close(0u8.into(), b"");
    self.endpoint.wait_idle().await;
    self.socket.unbind().await;
  }
}

#[napi]
pub struct Client {
  endpoint: quinn::Endpoint,
}

#[napi]
impl Client {
  #[napi(constructor)]
  pub fn new(config: &config::QuinnConfig, family: SocketFamily) -> Result<Self> {
    let bind_addr = match family {
      SocketFamily::Ipv4 => SocketAddr::new(std::net::Ipv4Addr::UNSPECIFIED.into(), 0),
      SocketFamily::Ipv6 => SocketAddr::new(std::net::Ipv6Addr::UNSPECIFIED.into(), 0),
    };
    let socket = std::net::UdpSocket::bind(bind_addr)?;
    let mut endpoint = quinn::Endpoint::new(
      config.endpoint_config.clone(),
      None,
      socket,
      Arc::new(quinn::TokioRuntime),
    )?;
    endpoint.set_default_client_config(config.client_config.clone());
    Ok(Client { endpoint })
  }

  #[napi]
  pub async fn outbound_connection(&self, ip: String, port: u16) -> Result<Connection> {
    let ip_addr = ip.parse::<IpAddr>().map_err(to_err)?;
    let socket_addr = SocketAddr::new(ip_addr, port);
    let connecting = self.endpoint.connect(socket_addr, "l").map_err(to_err)?;
    let connection = connecting.await.map_err(to_err)?;

    Ok(Connection::new(connection))
  }

  #[napi]
  pub fn abort(&self) {
    self.endpoint.close(0u8.into(), b"");
  }
}

#[napi]
pub struct Connection {
  connection: quinn::Connection,
}

#[napi]
impl Connection {
  pub fn new(connection: quinn::Connection) -> Self {
    Self { connection }
  }

  #[napi]
  pub async fn inbound_stream(&self) -> Result<Stream> {
    let (send, recv) = self.connection.accept_bi().await.map_err(to_err)?;
    Ok(Stream::new(send, recv))
  }

  #[napi]
  pub async fn outbound_stream(&self) -> Result<Stream> {
    let (send, recv) = self.connection.open_bi().await.map_err(to_err)?;
    Ok(Stream::new(send, recv))
  }

  #[napi]
  /// close the connection immediately
  pub fn abort(&self) {
    self.connection.close(0u8.into(), b"");
  }

  #[napi]
  pub fn rtt(&self) -> u32 {
    self.connection.rtt().as_millis() as u32
  }

  #[napi]
  pub fn id(&self) -> String {
    self.connection.stable_id().to_string()
  }

  #[napi]
  pub fn remote_multiaddr(&self) -> String {
    let remote_addr = self.connection.remote_address();
    format!(
      "/{}/{}/udp/{}/quic-v1",
      match remote_addr {
        SocketAddr::V4(_) => "ip4",
        SocketAddr::V6(_) => "ip6",
      },
      remote_addr.ip(),
      remote_addr.port()
    )
  }

  #[napi]
  pub async fn closed(&self) -> () {
    self.connection.closed().await;
  }
}

#[napi]
pub struct Stream {
  send: quinn::SendStream,
  recv: quinn::RecvStream,
}

#[napi]
impl Stream {
  pub fn new(send: quinn::SendStream, recv: quinn::RecvStream) -> Self {
    Self { send, recv }
  }

  #[napi]
  pub fn id(&self) -> String {
    self.send.id().index().to_string()
  }

  #[napi]
  pub async unsafe fn write(&mut self, data: Uint8Array) -> Result<()> {
    self.send.write_all(&data).await.map_err(to_err)
  }

  #[napi]
  pub async unsafe fn read(&mut self, max_length: u32) -> Result<Option<Uint8Array>> {
    let chunk = self
      .recv
      .read_chunk(max_length as usize, true)
      .await
      .map_err(to_err)?;
    match chunk {
      Some(data) => Ok(Some(data.bytes.into())),
      None => Ok(None),
    }
  }

  #[napi]
  pub fn finish_write(&mut self) {
    let _ = self.send.finish();
  }

  #[napi]
  pub fn reset_write(&mut self) {
    let _ = self.send.reset(0u8.into());
  }

  #[napi]
  pub fn stop_read(&mut self) {
    let _ = self.recv.stop(0u8.into());
  }
}

fn to_err<T: ToString>(str: T) -> napi::Error {
  napi::Error::new(Status::Unknown, str)
}
