import { AbortError, transportSymbol } from '@libp2p/interface'
import { marshalPrivateKey } from '@libp2p/crypto/keys'
import type { ComponentLogger, Connection, DialTransportOptions, Listener, Logger, Metrics, MultiaddrFilter, PrivateKey, Transport } from '@libp2p/interface'
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

export class QuicTransport implements Transport {
  readonly [Symbol.toStringTag]: string = "quic"
  readonly [transportSymbol] = true

  readonly log: Logger
  readonly components: QuicComponents

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
    const connection = await dialer?.outboundConnection(addr.address, addr.port)
    const maConn = new QuicConnection({
      connection,
      logger: this.components.logger,
      direction: 'outbound',
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
    })
  }
}
