import { raceSignal } from 'race-signal'
import { QuicStream } from './stream.js'
import type * as napi from './napi.js'
import type { AbortOptions, ComponentLogger, Logger, Stream, StreamMuxer, StreamMuxerFactory, StreamMuxerInit } from '@libp2p/interface'
import type { Sink } from 'it-stream-types'
import type { Uint8ArrayList } from 'uint8arraylist'

interface QuicStreamMuxerFactoryInit {
  connection: napi.Connection
  logger: ComponentLogger
}

/**
 * Each stream muxer factory is only configured for a single connection
 */
export class QuicStreamMuxerFactory implements StreamMuxerFactory {
  readonly #connection: napi.Connection
  init: QuicStreamMuxerFactoryInit
  protocol: string = 'quic'

  constructor (init: QuicStreamMuxerFactoryInit) {
    this.#connection = init.connection
    this.init = init
  }

  createStreamMuxer (init?: StreamMuxerInit): StreamMuxer {
    return new QuicStreamMuxer({
      ...init,
      connection: this.#connection,
      logger: this.init.logger
    })
  }
}

type QuicStreamMuxerInit = StreamMuxerInit & {
  connection: napi.Connection
  logger: ComponentLogger
}

class QuicStreamMuxer implements StreamMuxer {
  id: string
  readonly #connection: napi.Connection
  init: QuicStreamMuxerInit
  log: Logger

  protocol: string = 'quic'
  streams: Stream[] = []
  source: AsyncGenerator<Uint8Array | Uint8ArrayList> = (async function * () {})()
  sink: Sink<AsyncGenerator<Uint8Array | Uint8ArrayList>> = async function * () {}
  controller = new AbortController()

  constructor (init: QuicStreamMuxerInit) {
    this.id = init.connection.id()
    this.#connection = init.connection
    this.init = init
    this.log = init.logger.forComponent('libp2p:quic:muxer')

    void this.awaitInboundStreams()
    this.log('new', this.id)
  }

  async awaitInboundStreams (): Promise<void> {
    while (true) {
      try {
        const stream = await raceSignal(this.#connection.inboundStream(), this.controller.signal)

        this.onInboundStream(stream)
      } catch (e) {
        this.log.error('%s error accepting stream - %e', this.id, e)

        if (this.controller.signal.aborted) {
          break
        }
      }
    }

    this.log('%s no longer awaiting inbound streams', this.id)
  }

  private readonly onInboundStream = (str: napi.Stream): void => {
    const stream = new QuicStream({
      id: str.id(),
      stream: str,
      direction: 'inbound',
      log: this.init.logger.forComponent(`libp2p:quic:stream:${this.#connection.id()}:${str.id()}:inbound`),
      onEnd: () => {
        const index = this.streams.findIndex(s => s === stream)
        if (index !== -1) {
          this.streams.splice(index, 1)
        }

        this.init.onStreamEnd?.(stream)
      }
    })
    this.streams.push(stream)
    this.init.onIncomingStream?.(stream)
  }

  async newStream (name?: string): Promise<Stream> {
    const str = await this.#connection.outboundStream()
    this.controller.signal.throwIfAborted()
    const stream = new QuicStream({
      id: str.id(),
      stream: str,
      direction: 'outbound',
      log: this.init.logger.forComponent(`libp2p:quic:stream:${this.#connection.id()}:${str.id()}:outbound`),
      onEnd: () => {
        const index = this.streams.findIndex(s => s === stream)
        if (index !== -1) {
          this.streams.splice(index, 1)
        }

        this.init.onStreamEnd?.(stream)
      }
    })
    this.streams.push(stream)
    return stream
  }

  async close (options?: AbortOptions): Promise<void> {
    this.controller.abort()
    await Promise.all(this.streams.map(async (stream) => stream.close(options)))
    this.streams = []

    this.log('%s closed', this.id)
  }

  abort (err: Error): void {
    this.controller.abort()
    for (const stream of this.streams) {
      stream.abort(err)
    }
    this.streams = []

    this.log('%s aborted', this.id)
  }
}
