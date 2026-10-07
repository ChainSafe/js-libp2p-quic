#![deny(clippy::all)]

use std::{
  collections::HashMap,
  net::{IpAddr, SocketAddr},
  sync::{atomic::{AtomicU64, Ordering}, Arc},
  time::Duration,
};

use napi::bindgen_prelude::*;
use napi_derive::napi;
use tokio::sync::{mpsc, Mutex, OwnedSemaphorePermit, Semaphore};

mod config;
mod socket;
mod stats;

#[napi]
pub enum SocketFamily {
  Ipv4,
  Ipv6,
}

/// Maximum number of inbound connections that may be handshaking, or established and
/// waiting to be picked up by `inbound_connection()`, at any one time.
/// Further incoming connections are refused until one of these completes.
const MAX_PENDING_INBOUND: usize = 256;

/// Once this many pending slots are taken, a remote address must be validated with a
/// Retry before its connection is given a slot.
const RETRY_THRESHOLD: usize = MAX_PENDING_INBOUND / 2;

/// Maximum number of pending slots held by one remote IPv4 address or IPv6 /64.
/// Validated and unvalidated addresses are counted separately, so packets sent with
/// someone else's source address cannot use up that address's share.
const MAX_PENDING_INBOUND_PER_SOURCE: usize = 8;

/// A remote IPv4 address or IPv6 /64, and whether the address has been validated
type Source = (IpAddr, bool);

type PendingSources = Arc<std::sync::Mutex<HashMap<Source, usize>>>;

fn source(incoming: &quinn::Incoming) -> Source {
  let ip = match incoming.remote_address().ip() {
    IpAddr::V6(ip) => IpAddr::V6((u128::from(ip) & (!0u128 << 64)).into()),
    ip => ip,
  };
  (ip, incoming.remote_address_validated())
}

/// A pending inbound connection's share of the limits above, released on drop
struct PendingSlot {
  _permit: OwnedSemaphorePermit,
  sources: PendingSources,
  source: Source,
}

impl Drop for PendingSlot {
  fn drop(&mut self) {
    let mut sources = self.sources.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(count) = sources.get_mut(&self.source) {
      *count -= 1;
      if *count == 0 {
        sources.remove(&self.source);
      }
    }
  }
}

#[napi]
pub struct Server {
  socket: Arc<socket::UdpSocket>,
  endpoint: quinn::Endpoint,
  shutdown_timeout: Duration,
  inbound: Arc<Mutex<mpsc::Receiver<(Connection, PendingSlot)>>>,
  failed_handshakes: Arc<AtomicU64>,
  retried_handshakes: Arc<AtomicU64>,
  accept_task: tokio::task::JoinHandle<()>,
  handshake_timeout: Duration,
}

impl Drop for Server {
  fn drop(&mut self) {
    // the accept loop holds its own handle to the endpoint, stop it so the endpoint can be dropped
    self.accept_task.abort();
  }
}

/// Accept incoming connections and drive each handshake on its own task, so a slow or
/// stalled handshake does not delay the connections behind it.
/// Established connections are queued on `established`.
async fn accept_loop(
  endpoint: quinn::Endpoint,
  established: mpsc::Sender<(Connection, PendingSlot)>,
  handshake_timeout: Duration,
  failed_handshakes: Arc<AtomicU64>,
  retried_handshakes: Arc<AtomicU64>,
) {
  let permits = Arc::new(Semaphore::new(MAX_PENDING_INBOUND));
  let sources = PendingSources::default();

  while let Some(incoming) = endpoint.accept().await {
    let source = source(&incoming);
    let validated = source.1;
    let slot = {
      let mut sources_guard = sources.lock().unwrap_or_else(|e| e.into_inner());
      let pending = sources_guard.get(&source).copied().unwrap_or(0);
      let source_full = pending >= MAX_PENDING_INBOUND_PER_SOURCE;
      let busy = MAX_PENDING_INBOUND - permits.available_permits() >= RETRY_THRESHOLD;

      if !validated && (source_full || busy) {
        // the peer will come back with a token that validates its address
        match incoming.retry() {
          Ok(()) => {
            retried_handshakes.fetch_add(1, Ordering::Relaxed);
          }
          Err(e) => e.into_incoming().refuse(),
        }
        continue;
      }

      let permit = match permits.clone().try_acquire_owned() {
        Ok(permit) if !source_full => permit,
        _ => {
          incoming.refuse();
          continue;
        }
      };

      sources_guard.insert(source, pending + 1);
      PendingSlot { _permit: permit, sources: sources.clone(), source }
    };
    let established = established.clone();
    let failed_handshakes = failed_handshakes.clone();

    tokio::spawn(async move {
      // on timeout the handshake future is dropped, which closes the connection
      let connection = match tokio::time::timeout(handshake_timeout, incoming).await {
        Ok(Ok(connection)) => Connection::new(connection).ok(),
        _ => None,
      };

      match connection {
        // the slot travels with the connection and is released once it has been picked up
        Some(connection) => {
          if let Err(mpsc::error::SendError((connection, _))) = established.send((connection, slot)).await {
            // the server has gone away
            connection.abort();
          }
        }
        None => {
          failed_handshakes.fetch_add(1, Ordering::Relaxed);
        }
      }
    });
  }
}

