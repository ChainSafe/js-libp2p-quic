import { privateKeyToProtobuf } from '@libp2p/crypto/keys'
import { AbortError, serviceCapabilities, transportSymbol } from '@libp2p/interface'
import { peerIdFromString } from '@libp2p/peer-id'
import { QuicConnection } from './connection.js'
import { dialFilter, listenFilter } from './filter.js'
import { QuicListener } from './listener.js'
import * as napi from './napi.js'
import { QuicStreamMuxerFactory } from './stream-muxer.js'
import { nodeAddressFromMultiaddr } from './utils.js'
import type { QuicComponents, QuicDialOptions, QuicOptions } from './index.js'
import type { QuicCreateListenerOptions } from './listener.js'
import type { Connection, CounterGroup, Listener, Logger, MultiaddrFilter, Transport } from '@libp2p/interface'
import type { Multiaddr } from '@multiformats/multiaddr'

interface QuicTransportMetrics {
  events?: CounterGroup
  errors?: CounterGroup
}

export class QuicTransport implements Transport {
  readonly [Symbol.toStringTag]: string = 'quic'
  readonly [transportSymbol] = true
  readonly [serviceCapabilities]: string[] = [
    '@libp2p/transport'
  ]

  readonly log: Logger
  readonly components: QuicComponents
  readonly metrics: QuicTransportMetrics

  readonly #config: napi.QuinnConfig

  readonly #clients: {
    ip4?: napi.Client
    ip6?: napi.Client
  }

  readonly listenFilter: MultiaddrFilter
  readonly dialFilter: MultiaddrFilter

  constructor (components: QuicComponents, options: QuicOptions) {
    const privateKeyProto = privateKeyToProtobuf(components.privateKey)
    const config = { ...options, privateKeyProto }

    this.log = components.logger.forComponent('libp2p:quic:transport')
    this.components = components

    this.#config = new napi.QuinnConfig(config)

    let ip4Client: napi.Client | undefined
    if (options.ipv4 !== false) {
      try {
        ip4Client = new napi.Client(this.#config, 0)
      } catch {
        this.log('IPv4 QUIC client not available')
      }
    }

    let ip6Client: napi.Client | undefined
    if (options.ipv6 !== false) {
      try {
        ip6Client = new napi.Client(this.#config, 1)
      } catch {
        this.log('IPv6 QUIC client not available')
      }
    }

    if (ip4Client == null && ip6Client == null) {
      throw new Error('At least one of ipv4 or ipv6 must be enabled for QUIC transport')
    }

    this.#clients = {
      ip4: ip4Client,
      ip6: ip6Client
    }

    this.metrics = {
      events: this.components.metrics?.registerCounterGroup('libp2p_quic_dialer_events_total', {
        label: 'event',
        help: 'Total count of QUIC dialer events by type'
      }),
      errors: this.components.metrics?.registerCounterGroup('libp2p_quic_dialer_errors_total', {
        label: 'event',
        help: 'Total count of QUIC dialer errors by type'
      })
    }

    this.listenFilter = listenFilter
    this.dialFilter = dialFilter

    this.log('new')
  }

  async dial (ma: Multiaddr, options: QuicDialOptions): Promise<Connection> {
    if (options.signal?.aborted) {
      throw new AbortError()
    }

    this.log('dialing', ma.toString())
    const addr = nodeAddressFromMultiaddr(ma)
    const dialer = addr.family === 4 ? this.#clients.ip4 : this.#clients.ip6

    if (dialer == null) {
      throw new Error(`No QUIC client available for IPv${addr.family}`)
    }

    const dialPromise = dialer.outboundConnection(addr.address, addr.port)
    dialPromise
      .then(() => this.metrics.events?.increment({ connect: true }))
      .catch(() => this.metrics.events?.increment({ error: true }))
    const connection = await dialPromise

    let maConn: QuicConnection

    try {
      maConn = new QuicConnection({
        connection,
        log: this.components.logger.forComponent(`libp2p:quic:connection:${connection.id()}:outbound`),
        direction: 'outbound',
        metrics: this.metrics?.events
      })
    } catch (err) {
      this.metrics.errors?.increment({ outbound_to_connection: true })
      throw err
    }

    try {
      this.log('new outbound connection %a', maConn.remoteAddr)

      // Extract remote peer ID from the TLS-derived multiaddr
      const p2pComponent = maConn.remoteAddr.getComponents().find(c => c.name === 'p2p')
      if (p2pComponent?.value == null) {
        throw new Error(`Remote multiaddr does not contain a peer ID: ${maConn.remoteAddr.toString()}`)
      }
      const remotePeer = peerIdFromString(p2pComponent.value)

      // If the dialed multiaddr included a /p2p/ component, verify it matches
      // the TLS-derived peer ID to prevent connecting to an unexpected peer
      const expectedP2p = ma.getComponents().find(c => c.name === 'p2p')
      if (expectedP2p?.value != null) {
        const expectedPeer = peerIdFromString(expectedP2p.value)
        if (!remotePeer.equals(expectedPeer)) {
          throw new Error(`Dialed peer ${expectedPeer.toString()} but connected to ${remotePeer.toString()}`)
        }
      }

      return await options.upgrader.upgradeOutbound(maConn, {
        skipEncryption: true,
        skipProtection: true,
        remotePeer,
        muxerFactory: new QuicStreamMuxerFactory({
          connection,
          logger: this.components.logger
        }),
        signal: options.signal
      })
    } catch (err: any) {
      this.metrics.errors?.increment({ outbound_upgrade: true })
      this.log.error('error upgrading outbound connection - %e', err)
      maConn.abort(err)
      throw err
    }
  }

  createListener (options: QuicCreateListenerOptions): Listener {
    return new QuicListener({
      options,
      config: this.#config,
      logger: this.components.logger,
      metrics: this.components.metrics
    })
  }
}
