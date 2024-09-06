#![deny(clippy::all)]

use std::{
  net::{IpAddr, SocketAddr}, sync::Arc, vec
};

use napi::{bindgen_prelude::*, JsArrayBufferValue, JsBuffer, JsNumber, JsObject, JsTypedArray, JsUndefined, Ref};
use napi_derive::napi;
use quinn::SendStream;
use tokio::sync::Mutex;

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
  recv: quinn::RecvStream,
  // send: Arc<quinn::SendStream>,
  // recv: Arc<quinn::RecvStream>,
}

#[napi]
impl Stream {
  pub fn new(send: quinn::SendStream, recv: quinn::RecvStream) -> Self {
    Self { send: Arc::new(Mutex::new(send)), recv }
  }

  #[napi]
  pub fn id(&self) -> Result<String> {
    let send = self.send.try_lock().map_err(to_err)?;
    Ok(send.id().index().to_string())
  }

  #[napi]
  pub async unsafe fn write(&mut self, data: Uint8Array) -> Result<()> {
    // self.send.write_all(&data).await.map_err(to_err)
    Ok(())
  }

  #[napi]
  pub async unsafe fn read(&mut self, mut buf: Uint8Array) -> Result<Option<u32>> {
    let chunk = self.recv.read(buf.as_mut()).await.map_err(to_err)?;
    match chunk {
      Some(len) => Ok(Some(len as u32)),
      None => Ok(None),
    }
  }

  #[napi]
  pub async unsafe fn read2(&mut self) -> Result<Option<Uint8Array>> {
    let mut buf = vec![0u8; 1024];
    let chunk = self.recv.read(buf.as_mut()).await.map_err(to_err)?;
    match chunk {
      Some(len) => Ok(Some(Uint8Array::with_data_copied(&buf[..len as usize])),),
      None => Ok(None),
    }
  }

  #[napi]
  pub fn write2(&mut self, #[napi(ts_arg_type = "Uint8Array")] data: JsTypedArray) -> Result<AsyncTask<Write>> {
    let data = data.into_value()?;
    let byte_offset = data.byte_offset;
    let length = data.length;
    let data = data.arraybuffer.into_ref()?;
    Ok(AsyncTask::new(Write {
      data,
      byte_offset,
      length,
      send: self.send.clone(),
    }))
  }

  #[napi]
  pub async unsafe fn finish_write(&mut self) {
    let _ = self.send.lock().await.finish();
  }

  #[napi]
  pub async unsafe fn reset_write(&mut self) {
    let _ = self.send.lock().await.reset(0u8.into());
  }

  #[napi]
  pub fn stop_read(&mut self) {
    let _ = self.recv.stop(0u8.into());
  }
}

fn to_err<T: ToString>(str: T) -> napi::Error {
  napi::Error::new(Status::Unknown, str)
}

pub struct Write {
  data: Ref<JsArrayBufferValue>,
  byte_offset: usize,
  length: usize,
  send: Arc<Mutex<SendStream>>,
}

impl Task for Write {
  type Output = ();
  type JsValue = JsUndefined;

  fn compute(&mut self) -> Result<Self::Output> {
    block_on(async move {
      let mut send = self.send.lock().await;
      send.write_all(&self.data[self.byte_offset..self.byte_offset+self.length]).await.map_err(to_err)
    })
  }

  fn resolve(&mut self, env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    env.get_undefined()
  }

  fn finally(&mut self, env: Env) -> Result<()> {
    self.data.unref(env)?;
    Ok(())
  }
}

// struct Read {
//   buf: Uint8Array,
//   stream: Stream,
// }

// impl Task for Read {
//   type Output = Option<u32>;
//   type JsValue = Either<JsNumber, JsUndefined>;

//   fn compute(&mut self) -> Result<Self::Output> {
//     block_on(async move {
//       let chunk = self.stream.recv.read(self.buf.as_mut()).await.map_err(to_err)?;
//       match chunk {
//         Some(len) => Ok(Some(len as u32)),
//         None => Ok(None),
//       }
//     })
//   }

//   fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
//     if let Some(output) = output {
//       env.create_uint32(output).map(Either::A)
//     } else {
//       env.get_undefined().map(Either::B)
//     }
//   }
// }

// mod out;