#[napi]
impl Server {
  #[napi(constructor)]
  pub fn new(config: &config::QuinnConfig, ip: String, port: u16) -> Result<Self> {
    let ip_addr = ip.parse::<IpAddr>().map_err(to_err)?;
    let socket_addr = SocketAddr::new(ip_addr, port);
    let socket = socket::create_socket(config.socket_config, socket_addr)?;

    let socket: Arc<socket::UdpSocket> = block_on(async move{
      socket::UdpSocket::wrap_udp_socket(socket)
    })?;
    let mut endpoint = block_on(async {
      quinn::Endpoint::new_with_abstract_socket(
        config.endpoint_config.clone(),
        Some(config.server_config.clone()),
        socket.clone(),
        Arc::new(quinn::TokioRuntime),
      )
    })?;
    endpoint.set_default_client_config(config.client_config.clone());
    let (established, inbound) = mpsc::channel(MAX_PENDING_INBOUND);
    let failed_handshakes = Arc::new(AtomicU64::new(0));
    let retried_handshakes = Arc::new(AtomicU64::new(0));
    let accept_task = spawn(accept_loop(
      endpoint.clone(),
      established,
      config.handshake_timeout,
      failed_handshakes.clone(),
      retried_handshakes.clone(),
    ));

    Ok(Self {
      socket,
      endpoint,
      shutdown_timeout: config.shutdown_timeout,
      inbound: Arc::new(Mutex::new(inbound)),
      failed_handshakes,
      retried_handshakes,
      accept_task,
      handshake_timeout: config.handshake_timeout,
    })
  }

  #[napi]
  pub fn port(&self) -> u16 {
    return self.endpoint.local_addr().unwrap().port();
  }

  #[napi(ts_return_type = "Promise<Connection>")]
  /// Resolves with the next inbound connection that has completed its handshake.
  ///
  /// Handshakes that fail, time out or are answered with a Retry are not surfaced here,
  /// they are counted in `stats()`.
  pub fn inbound_connection<'env>(&self, env: &'env Env) -> Result<PromiseRaw<'env, Connection>> {
    let inbound = self.inbound.clone();
    env.spawn_future(async move {
      match inbound.lock().await.recv().await {
        Some((connection, _slot)) => Ok(connection),
        None => Err(to_err("server closed")),
      }
    })
  }

  #[napi(ts_return_type = "Promise<Connection>")]
  /// Dial from the listening socket, so the connection leaves from the port peers were told to reach
  pub fn outbound_connection<'env>(
    &self,
    env: &'env Env,
    ip: String,
    port: u16,
  ) -> Result<PromiseRaw<'env, Connection>> {
    env.spawn_future(dial(self.endpoint.clone(), self.handshake_timeout, ip, port))
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn abort<'env>(&self, env: &'env Env) -> Result<PromiseRaw<'env, ()>> {
    let endpoint = self.endpoint.clone();
    let socket = self.socket.clone();
    let shutdown_timeout = self.shutdown_timeout;
    env.spawn_future(async move {
      endpoint.close(0u8.into(), b"");

      // wait_idle() depends on every connection driver reaching the drained
      // state, we bound the wait so one stalled driver cannot block listener close.
      let _ = tokio::time::timeout(shutdown_timeout, endpoint.wait_idle()).await;

      socket.unbind().await;
      Ok(())
    })
  }

  #[napi]
  pub fn stats(&self) -> stats::EndpointStats {
    let stats = self.endpoint.stats();
    stats::EndpointStats::new(
      stats.accepted_handshakes,
      stats.outgoing_handshakes,
      stats.refused_handshakes,
      stats.ignored_handshakes,
      self.failed_handshakes.load(Ordering::Relaxed),
      self.retried_handshakes.load(Ordering::Relaxed),
    )
  }
}

