import { TypedEventEmitter } from '@libp2p/interface'
import { multiaddr } from '@multiformats/multiaddr'
import type * as napi from './napi.js'
import type { AbortOptions, ComponentLogger, CounterGroup, Direction, Logger, MultiaddrConnection, MultiaddrConnectionTimeline } from '@libp2p/interface'
import type { Multiaddr } from '@multiformats/multiaddr'
import type { Sink } from 'it-stream-types'
import type { Uint8ArrayList } from 'uint8arraylist'

interface QuicConnectionInit {
  connection: napi.Connection
  logger: ComponentLogger
  direction: Direction
  metrics?: CounterGroup
  metricsPrefix?: string
}

interface QuicConnectionEvents {
  close: CustomEvent
}

export class QuicConnection extends TypedEventEmitter<QuicConnectionEvents> implements MultiaddrConnection {
  readonly #connection: napi.Connection

  readonly log: Logger
  readonly remoteAddr: Multiaddr
  readonly metrics?: CounterGroup
  readonly metricsPrefix: string

  private remoteClosed?: boolean

  timeline: MultiaddrConnectionTimeline = {
    open: Date.now()
  }

  source: AsyncGenerator<Uint8Array | Uint8ArrayList> = (async function * () {})()
  sink: Sink<AsyncGenerator<Uint8Array | Uint8ArrayList>> = async function * () {}

  constructor (init: QuicConnectionInit) {
    super()

    this.#connection = init.connection
    this.log = init.logger.forComponent(`libp2p:quic:connection:${this.#connection.id()}:${init.direction}`)
    this.remoteAddr = multiaddr(this.#connection.remoteMultiaddr())
    this.metrics = init.metrics
    this.metricsPrefix = init.metricsPrefix ?? ''

    // close maconn when connection is closed by remote
    this.#connection.closed().then(() => {
      this.remoteClosed = true
      this.metrics?.increment({ [`${this.metricsPrefix}end`]: true })
      this.close()
        .catch(err => {
          this.abort(err)
        })
    }, (err) => {
      this.abort(err)
    })
  }

  async close (options?: AbortOptions): Promise<void> {
    if (this.timeline.close != null) {
      return
    }

    this.#connection.abort()

    this.timeline.close = Date.now()
    this.log('closed')

    if (this.remoteClosed !== true) {
      this.metrics?.increment({ [`${this.metricsPrefix}close`]: true })
    }

    this.safeDispatchEvent('close')
  }

  abort (err: Error): void {
    if (this.timeline.close != null) {
      return
    }

    this.#connection.abort()

    this.timeline.close = Date.now()
    this.log('aborted - %e', err)
    this.metrics?.increment({ [`${this.metricsPrefix}abort`]: true })
    this.safeDispatchEvent('close')
  }
}
