#![deny(clippy::all)]

use napi_derive::napi;

// Quinn exposes a loooot of stats
// Are they all useful? who knows, but we expose them all

#[napi]
pub struct ConnectionStats {
  // UDP statistics
  /// Statistics about UDP datagrams transmitted on a connection
  ///
  /// The amount of UDP datagrams observed
  pub udp_tx_datagrams: u32,
  /// Statistics about UDP datagrams transmitted on a connection
  ///
  /// The total amount of bytes which have been transferred inside UDP datagrams
  pub udp_tx_bytes: u32,
  /// Statistics about UDP datagrams transmitted on a connection
  ///
  /// The amount of I/O operations executed
  ///
  /// Can be less than `datagrams` when GSO, GRO, and/or batched system calls are in use.
  pub udp_tx_ios: u32,
  /// Statistics about UDP datagrams received on a connection
  ///
  /// The amount of UDP datagrams observed
  pub udp_rx_datagrams: u32,
  /// Statistics about UDP datagrams received on a connection
  ///
  /// The total amount of bytes which have been transferred inside UDP datagrams
  pub udp_rx_bytes: u32,
  /// Statistics about UDP datagrams received on a connection
  ///
  /// The amount of I/O operations executed
  ///
  /// Can be less than `datagrams` when GSO, GRO, and/or batched system calls are in use.
  pub udp_rx_ios: u32,

  // Frame statistics
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_acks: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_ack_frequency: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_crypto: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_connection_close: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_data_blocked: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_datagram: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_handshake_done: u8,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_immediate_ack: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_max_data: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_max_stream_data: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_max_streams_bidi: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_max_streams_uni: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_new_connection_id: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_new_token: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_path_challenge: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_path_response: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_ping: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_reset_stream: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_retire_connection_id: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_stream_data_blocked: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_streams_blocked_bidi: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_streams_blocked_uni: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_stop_sending: u32,
  /// Statistics about frames transmitted on a connection
  /// Number of frames transmitted of this frame type
  pub frame_tx_stream: u32,

  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_acks: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_ack_frequency: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_crypto: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_connection_close: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_data_blocked: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_datagram: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_handshake_done: u8,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_immediate_ack: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_max_data: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_max_stream_data: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_max_streams_bidi: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_max_streams_uni: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_new_connection_id: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_new_token: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_path_challenge: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_path_response: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_ping: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_reset_stream: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_retire_connection_id: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_stream_data_blocked: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_streams_blocked_bidi: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_streams_blocked_uni: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_stop_sending: u32,
  /// Statistics about frames received on a connection
  /// Number of frames transmitted of this frame type
  pub frame_rx_stream: u32,

  // Path statistics
  /// Current best estimate of this connection's latency (round-trip-time)
  pub path_rtt: u32,
  /// Current congestion window of the connection
  pub path_cwnd: u32,
  /// Congestion events on the connection
  pub path_congestion_events: u32,
  /// The amount of packets lost on this path
  pub path_lost_packets: u32,
  /// The amount of bytes lost on this path
  pub path_lost_bytes: u32,
  /// The amount of packets sent on this path
  pub path_sent_packets: u32,
  /// The amount of PLPMTUD probe packets sent on this path (also counted by `sent_packets`)
  pub path_sent_plpmtud_probes: u32,
  /// The amount of PLPMTUD probe packets lost on this path (ignored by `lost_packets` and
  /// `lost_bytes`)
  pub path_lost_plpmtud_probes: u32,
  /// The number of times a black hole was detected in the path
  pub path_black_holes_detected: u32,
}

