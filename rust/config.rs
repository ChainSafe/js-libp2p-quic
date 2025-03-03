#![deny(clippy::all)]

use std::{sync::Arc, time::Duration};

use libp2p_identity::DecodingError;
use libp2p_tls::certificate::GenError;
use napi::bindgen_prelude::Uint8Array;
use napi_derive::napi;

// Config code mostly copied from rust-libp2p quic
// with some minor tweaks and simplifications

#[derive(Debug)]
pub enum ConfigError {
  InvalidPrivateKey(DecodingError),
  CertificateGeneration(GenError),
  NoInitialCipherSuite,
}

pub enum ConfigErrorCode {
  InvalidPrivateKey,
  CertificateGeneration,
  NoInitialCipherSuite,
}

impl AsRef<str> for ConfigErrorCode {
  fn as_ref(&self) -> &str {
    match self {
      ConfigErrorCode::InvalidPrivateKey => "INVALID_PRIVATE_KEY",
      ConfigErrorCode::CertificateGeneration => "CERTIFICATE_GENERATION",
      ConfigErrorCode::NoInitialCipherSuite => "NO_INITIAL_CIPHER_SUITE",
    }
  }
}

impl From<ConfigError> for napi::Error<ConfigErrorCode> {
  fn from(err: ConfigError) -> Self {
    match err {
      ConfigError::InvalidPrivateKey(e) => napi::Error::new(
        ConfigErrorCode::InvalidPrivateKey,
        format!("Invalid private key: {}", e),
      ),
      ConfigError::CertificateGeneration(e) => napi::Error::new(
        ConfigErrorCode::CertificateGeneration,
        format!("Certificate generation error: {}", e),
      ),
      ConfigError::NoInitialCipherSuite => napi::Error::new(
        ConfigErrorCode::NoInitialCipherSuite,
        "No initial cipher suite",
      ),
    }
  }
}

// All errors returned from this module will be of type `napi::Error<ConfigErrorCode>`
type Result<T> = std::result::Result<T, napi::Error<ConfigErrorCode>>;

#[napi(object)]
/// User-level configuration
pub struct Config {
  /// Libp2p identity of the node, protobuf encoded.
  pub private_key_proto: Uint8Array,

  /// Timeout for the initial handshake when establishing a connection.
  /// The actual timeout is the minimum of this and the [`Config::max_idle_timeout`].
  pub handshake_timeout: u32,
  /// Maximum duration of inactivity in ms to accept before timing out the connection.
  pub max_idle_timeout: u32,
  /// Period of inactivity before sending a keep-alive packet.
  /// Must be set lower than the idle_timeout of both
  /// peers to be effective.
  ///
  /// See [`quinn::TransportConfig::keep_alive_interval`] for more
  /// info.
  pub keep_alive_interval: u32,
  /// Maximum number of incoming bidirectional streams that may be open
  /// concurrently by the remote peer.
  pub max_concurrent_stream_limit: u32,

  /// Max unacknowledged data in bytes that may be sent on a single stream.
  pub max_stream_data: u32,

  /// Max unacknowledged data in bytes that may be sent in total on all streams
  /// of a connection.
  pub max_connection_data: u32,
}

/// Configuration used by the QUIC library
#[derive(Clone)]
#[napi]
pub struct QuinnConfig {
  pub(crate) client_config: quinn::ClientConfig,
  pub(crate) server_config: quinn::ServerConfig,
  pub(crate) endpoint_config: quinn::EndpointConfig,
}

#[napi]
impl QuinnConfig {
  #[napi(constructor)]
  pub fn new(config: Config) -> Result<Self> {
    Ok(QuinnConfig::try_from(config)?)
  }
}

impl TryFrom<Config> for QuinnConfig {
  type Error = napi::Error<ConfigErrorCode>;
  fn try_from(config: Config) -> Result<QuinnConfig> {
    let Config {
      private_key_proto,
      max_idle_timeout,
      max_concurrent_stream_limit,
      keep_alive_interval,
      max_connection_data,
      max_stream_data,
      handshake_timeout: _,
    } = config;

    let keypair = libp2p_identity::Keypair::from_protobuf_encoding(&private_key_proto)
      .map_err(|e| ConfigError::InvalidPrivateKey(e))?;

    let mut transport = quinn::TransportConfig::default();
    // Disable uni-directional streams.
    transport.max_concurrent_uni_streams(0u32.into());
    transport.max_concurrent_bidi_streams(max_concurrent_stream_limit.into());
    // Disable datagrams.
    transport.datagram_receive_buffer_size(None);
    transport.keep_alive_interval(Some(Duration::from_millis(keep_alive_interval.into())));
    transport.max_idle_timeout(Some(quinn::VarInt::from_u32(max_idle_timeout).into()));
    transport.allow_spin(false);
    transport.stream_receive_window(max_stream_data.into());
    transport.receive_window(max_connection_data.into());
    transport.mtu_discovery_config(Default::default());
    let transport = Arc::new(transport);

    let client_tls_config = Arc::new(
      quinn::crypto::rustls::QuicClientConfig::try_from(
        libp2p_tls::make_client_config(&keypair, None)
          .map_err(|e| ConfigError::CertificateGeneration(e))?,
      )
      .map_err(|_| ConfigError::NoInitialCipherSuite)?,
    );
    let server_tls_config = Arc::new(
      quinn::crypto::rustls::QuicServerConfig::try_from(
        libp2p_tls::make_server_config(&keypair)
          .map_err(|e| ConfigError::CertificateGeneration(e))?,
      )
      .map_err(|_| ConfigError::NoInitialCipherSuite)?,
    );

    let mut server_config = quinn::ServerConfig::with_crypto(server_tls_config);
    server_config.transport = Arc::clone(&transport);
    // Disables connection migration.
    // Long-term this should be enabled, however we then need to handle address change
    // on connections in the `Connection`.
    server_config.migration(false);

    let mut client_config = quinn::ClientConfig::new(client_tls_config);
    client_config.transport_config(transport);

    let mut endpoint_config = keypair
      .derive_secret(b"libp2p quic stateless reset key")
      .map(|secret| {
        let reset_key = Arc::new(ring::hmac::Key::new(ring::hmac::HMAC_SHA256, &secret));
        quinn::EndpointConfig::new(reset_key)
      })
      .unwrap_or_default();
    endpoint_config.supported_versions(vec![1]);

    Ok(QuinnConfig {
      client_config,
      server_config,
      endpoint_config,
    })
  }
}
