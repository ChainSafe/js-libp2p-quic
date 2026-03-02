import { TypedEventEmitter } from '@libp2p/interface'
import { raceSignal } from 'race-signal'
import { QuicStream } from './stream.js'
import type * as napi from './napi.js'
import type { AbortOptions, ComponentLogger, CreateStreamOptions, Logger, MessageStream, Stream, StreamMuxer, StreamMuxerEvents, StreamMuxerFactory, StreamMuxerStatus } from '@libp2p/interface'

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

  createStreamMuxer (maConn: MessageStream): StreamMuxer {
    return new QuicStreamMuxer({
      connection: this.#connection,
      logger: this.init.logger,
      maConn
    })
  }
}

interface QuicStreamMuxerInit {
  connection: napi.Connection
  logger: ComponentLogger
  maConn: MessageStream
}

class QuicStreamMuxer extends TypedEventEmitter<StreamMuxerEvents> implements StreamMuxer {
  id: string
  readonly #connection: napi.Connection
  init: QuicStreamMuxerInit
  log: Logger

  protocol: string = 'quic'
  streams: Stream[] = []
  status: StreamMuxerStatus = 'open'
  controller = new AbortController()

  constructor (init: QuicStreamMuxerInit) {
    super()
    this.id = init.connection.id()
    this.#connection = init.connection
    this.init = init
    this.log = init.logger.forComponent('libp2p:quic:muxer')

    void this.awaitInboundStreams()

    // Abort all streams when the underlying connection closes.
    // This listener is registered before the libp2p Connection's listener,
    // ensuring streams are cleaned up before the Connection's 'close' event propagates.
    init.maConn.addEventListener('close', () => {
      this.abort(new Error('connection closed'))
    }, { once: true })

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
      log: this.init.logger.forComponent(`libp2p:quic:stream:${this.#connection.id()}:${str.id()}:inbound`)
    })
    this.streams.push(stream)
    this.cleanUpStream(stream)
    this.safeDispatchEvent('stream', { detail: stream })
  }

  async createStream (_options?: CreateStreamOptions): Promise<Stream> {
    const str = await this.#connection.outboundStream()
    this.controller.signal.throwIfAborted()
    const stream = new QuicStream({
      id: str.id(),
      stream: str,
      direction: 'outbound',
      log: this.init.logger.forComponent(`libp2p:quic:stream:${this.#connection.id()}:${str.id()}:outbound`)
    })
    this.streams.push(stream)
    this.cleanUpStream(stream)
    return stream
  }

  private cleanUpStream (stream: Stream): void {
    stream.addEventListener('close', () => {
      const index = this.streams.findIndex(s => s === stream)
      if (index !== -1) {
        this.streams.splice(index, 1)
      }
    }, { once: true })
  }

  async close (options?: AbortOptions): Promise<void> {
    if (this.status === 'closed' || this.status === 'closing') {
      return
    }
    this.status = 'closing'
    this.controller.abort()
    try {
      // Gracefully close write side of all streams
      await Promise.all(this.streams.map(async (stream) => stream.close(options)))
    } catch {
      // Timeout or other error — fall through to force-abort
    }
    // Force-abort any streams still open (read side hasn't closed yet)
    for (const stream of [...this.streams]) {
      stream.abort(new Error('muxer closed'))
    }
    this.streams = []
    this.status = 'closed'

    this.log('%s closed', this.id)
  }

  abort (err: Error): void {
    if (this.status === 'closed') {
      return
    }
    this.status = 'closing'
    this.controller.abort()
    for (const stream of this.streams) {
      stream.abort(err)
    }
    this.streams = []
    this.status = 'closed'

    this.log('%s aborted', this.id)
  }
}
