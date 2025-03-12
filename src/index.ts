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

import { QuicTransport, type QuicComponents, type QuicOptions } from './transport.js'
import type { Transport } from '@libp2p/interface'

export function quic (options?: Partial<QuicOptions>): (components: QuicComponents) => Transport {
  return (components) => new QuicTransport(components, { ...defaultOptions, ...options })
}

export const defaultOptions: QuicOptions = {
  handshakeTimeout: 5_000,
  maxIdleTimeout: 10_000,
  keepAliveInterval: 5_000,
  maxConcurrentStreamLimit: 256,
  maxStreamData: 10_000_000,
  maxConnectionData: 15_000_000
}
