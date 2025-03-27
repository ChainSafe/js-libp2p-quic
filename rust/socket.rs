// Unbinding a UDP socket is controlled by dropping the std::net::UdpSocket.
// Quinn passes a reference the bound UDP socket to all connections and streams.
// This becomes problematic when trying to fully close a server
// because the socket will still be referenced by any existing connections/streams
// and so will not be dropped and thus will not be unbound.
//
// Also, we don't have control over when these structs will be GC'd on the JS side.
// In order to ensure that the socket is unbound, we need to create a wrapped socket
// that can be manually dropped.
//
// Most of the code in this file is modified from the quinn source code,
// taking their existing UdpSocket struct and wrapping it in a RwLock<Option>
// that can be explicitly dropped.

use std::{
  future::Future,
  io,
  pin::Pin,
  sync::Arc,
  task::{Context, Poll},
};

use quinn::udp;
use quinn::{AsyncUdpSocket, UdpPoller};
use tokio::{io::Interest, sync::RwLock};
use socket2::Socket;

use crate::config;

pub fn create_socket(config: config::SocketConfig, socket_addr: std::net::SocketAddr) -> napi::Result<std::net::UdpSocket> {
    let socket = std::net::UdpSocket::bind(socket_addr)?;

    let socket = Socket::from(socket);
    socket.set_send_buffer_size(config.send_buffer_size as usize).unwrap();
    socket.set_recv_buffer_size(config.receive_buffer_size as usize).unwrap();
    let socket = std::net::UdpSocket::from(socket);
    Ok(socket)
}

#[derive(Debug)]
pub struct UdpSocket {
  inner: RwLock<Option<UdpSocketInner>>,
}

impl UdpSocket {
  pub fn wrap_udp_socket(sock: std::net::UdpSocket) -> io::Result<Arc<UdpSocket>> {
    let socket = UdpSocket {
      inner: RwLock::new(Some(UdpSocketInner {
        inner: udp::UdpSocketState::new((&sock).into())?,
        io: tokio::net::UdpSocket::from_std(sock)?,
      })),
    };
    Ok(Arc::new(socket))
  }

  pub async fn unbind(&self) {
    if let Some(socket) = self.inner.write().await.take() {
      drop(socket);
    }
  }
}

impl AsyncUdpSocket for UdpSocket {
  fn create_io_poller(self: Arc<Self>) -> Pin<Box<dyn UdpPoller>> {
    Box::pin(UdpPollHelper::new(move || {
      let socket = self.clone();
      async move {
        let inner = socket.inner.read().await;
        match &*inner {
          Some(inner) => inner.io.writable().await,
          None => Err(io::Error::new(io::ErrorKind::Other, "Socket not bound")),
        }
      }
    }))
  }

  fn try_send(&self, transmit: &udp::Transmit) -> io::Result<()> {
    let inner = self
      .inner
      .try_read()
      .map_err(|_| io::Error::new(io::ErrorKind::Other, "Can't acquire lock"))?;
    match &*inner {
      Some(inner) => inner.try_send(transmit),
      None => Err(io::Error::new(io::ErrorKind::Other, "Socket not bound")),
    }
  }

  fn poll_recv(
    &self,
    cx: &mut Context,
    bufs: &mut [io::IoSliceMut<'_>],
    meta: &mut [udp::RecvMeta],
  ) -> Poll<io::Result<usize>> {
    let inner = self
      .inner
      .try_read()
      .map_err(|_| io::Error::new(io::ErrorKind::Other, "Can't acquire lock"))?;
    match &*inner {
      Some(inner) => inner.poll_recv(cx, bufs, meta),
      None => Poll::Ready(Err(io::Error::new(
        io::ErrorKind::Other,
        "Socket not bound",
      ))),
    }
  }

  fn local_addr(&self) -> io::Result<std::net::SocketAddr> {
    let inner = self
      .inner
      .try_read()
      .map_err(|_| io::Error::new(io::ErrorKind::Other, "Can't acquire lock"))?;
    match &*inner {
      Some(inner) => inner.local_addr(),
      None => Err(io::Error::new(io::ErrorKind::Other, "Socket not bound")),
    }
  }

  fn may_fragment(&self) -> bool {
    if let Ok(inner) = self.inner.try_read() {
      match &*inner {
        Some(inner) => inner.may_fragment(),
        None => true,
      }
    } else {
      true
    }
  }

  fn max_transmit_segments(&self) -> usize {
    if let Ok(inner) = self.inner.try_read() {
      match &*inner {
        Some(inner) => inner.max_transmit_segments(),
        None => 1,
      }
    } else {
      1
    }
  }

  fn max_receive_segments(&self) -> usize {
    if let Ok(inner) = self.inner.try_read() {
      match &*inner {
        Some(inner) => inner.max_receive_segments(),
        None => 1,
      }
    } else {
      1
    }
  }
}

// Below mostly copied from quinn

macro_rules! ready {
  ($e:expr $(,)?) => {
    match $e {
      std::task::Poll::Ready(t) => t,
      std::task::Poll::Pending => return std::task::Poll::Pending,
    }
  };
}

#[derive(Debug)]
struct UdpSocketInner {
  io: tokio::net::UdpSocket,
  inner: udp::UdpSocketState,
}

impl UdpSocketInner {
  fn try_send(&self, transmit: &udp::Transmit) -> io::Result<()> {
    self.io.try_io(Interest::WRITABLE, || {
      self.inner.send((&self.io).into(), transmit)
    })
  }

