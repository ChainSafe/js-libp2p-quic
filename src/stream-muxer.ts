import { Uint8ArrayList } from 'uint8arraylist'

import type { AbortOptions, ComponentLogger, Logger, Stream, StreamMuxer, StreamMuxerFactory, StreamMuxerInit } from '@libp2p/interface'
import type { Sink } from 'it-stream-types'

import * as napi from './napi.js'
import { QuicStream } from './stream.js'

type QuicStreamMuxerFactoryInit = {
  connection: napi.Connection
  logger: ComponentLogger
}

/**
 * Each stream muxer factory is only configured for a single connection
 */
export class QuicStreamMuxerFactory implements StreamMuxerFactory {
  #connection: napi.Connection
  init: QuicStreamMuxerFactoryInit
  protocol: string = 'quic'

  constructor(init: QuicStreamMuxerFactoryInit) {
    this.#connection = init.connection
    this.init = init
  }

  createStreamMuxer(init?: StreamMuxerInit): StreamMuxer {
    return new QuicStreamMuxer({
      ...init,
      connection: this.#connection,
      logger: this.init.logger,
    })
  }
}

type QuicStreamMuxerInit = StreamMuxerInit & {
  connection: napi.Connection
  logger: ComponentLogger
}

class QuicStreamMuxer implements StreamMuxer {
  id: string
  #connection: napi.Connection
  init: QuicStreamMuxerInit
  log: Logger

  protocol: string = 'quic'
  streams: Stream[] = []
  source: AsyncGenerator<Uint8Array | Uint8ArrayList> = (async function* () {})()
  sink: Sink<AsyncGenerator<Uint8Array | Uint8ArrayList>> = async function* () {}
  controller = new AbortController()

  constructor(init: QuicStreamMuxerInit) {
    this.id = init.connection.id()
    this.#connection = init.connection
    this.init = init
    this.log = init.logger.forComponent('libp2p:quic:muxer')

    void this.awaitInboundStreams()
    this.log('new', this.id)
  }

  async awaitInboundStreams(): Promise<void> {
    const aborted = new Promise((resolve) => {
      this.controller.signal.addEventListener('abort', () => resolve(undefined), { once: true })
    })
    while (true) {
      const stream = await Promise.race([
        aborted,
        this.#connection.inboundStream()
      ]) as napi.Stream | undefined
      if (stream == null) {
        break
      }

      try {
        this.onInboundStream(stream)
      } catch (e) {
        this.log.error('%s error accepting stream', this.id, e)
      }
    }
    this.log('%s no longer awaiting inbound streams', this.id)
  }

  private onInboundStream = (str: napi.Stream) => {
    const stream = new QuicStream({
      connId: this.#connection.id(),
      stream: str,
      direction: 'inbound',
      logger: this.init.logger,
    })
    this.streams.push(stream)
    this.init.onIncomingStream?.(stream)
  }

  async newStream(name?: string): Promise<Stream> {
    const stream = new QuicStream({
      connId: this.#connection.id(),
      stream:  await this.#connection.outboundStream(),
      direction: 'outbound',
      logger: this.init.logger,
    })
    this.streams.push(stream)
    return stream
  }
  async close(options?: AbortOptions): Promise<void> {
    this.controller.abort()
    await Promise.all(this.streams.map((stream) => stream.close(options)))
    this.streams = []

    this.log('%s closed', this.id)
  }
  abort(err: Error): void {
    this.controller.abort()
    for (const stream of this.streams) {
      stream.abort(err)
    }
    this.streams = []

    this.log('%s aborted', this.id)
  }
}
