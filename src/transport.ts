import { AbortError, transportSymbol } from '@libp2p/interface'
import { marshalPrivateKey } from '@libp2p/crypto/keys'
import type { ComponentLogger, Connection, CounterGroup, DialTransportOptions, Listener, Logger, Metrics, MultiaddrFilter, PrivateKey, Transport } from '@libp2p/interface'
import type { Multiaddr } from '@multiformats/multiaddr'

import * as napi from './napi.js'
import { dialFilter, listenFilter } from './filter.js'
import { QuicListener, type QuicCreateListenerOptions } from './listener.js'
import { QuicConnection } from './connection.js'
import { QuicStreamMuxerFactory } from './stream-muxer.js'

export type QuicOptions = Omit<napi.Config, "privateKeyProto"> & {}

export type QuicComponents = {
  metrics?: Metrics
  logger: ComponentLogger
  privateKey: PrivateKey
}

export type QuicDialOptions = DialTransportOptions & {}

export type QuicTransportMetrics = {
  events: CounterGroup
}

export class QuicTransport implements Transport {
  readonly [Symbol.toStringTag]: string = "quic"
  readonly [transportSymbol] = true

  readonly log: Logger
  readonly components: QuicComponents
  readonly metrics?: QuicTransportMetrics

  readonly #config: napi.QuinnConfig

  readonly #clients: {
    ip4: napi.Client
    ip6: napi.Client
  }

  readonly listenFilter: MultiaddrFilter
  readonly dialFilter: MultiaddrFilter

  constructor(components: QuicComponents, options: QuicOptions) {
    const privateKeyProto = marshalPrivateKey(components.privateKey);
    const config = { ...options, privateKeyProto };

    this.log = components.logger.forComponent('libp2p:quic:transport')
    this.components = components

    this.#config = new napi.QuinnConfig(config);
    this.#clients = {
      ip4: new napi.Client(this.#config, 0),
      ip6: new napi.Client(this.#config, 1),
    }

    if (this.components.metrics != null) {
      this.metrics = {
        events: this.components.metrics?.registerCounterGroup('libp2p_quic_dialer_events_total', {
          label: 'event',
          help: 'Total count of QUIC dialer events by type',
        }),
      }
    }

    this.listenFilter = listenFilter
    this.dialFilter = dialFilter

    this.log('new')
  }

  async dial(ma: Multiaddr, options: QuicDialOptions): Promise<Connection> {
    if (options.signal?.aborted) {
      throw new AbortError()
    }

    this.log('dialing', ma.toString())
    const addr = ma.nodeAddress()
    const dialer = addr.family === 4 ? this.#clients.ip4 : this.#clients.ip6

    const dialPromise = dialer.outboundConnection(addr.address, addr.port)
    dialPromise
      .then(() => this.metrics?.events.increment({ connect: true }))
      .catch(() => this.metrics?.events.increment({ error: true }))
    const connection = await dialPromise

    const maConn = new QuicConnection({
      connection,
      logger: this.components.logger,
      direction: 'outbound',
      metrics: this.metrics?.events,
    })
    return options.upgrader.upgradeOutbound(maConn, {
      skipEncryption: true,
      skipProtection: true,
      muxerFactory: new QuicStreamMuxerFactory({
        connection,
        logger: this.components.logger,
      }),
    })
  }

  createListener(options: QuicCreateListenerOptions): Listener {
    return new QuicListener({
      options,
      config: this.#config,
      logger: this.components.logger,
      metrics: this.components.metrics,
    })
  }
}
