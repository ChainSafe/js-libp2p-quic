#![deny(clippy::all)]

use std::{
  net::{IpAddr, SocketAddr},
  sync::Arc,
};

use napi::bindgen_prelude::*;
use napi_derive::napi;

mod config;
mod stats;

#[napi]
pub enum SocketFamily {
  Ipv4,
  Ipv6,
}

#[napi]
pub struct Server {
  endpoint: quinn::Endpoint,
}

#[napi]
impl Server {
  #[napi(constructor)]
  pub fn new(config: &config::QuinnConfig, ip: String, port: u16) -> Result<Self> {
    let ip_addr = ip.parse::<IpAddr>().map_err(to_err)?;
    let socket_addr = SocketAddr::new(ip_addr, port);
    let socket = std::net::UdpSocket::bind(socket_addr)?;
    let endpoint = quinn::Endpoint::new(
      config.endpoint_config.clone(),
      Some(config.server_config.clone()),
      socket,
      Arc::new(quinn::TokioRuntime),
    )?;
    Ok(Self { endpoint })
  }

  #[napi]
  pub async fn inbound_connection(&self) -> Result<Connection> {
    let incoming = match self.endpoint.accept().await {
      Some(incoming) => Ok(incoming),
      None => Err(to_err("connection closed")),
    }?;
    let connection = incoming.await.map_err(to_err)?;
    Ok(Connection::new(connection))
  }

  #[napi]
  pub fn abort(&self) {
    self.endpoint.close(0u8.into(), b"");
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
    let ip_addr = ip
      .parse::<IpAddr>()
      .map_err(to_err)?;
    let socket_addr = SocketAddr::new(ip_addr, port);
    let connecting = self
      .endpoint
      .connect(socket_addr, "l")
      .map_err(to_err)?;
    let connection = connecting
      .await
      .map_err(to_err)?;

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
    let (send, recv) = self
      .connection
      .accept_bi()
      .await
      .map_err(to_err)?;
    Ok(Stream::new(send, recv))
  }

  #[napi]
  pub async fn outbound_stream(&self) -> Result<Stream> {
    let (send, recv) = self
      .connection
      .open_bi()
      .await
      .map_err(to_err)?;
    Ok(Stream::new(send, recv))
  }

  #[napi]
  /// close the connection immediately
  pub fn abort(&self) {
    self.connection.close(0u8.into(), b"");
  }

  #[napi]
  pub fn id(&self) -> u32 {
    self.connection.stable_id() as u32
  }

  #[napi]
  pub fn rtt(&self) -> u32 {
    self.connection.rtt().as_millis() as u32
  }
}

#[napi]
pub struct Stream {
  pub id: String,
  send: quinn::SendStream,
  recv: quinn::RecvStream,
}

#[napi]
impl Stream {
  pub fn new(send: quinn::SendStream, recv: quinn::RecvStream) -> Self {
    Self {
      id: send.id().to_string(),
      send,
      recv,
    }
  }

  #[napi]
  pub async unsafe fn write(&mut self, data: Uint8Array) -> Result<()> {
    self
      .send
      .write_all(&data)
      .await
      .map_err(to_err)
  }

  #[napi]
  pub async unsafe fn read(&mut self, max_length: u32) -> Result<Uint8Array> {
    let chunk = self
      .recv
      .read_chunk(max_length as usize, true)
      .await
      .map_err(to_err)?;
    match chunk {
      Some(data) => Ok(data.bytes.into()),
      None => Err(to_err("EOF")),
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