  fn poll_recv(
    &self,
    cx: &mut Context,
    bufs: &mut [std::io::IoSliceMut<'_>],
    meta: &mut [udp::RecvMeta],
  ) -> Poll<io::Result<usize>> {
    loop {
      ready!(self.io.poll_recv_ready(cx))?;
      if let Ok(res) = self.io.try_io(Interest::READABLE, || {
        self.inner.recv((&self.io).into(), bufs, meta)
      }) {
        return Poll::Ready(Ok(res));
      }
    }
  }

  fn local_addr(&self) -> io::Result<std::net::SocketAddr> {
    self.io.local_addr()
  }

  fn may_fragment(&self) -> bool {
    self.inner.may_fragment()
  }

  fn max_transmit_segments(&self) -> usize {
    self.inner.max_gso_segments()
  }

  fn max_receive_segments(&self) -> usize {
    self.inner.gro_segments()
  }
}

pin_project_lite::pin_project! {
    /// Helper adapting a function `MakeFut` that constructs a single-use future `Fut` into a
    /// [`UdpPoller`] that may be reused indefinitely
    struct UdpPollHelper<MakeFut, Fut> {
        make_fut: MakeFut,
        #[pin]
        fut: Option<Fut>,
    }
}

impl<MakeFut, Fut> UdpPollHelper<MakeFut, Fut> {
  /// Construct a [`UdpPoller`] that calls `make_fut` to get the future to poll, storing it until
  /// it yields [`Poll::Ready`], then creating a new one on the next
  /// [`poll_writable`](UdpPoller::poll_writable)
  fn new(make_fut: MakeFut) -> Self {
    Self {
      make_fut,
      fut: None,
    }
  }
}

impl<MakeFut, Fut> UdpPoller for UdpPollHelper<MakeFut, Fut>
where
  MakeFut: Fn() -> Fut + Send + Sync + 'static,
  Fut: Future<Output = io::Result<()>> + Send + Sync + 'static,
{
  fn poll_writable(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
    let mut this = self.project();
    if this.fut.is_none() {
      this.fut.set(Some((this.make_fut)()));
    }
    // We're forced to `unwrap` here because `Fut` may be `!Unpin`, which means we can't safely
    // obtain an `&mut Fut` after storing it in `self.fut` when `self` is already behind `Pin`,
    // and if we didn't store it then we wouldn't be able to keep it alive between
    // `poll_writable` calls.
    let result = this.fut.as_mut().as_pin_mut().unwrap().poll(cx);
    if result.is_ready() {
      // Polling an arbitrary `Future` after it becomes ready is a logic error, so arrange for
      // a new `Future` to be created on the next call.
      this.fut.set(None);
    }
    result
  }
}

impl<MakeFut, Fut> std::fmt::Debug for UdpPollHelper<MakeFut, Fut> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("UdpPollHelper").finish_non_exhaustive()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Check how buffer sizes are set. On my (linux, ubuntu 24.04.2) machine, this is the output:
  ///
  /// send buffer size: 212992
  /// receive buffer size: 212992
  /// updating buffer sizes: 1000000
  /// send buffer size: 425984
  /// receive buffer size: 425984
  #[test]
  fn check_default_buffer_sizes() {
    let socket = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    let socket = Socket::from(socket);
    let send_buffer_size = socket.send_buffer_size().unwrap();
    let receive_buffer_size = socket.recv_buffer_size().unwrap();
    println!("send buffer size: {}", send_buffer_size);
    println!("receive buffer size: {}", receive_buffer_size);

    let new_size: usize = 1_000_000;
    println!("updating buffer sizes: {}", new_size);

    socket.set_send_buffer_size(new_size).unwrap();
    socket.set_recv_buffer_size(new_size).unwrap();

    let send_buffer_size = socket.send_buffer_size().unwrap();
    let receive_buffer_size = socket.recv_buffer_size().unwrap();
    println!("send buffer size: {}", send_buffer_size);
    println!("receive buffer size: {}", receive_buffer_size);
  }
}