/**
 * @packageDocumentation
 *
 * A [libp2p transport](https://docs.libp2p.io/concepts/transports/overview/) based on the QUIC networking stack.
 *
 * @example
 *
 * ```TypeScript
 * import { createLibp2p } from 'libp2p'
 * import { quic } from '@chainsafe/libp2p-quic'
 * import { multiaddr } from '@multiformats/multiaddr'
 *
 * const node = await createLibp2p({
 *   transports: [
 *     quic()
 *   ]
 * })
 *
 * const ma = multiaddr('/ip4/123.123.123.123/udp/1234/quic-v1')
 *
 * // dial a TCP connection, timing out after 10 seconds
 * const connection = await node.dial(ma, {
 *   signal: AbortSignal.timeout(10_000)
 * })
 *
 * // use connection...
 * ```
 */

import { QuicTransport } from './transport.js'
import type * as napi from './napi.js'
import type { ComponentLogger, DialTransportOptions, Metrics, PrivateKey, Transport } from '@libp2p/interface'

export type QuicOptions = Omit<napi.Config, 'privateKeyProto'> & {
  /**
   * Enable IPv4 QUIC client for outbound connections.
   * When set to false, IPv4 connections will not be supported.
   *
   * @default true
   */
  ipv4?: boolean

  /**
   * Enable IPv6 QUIC client for outbound connections.
   * When set to false, IPv6 connections will not be supported.
   *
   * @default true
   */
  ipv6?: boolean
}

export interface QuicComponents {
  metrics?: Metrics
  logger: ComponentLogger
  privateKey: PrivateKey
}

export type QuicDialOptions = DialTransportOptions

export function quic(options?: Partial<QuicOptions>): (components: QuicComponents) => Transport {
  return (components) => new QuicTransport(components, { ...defaultOptions, ...options })
}

export const defaultOptions: QuicOptions = {
  handshakeTimeout: 5_000,
  /// The 3s default is derived from QUIC's draining period:
  ///
  /// quinn arms each connection's close/drain timer at 3x PTO (RFC 9000 §10.2)
  ///
  /// PTO = RTT + 4x RTT variance + the peer's max_ack_delay
  ///
  /// RTT = 333ms
  /// RTT variance: RTT / 2
  /// PTO = 333 ms + 4 × 166.5 ms = 999 ms
  /// 3 * PTO = roughly 3s
  ///
  /// This timeout can only trip when a connection driver has stopped unexpectedly.
  /// Waiting longer cannot help — an unbounded wait
  /// hangs the caller's entire shutdown (see ChainSafe/lodestar#9744).
  shutdownTimeout: 3_000,
  maxIdleTimeout: 10_000,
  keepAliveInterval: 5_000,
  maxConcurrentStreamLimit: 256,
  maxStreamData: 10_000_000,
  maxConnectionData: 15_000_000,
  receiveBufferSize: 500_000,
  sendBufferSize: 500_000
}