#[napi]
pub struct Client {
  endpoint: quinn::Endpoint,
  handshake_timeout: Duration,
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
    Ok(Client { endpoint, handshake_timeout: config.handshake_timeout })
  }

  #[napi(ts_return_type = "Promise<Connection>")]
  pub fn outbound_connection<'env>(
    &self,
    env: &'env Env,
    ip: String,
    port: u16,
  ) -> Result<PromiseRaw<'env, Connection>> {
    env.spawn_future(dial(self.endpoint.clone(), self.handshake_timeout, ip, port))
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
      0,
      0,
    )
  }
}

#[napi]
pub struct Connection {
  connection: quinn::Connection,
  peer_id: libp2p_identity::PeerId,
  aborted: tokio::sync::watch::Sender<bool>,
}

#[napi]
impl Connection {
  pub fn new(connection: quinn::Connection) -> Result<Self> {
    let peer_id = match peer_id(&connection) {
      Ok(peer_id) => peer_id,
      Err(e) => {
        connection.close(0u8.into(), b"");
        return Err(e);
      }
    };
    let (aborted, _) = tokio::sync::watch::channel(false);
    Ok(Self { connection, peer_id, aborted })
  }

  #[napi(ts_return_type = "Promise<Stream>")]
  pub fn inbound_stream<'env>(&self, env: &'env Env) -> Result<PromiseRaw<'env, Stream>> {
    let connection = self.connection.clone();
    env.spawn_future(async move {
      let (send, recv) = connection.accept_bi().await.map_err(to_err)?;
      Ok(Stream::new(send, recv))
    })
  }

  #[napi(ts_return_type = "Promise<Stream>")]
  pub fn outbound_stream<'env>(&self, env: &'env Env) -> Result<PromiseRaw<'env, Stream>> {
    let connection = self.connection.clone();
    env.spawn_future(async move {
      let (send, recv) = connection.open_bi().await.map_err(to_err)?;
      Ok(Stream::new(send, recv))
    })
  }

  #[napi]
  /// close the connection immediately
  pub fn abort(&self) {
    self.connection.close(0u8.into(), b"");
    // `send_replace` rather than `send`, the latter drops the value when there is no receiver yet
    self.aborted.send_replace(true);
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
      self.peer_id.to_base58()
    )
  }

  #[napi(ts_return_type = "Promise<void>")]
  /// Resolves when the connection closes, or as soon as `abort()` is called.
  ///
  /// Signalling local abort ensures the JS transport observes the closure even if
  /// the connection drivers do not finish shutting down.
  pub fn closed<'env>(&self, env: &'env Env) -> Result<PromiseRaw<'env, ()>> {
    let mut aborted = self.aborted.subscribe();
    let connection = self.connection.clone();
    env.spawn_future(async move {
      if *aborted.borrow_and_update() {
        return Ok(());
      }

      tokio::select! {
        _ = connection.closed() => {}
        _ = aborted.changed() => {}
      }
      Ok(())
    })
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

  #[napi(ts_return_type = "Promise<Uint8Array | null>")]
  pub fn read<'env>(
    &self,
    env: &'env Env,
    max_len: u32,
  ) -> Result<PromiseRaw<'env, Option<Uint8Array>>> {
    let recv = self.recv.clone();
    env.spawn_future(async move {
      let mut buf = vec![0u8; max_len as usize];
      let chunk = recv.lock().await.read(&mut buf).await.map_err(to_err)?;
      match chunk {
        Some(len) => {
          buf.truncate(len);
          Ok(Some(buf.into()))
        },
        None => Ok(None),
      }
    })
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn write<'env>(
    &self,
    env: &'env Env,
    data: Uint8Array,
  ) -> Result<PromiseRaw<'env, WriteResult>> {
    let send = self.send.clone();
    env.spawn_future(async move {
      let error = send.lock().await.write_all(&data).await
        .err().map(|e| e.to_string());
      // Always return Ok so the Uint8Array reaches the JS thread for safe
      // napi_ref cleanup.  WriteResult::to_napi_value handles both cleanup
      // and error propagation (rejecting the promise when error is Some).
      Ok(WriteResult { data, error })
    })
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn finish_write<'env>(&self, env: &'env Env) -> Result<PromiseRaw<'env, ()>> {
    let send = self.send.clone();
    env.spawn_future(async move {
      let _ = send.lock().await.finish();
      Ok(())
    })
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn reset_write<'env>(&self, env: &'env Env) -> Result<PromiseRaw<'env, ()>> {
    let send = self.send.clone();
    env.spawn_future(async move {
      let _ = send.lock().await.reset(0u8.into());
      Ok(())
    })
  }

  #[napi(ts_return_type = "Promise<void>")]
  pub fn stop_read<'env>(&self, env: &'env Env) -> Result<PromiseRaw<'env, ()>> {
    let recv = self.recv.clone();
    env.spawn_future(async move {
      let _ = recv.lock().await.stop(0u8.into());
      Ok(())
    })
  }
}

