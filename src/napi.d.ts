/* auto-generated by NAPI-RS */
/* eslint-disable */
export declare class Client {
  constructor(config: QuinnConfig, family: SocketFamily)
  outboundConnection(ip: string, port: number): Promise<Connection>
  abort(): void
  stats(): EndpointStats
}

export declare class Connection {
  inboundStream(): Promise<Stream>
  outboundStream(): Promise<Stream>
  /** close the connection immediately */
  abort(): void
  rtt(): number
  id(): string
  remoteMultiaddr(): string
  closed(): Promise<void>
  stats(): ConnectionStats
}

export declare class ConnectionStats {
  /**
   * Statistics about UDP datagrams transmitted on a connection
   *
   * The amount of UDP datagrams observed
   */
  udpTxDatagrams: number
  /**
   * Statistics about UDP datagrams transmitted on a connection
   *
   * The total amount of bytes which have been transferred inside UDP datagrams
   */
  udpTxBytes: number
  /**
   * Statistics about UDP datagrams transmitted on a connection
   *
   * The amount of I/O operations executed
   *
   * Can be less than `datagrams` when GSO, GRO, and/or batched system calls are in use.
   */
  udpTxIos: number
  /**
   * Statistics about UDP datagrams received on a connection
   *
   * The amount of UDP datagrams observed
   */
  udpRxDatagrams: number
  /**
   * Statistics about UDP datagrams received on a connection
   *
   * The total amount of bytes which have been transferred inside UDP datagrams
   */
  udpRxBytes: number
  /**
   * Statistics about UDP datagrams received on a connection
   *
   * The amount of I/O operations executed
   *
   * Can be less than `datagrams` when GSO, GRO, and/or batched system calls are in use.
   */
  udpRxIos: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxAcks: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxAckFrequency: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxCrypto: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxConnectionClose: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxDataBlocked: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxDatagram: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxHandshakeDone: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxImmediateAck: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxMaxData: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxMaxStreamData: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxMaxStreamsBidi: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxMaxStreamsUni: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxNewConnectionId: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxNewToken: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxPathChallenge: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxPathResponse: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxPing: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxResetStream: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxRetireConnectionId: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxStreamDataBlocked: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxStreamsBlockedBidi: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxStreamsBlockedUni: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxStopSending: number
  /**
   * Statistics about frames transmitted on a connection
   * Number of frames transmitted of this frame type
   */
  frameTxStream: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxAcks: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxAckFrequency: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxCrypto: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxConnectionClose: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxDataBlocked: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxDatagram: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxHandshakeDone: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxImmediateAck: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxMaxData: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxMaxStreamData: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxMaxStreamsBidi: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxMaxStreamsUni: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxNewConnectionId: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxNewToken: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxPathChallenge: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxPathResponse: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxPing: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxResetStream: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxRetireConnectionId: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxStreamDataBlocked: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxStreamsBlockedBidi: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxStreamsBlockedUni: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxStopSending: number
  /**
   * Statistics about frames received on a connection
   * Number of frames transmitted of this frame type
   */
  frameRxStream: number
  /** Current best estimate of this connection's latency (round-trip-time) */
  pathRtt: number
  /** Current congestion window of the connection */
  pathCwnd: number
  /** Congestion events on the connection */
  pathCongestionEvents: number
  /** The amount of packets lost on this path */
  pathLostPackets: number
  /** The amount of bytes lost on this path */
  pathLostBytes: number
  /** The amount of packets sent on this path */
  pathSentPackets: number
  /** The amount of PLPMTUD probe packets sent on this path (also counted by `sent_packets`) */
  pathSentPlpmtudProbes: number
  /**
   * The amount of PLPMTUD probe packets lost on this path (ignored by `lost_packets` and
   * `lost_bytes`)
   */
  pathLostPlpmtudProbes: number
  /** The number of times a black hole was detected in the path */
  pathBlackHolesDetected: number
}

export declare class EndpointStats {
  /** Cummulative number of Quic handshakes accepted by this Endpoint */
  acceptedHandshakes: number
  /** Cummulative number of Quic handshakees sent from this Endpoint */
  outgoingHandshakes: number
  /** Cummulative number of Quic handshakes refused on this Endpoint */
  refusedHandshakes: number
  /** Cummulative number of Quic handshakes ignored on this Endpoint */
  ignoredHandshakes: number
}

/** Configuration used by the QUIC library */
export declare class QuinnConfig {
  constructor(config: Config)
}

export declare class Server {
  constructor(config: QuinnConfig, ip: string, port: number)
  port(): number
  inboundConnection(): Promise<Connection>
  abort(): Promise<void>
  stats(): EndpointStats
}

export declare class Stream {
  id(): string
  read(buf: Uint8Array): Promise<number | null>
  read2(): Promise<Uint8Array | null>
  write(data: Uint8Array): Promise<void>
  finishWrite(): Promise<void>
  resetWrite(): Promise<void>
  stopRead(): Promise<void>
}

/** User-level configuration */
export interface Config {
  /** Libp2p identity of the node, protobuf encoded. */
  privateKeyProto: Uint8Array
  /**
   * Timeout for the initial handshake when establishing a connection.
   * The actual timeout is the minimum of this and the [`Config::max_idle_timeout`].
   */
  handshakeTimeout: number
  /** Maximum duration of inactivity in ms to accept before timing out the connection. */
  maxIdleTimeout: number
  /**
   * Period of inactivity before sending a keep-alive packet.
   * Must be set lower than the idle_timeout of both
   * peers to be effective.
   *
   * See [`quinn::TransportConfig::keep_alive_interval`] for more
   * info.
   */
  keepAliveInterval: number
  /**
   * Maximum number of incoming bidirectional streams that may be open
   * concurrently by the remote peer.
   */
  maxConcurrentStreamLimit: number
  /**
   * Maximum number of bytes the peer may transmit without acknowledgement on any one stream
   * before becoming blocked.
   *
   * This should be set to at least the expected connection latency multiplied by the maximum
   * desired throughput. Setting this smaller than `max_connection_data` helps ensure that a single
   * stream doesn't monopolize receive buffers, which may otherwise occur if the application
   * chooses not to read from a large stream for a time while still requiring data on other
   * streams.
   */
  maxStreamData: number
  /**
   * Maximum number of bytes the peer may transmit across all streams of a connection before
   * becoming blocked.
   *
   * This should be set to at least the expected connection latency multiplied by the maximum
   * desired throughput. Larger values can be useful to allow maximum throughput within a
   * stream while another is blocked.
   */
  maxConnectionData: number
  /**
   * OS socket receive buffer size.
   *
   * If this is set higher than the OS maximum, it will be clamped to the maximum allowed size.
   */
  receiveBufferSize: number
  /**
   * OS socket send buffer size.
   *
   * If this is set higher than the OS maximum, it will be clamped to the maximum allowed size.
   */
  sendBufferSize: number
}

export declare const enum SocketFamily {
  Ipv4 = 0,
  Ipv6 = 1
}
