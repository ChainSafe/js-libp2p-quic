#![deny(clippy::all)]

use std::{
  net::{IpAddr, SocketAddr}, sync::Arc
};

use napi::bindgen_prelude::*;
use napi_derive::napi;
use tokio::sync::Mutex;
use socket2::{Socket, Domain, Type};

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
    let socket;

    // Create a TCP listener bound to two addresses.
    if ip_addr.is_ipv6() {
      socket = Socket::new(Domain::IPV6, Type::DGRAM, None)?;
      socket.set_only_v6(true)?;
    } else {
      socket = Socket::new(Domain::IPV4, Type::DGRAM, None)?;
    }

    socket.bind(&socket_addr.into())?;

    let socket: Arc<socket::UdpSocket> = block_on(async move{
      socket::UdpSocket::wrap_udp_socket(socket.into())
    })?;
    let endpoint = block_on(async {
      quinn::Endpoint::new_with_abstract_socket(
        config.endpoint_config.clone(),
        Some(config.server_config.clone()),
        socket.clone(),
        Arc::new(quinn::TokioRuntime),
      )
    })?;
    Ok(Self { socket, endpoint })
  }

  #[napi]
  pub fn port(&self) -> u16 {
    return self.endpoint.local_addr().unwrap().port();
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
  pub async fn abort(&self) {
    self.endpoint.close(0u8.into(), b"");
    self.endpoint.wait_idle().await;
    self.socket.unbind().await;
  }

  #[napi]
  pub fn stats(&self) -> stats::EndpointStats {
    let stats = self.endpoint.stats();
    stats::EndpointStats::new(
      stats.accepted_handshakes,
      stats.outgoing_handshakes,
      stats.refused_handshakes,
      stats.ignored_handshakes,
    )
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
    let mut endpoint = block_on(async move {
      let socket = socket::create_socket(config.socket_config,bind_addr)?;
      let endpoint = quinn::Endpoint::new(
        config.endpoint_config.clone(),
        None,
        socket,
        Arc::new(quinn::TokioRuntime),
      )?;
      Ok::<quinn::Endpoint, Error>(endpoint)
    })?;
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

  #[napi]
  pub fn stats(&self) -> stats::EndpointStats {
    let stats = self.endpoint.stats();
    stats::EndpointStats::new(
      stats.accepted_handshakes,
      stats.outgoing_handshakes,
      stats.refused_handshakes,
      stats.ignored_handshakes,
    )
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
      "/{}/{}/udp/{}/quic-v1/p2p/{}",
      match remote_addr {
        SocketAddr::V4(_) => "ip4",
        SocketAddr::V6(_) => "ip6",
      },
      remote_addr.ip(),
      remote_addr.port(),
      self.peer_id().to_base58()
    )
  }

  // taken from rust-libp2p-quic
  pub fn peer_id(&self) -> libp2p_identity::PeerId {
    let identity = self
      .connection
      .peer_identity()
      .expect("connection got identity because it passed TLS handshake; qed");
    let certificates: Box<Vec<rustls::pki_types::CertificateDer>> =
      identity.downcast().expect("we rely on rustls feature; qed");
    let end_entity = certificates
      .first()
      .expect("there should be exactly one certificate; qed");
    let p2p_cert = libp2p_tls::certificate::parse(end_entity)
      .expect("the certificate was validated during TLS handshake; qed");
    p2p_cert.peer_id()
  }

  #[napi]
  pub async fn closed(&self) -> () {
    self.connection.closed().await;
  }

  #[napi]
  pub fn stats(&self) -> stats::ConnectionStats {
    self.connection.stats().into()
  }
}

#[napi]
pub struct Stream {
  send: Arc<Mutex<quinn::SendStream>>,
  recv: Arc<Mutex<quinn::RecvStream>>,
}

#[napi]
impl Stream {
  pub fn new(send: quinn::SendStream, recv: quinn::RecvStream) -> Self {
    Self { send: Arc::new(Mutex::new(send)), recv: Arc::new(Mutex::new(recv)) }
  }

  #[napi]
  pub fn id(&self) -> Result<String> {
    let send = self.send.try_lock().map_err(to_err)?;
    Ok(send.id().index().to_string())
  }

  #[napi]
  pub async fn read(&self, max_len: u32) -> Result<Option<Uint8Array>> {
    let mut buf = vec![0u8; max_len as usize];
    let chunk = self.recv.lock().await.read(&mut buf).await.map_err(to_err)?;
    match chunk {
      Some(len) => {
        buf.truncate(len);
        Ok(Some(buf.into()))
      },
      None => Ok(None),
    }
  }

  #[napi]
  pub async fn write(&self, data: Uint8Array) -> Result<()> {
    self.send.lock().await.write_all(&data).await.map_err(to_err)
  }

  #[napi]
  pub async fn finish_write(&self) {
    let _ = self.send.lock().await.finish();
  }

  #[napi]
  pub async fn reset_write(&self) {
    let _ = self.send.lock().await.reset(0u8.into());
  }

  #[napi]
  pub async fn stop_read(&self) {
    let _ = self.recv.lock().await.stop(0u8.into());
  }
}

fn to_err<T: ToString>(str: T) -> napi::Error {
  napi::Error::new(Status::Unknown, str)
}