/// Wrapper that smuggles a Uint8Array back to the JS thread even when the
/// write operation fails.  The async fn always returns `Ok(WriteResult)`,
/// guaranteeing the Uint8Array is never dropped on the Tokio thread.
///
/// `ToNapiValue` (which runs on the JS thread) cleans up the napi_ref and
/// then either resolves (success) or rejects (error) the promise.
pub struct WriteResult {
  data: Uint8Array,
  error: Option<String>,
}

impl ToNapiValue for WriteResult {
  unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> napi::Result<napi::sys::napi_value> {
    // Always clean up the Uint8Array's napi_ref on the JS thread.
    // to_napi_value consumes the Uint8Array, deleting its reference.
    let _ = Uint8Array::to_napi_value(env, val.data);

    // If the write failed, return Err to reject the promise.
    if let Some(err) = val.error {
      return Err(napi::Error::new(Status::Unknown, err));
    }

    let mut result = std::ptr::null_mut();
    let _ = napi::check_status!(napi::sys::napi_get_undefined(env, &mut result));
    Ok(result)
  }
}

async fn dial(endpoint: quinn::Endpoint, handshake_timeout: Duration, ip: String, port: u16) -> Result<Connection> {
  let ip_addr = ip.parse::<IpAddr>().map_err(to_err)?;
  let socket_addr = SocketAddr::new(ip_addr, port);
  let connecting = endpoint.connect(socket_addr, "l").map_err(to_err)?;
  // on timeout the handshake future is dropped, which closes the connection
  let connection = tokio::time::timeout(handshake_timeout, connecting)
    .await
    .map_err(|_| to_err("handshake timed out"))?
    .map_err(to_err)?;

  Connection::new(connection)
}

// taken from rust-libp2p-quic
fn peer_id(connection: &quinn::Connection) -> Result<libp2p_identity::PeerId> {
  let identity = connection
    .peer_identity()
    .ok_or_else(|| to_err("connection has no peer identity"))?;
  let certificates: Box<Vec<rustls::pki_types::CertificateDer>> = identity
    .downcast()
    .map_err(|_| to_err("unexpected peer identity type"))?;
  let end_entity = certificates
    .first()
    .ok_or_else(|| to_err("connection has no peer certificate"))?;
  let p2p_cert = libp2p_tls::certificate::parse(end_entity).map_err(to_err)?;
  Ok(p2p_cert.peer_id())
}

fn to_err<T: ToString>(str: T) -> napi::Error {
  napi::Error::new(Status::Unknown, str)
}