impl From<quinn::ConnectionStats> for ConnectionStats {
  fn from(stats: quinn::ConnectionStats) -> Self {
    Self {
      udp_tx_datagrams: stats.udp_tx.datagrams as u32,
      udp_tx_bytes: stats.udp_tx.bytes as u32,
      udp_tx_ios: stats.udp_tx.ios as u32,
      udp_rx_datagrams: stats.udp_rx.datagrams as u32,
      udp_rx_bytes: stats.udp_rx.bytes as u32,
      udp_rx_ios: stats.udp_rx.ios as u32,
      frame_tx_acks: stats.frame_tx.acks as u32,
      frame_tx_ack_frequency: stats.frame_tx.ack_frequency as u32,
      frame_tx_crypto: stats.frame_tx.crypto as u32,
      frame_tx_connection_close: stats.frame_tx.connection_close as u32,
      frame_tx_data_blocked: stats.frame_tx.data_blocked as u32,
      frame_tx_datagram: stats.frame_tx.datagram as u32,
      frame_tx_handshake_done: stats.frame_tx.handshake_done as u8,
      frame_tx_immediate_ack: stats.frame_tx.immediate_ack as u32,
      frame_tx_max_data: stats.frame_tx.max_data as u32,
      frame_tx_max_stream_data: stats.frame_tx.max_stream_data as u32,
      frame_tx_max_streams_bidi: stats.frame_tx.max_streams_bidi as u32,
      frame_tx_max_streams_uni: stats.frame_tx.max_streams_uni as u32,
      frame_tx_new_connection_id: stats.frame_tx.new_connection_id as u32,
      frame_tx_new_token: stats.frame_tx.new_token as u32,
      frame_tx_path_challenge: stats.frame_tx.path_challenge as u32,
      frame_tx_path_response: stats.frame_tx.path_response as u32,
      frame_tx_ping: stats.frame_tx.ping as u32,
      frame_tx_reset_stream: stats.frame_tx.reset_stream as u32,
      frame_tx_retire_connection_id: stats.frame_tx.retire_connection_id as u32,
      frame_tx_stream_data_blocked: stats.frame_tx.stream_data_blocked as u32,
      frame_tx_streams_blocked_bidi: stats.frame_tx.streams_blocked_bidi as u32,
      frame_tx_streams_blocked_uni: stats.frame_tx.streams_blocked_uni as u32,
      frame_tx_stop_sending: stats.frame_tx.stop_sending as u32,
      frame_tx_stream: stats.frame_tx.stream as u32,
      frame_rx_acks: stats.frame_rx.acks as u32,
      frame_rx_ack_frequency: stats.frame_rx.ack_frequency as u32,
      frame_rx_crypto: stats.frame_rx.crypto as u32,
      frame_rx_connection_close: stats.frame_rx.connection_close as u32,
      frame_rx_data_blocked: stats.frame_rx.data_blocked as u32,
      frame_rx_datagram: stats.frame_rx.datagram as u32,
      frame_rx_handshake_done: stats.frame_rx.handshake_done as u8,
      frame_rx_immediate_ack: stats.frame_rx.immediate_ack as u32,
      frame_rx_max_data: stats.frame_rx.max_data as u32,
      frame_rx_max_stream_data: stats.frame_rx.max_stream_data as u32,
      frame_rx_max_streams_bidi: stats.frame_rx.max_streams_bidi as u32,
      frame_rx_max_streams_uni: stats.frame_rx.max_streams_uni as u32,
      frame_rx_new_connection_id: stats.frame_rx.new_connection_id as u32,
      frame_rx_new_token: stats.frame_rx.new_token as u32,
      frame_rx_path_challenge: stats.frame_rx.path_challenge as u32,
      frame_rx_path_response: stats.frame_rx.path_response as u32,
      frame_rx_ping: stats.frame_rx.ping as u32,
      frame_rx_reset_stream: stats.frame_rx.reset_stream as u32,
      frame_rx_retire_connection_id: stats.frame_rx.retire_connection_id as u32,
      frame_rx_stream_data_blocked: stats.frame_rx.stream_data_blocked as u32,
      frame_rx_streams_blocked_bidi: stats.frame_rx.streams_blocked_bidi as u32,
      frame_rx_streams_blocked_uni: stats.frame_rx.streams_blocked_uni as u32,
      frame_rx_stop_sending: stats.frame_rx.stop_sending as u32,
      frame_rx_stream: stats.frame_rx.stream as u32,
      path_rtt: stats.path.rtt.as_millis() as u32,
      path_cwnd: stats.path.cwnd as u32,
      path_congestion_events: stats.path.congestion_events as u32,
      path_lost_packets: stats.path.lost_packets as u32,
      path_lost_bytes: stats.path.lost_bytes as u32,
      path_sent_packets: stats.path.sent_packets as u32,
      path_sent_plpmtud_probes: stats.path.sent_plpmtud_probes as u32,
      path_lost_plpmtud_probes: stats.path.lost_plpmtud_probes as u32,
      path_black_holes_detected: stats.path.black_holes_detected as u32,
    }
  }
}
