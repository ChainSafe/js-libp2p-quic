import { AbstractMultiaddrConnection } from '@libp2p/utils'
import { multiaddr } from '@multiformats/multiaddr'
import type * as napi from './napi.js'
import type { AbortOptions, CounterGroup, Logger, MessageStreamDirection } from '@libp2p/interface'
import type { SendResult } from '@libp2p/utils'
import type { Uint8ArrayList } from 'uint8arraylist'

interface QuicConnectionInit {
  connection: napi.Connection
  log: Logger
  direction: MessageStreamDirection
  metrics?: CounterGroup
  metricsPrefix?: string
}

export class QuicConnection extends AbstractMultiaddrConnection {
  readonly #connection: napi.Connection

  constructor (init: QuicConnectionInit) {
    const remoteAddr = multiaddr(init.connection.remoteMultiaddr())

    super({
      remoteAddr,
      direction: init.direction,
      log: init.log,
      metrics: init.metrics,
      metricPrefix: init.metricsPrefix
    })

    this.#connection = init.connection

    // close maconn when connection is closed by remote
    this.#connection.closed().then(() => {
      this.onTransportClosed()
    }, (err) => {
      this.onTransportClosed(err)
    })
  }

  sendData (_data: Uint8ArrayList): SendResult {
    // QUIC maConn doesn't send byte-level data directly — streams handle that.
    // Throw to fail fast if anything accidentally writes to the connection.
    throw new Error('QuicConnection does not support direct data writes — use streams instead')
  }

  sendReset (_err: Error): void {
    this.#connection.abort()
  }

  sendPause (): void {
    // QUIC handles flow control at transport level
  }

  sendResume (): void {
    // QUIC handles flow control at transport level
  }

  async sendClose (_options?: AbortOptions): Promise<void> {
    this.#connection.abort()
  }
}
