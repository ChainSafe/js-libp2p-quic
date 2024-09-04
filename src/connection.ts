import { TypedEventEmitter } from '@libp2p/interface'
import { multiaddr, type Multiaddr } from '@multiformats/multiaddr'
import { Uint8ArrayList } from 'uint8arraylist'

import type { AbortOptions, ComponentLogger, CounterGroup, Direction, Logger, MultiaddrConnection, MultiaddrConnectionTimeline } from '@libp2p/interface'
import type { Sink } from 'it-stream-types'

import * as napi from './napi.js'

type QuicConnectionInit = {
  connection: napi.Connection
  logger: ComponentLogger
  direction: Direction
  metrics?: CounterGroup
}

type QuicConnectionEvents = {
  'close': CustomEvent
}

export class QuicConnection extends TypedEventEmitter<QuicConnectionEvents> implements MultiaddrConnection {
  readonly #connection: napi.Connection

  readonly log: Logger
  readonly remoteAddr: Multiaddr
  readonly metrics?: CounterGroup

  timeline: MultiaddrConnectionTimeline = {
    open: Date.now(),
  }
  source: AsyncGenerator<Uint8Array | Uint8ArrayList> = (async function* () {})()
  sink: Sink<AsyncGenerator<Uint8Array | Uint8ArrayList>> = async function* () {}

  constructor(init: QuicConnectionInit) {
    super()

    this.#connection = init.connection
    this.log = init.logger.forComponent('libp2p:quic:connection')
    this.remoteAddr = multiaddr(this.#connection.remoteMultiaddr())

    this.log('new', init.direction, this.#connection.id())
  }
  async close(options?: AbortOptions): Promise<void> {
    this.#connection.abort()

    this.timeline.close = Date.now()
    this.log('%s closed', this.#connection.id())
    this.metrics?.increment({close: true})
    this.safeDispatchEvent('close')
  }
  abort(err: Error): void {
    this.#connection.abort()

    this.timeline.close = Date.now()
    this.log('%s aborted', this.#connection.id())
    this.metrics?.increment({abort: true})
    this.safeDispatchEvent('close')
  }
}
