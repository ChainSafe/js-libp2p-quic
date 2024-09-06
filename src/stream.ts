import { Uint8ArrayList } from 'uint8arraylist'
import type { AbortOptions, ComponentLogger, Direction, Logger, ReadStatus, Stream, StreamStatus, StreamTimeline, WriteStatus } from '@libp2p/interface'
import type { Sink, Source } from 'it-stream-types'

import * as napi from './napi.js'

type QuicStreamInit = {
  connId: string
  stream: napi.Stream
  direction: Direction
  logger: ComponentLogger
}

export class QuicStream implements Stream {
  readonly #stream: napi.Stream

  readonly id: string
  readonly direction: Direction
  readonly log: Logger


  protocol?: string | undefined
  timeline: StreamTimeline = {
    open: Date.now(),
  }
  metadata: Record<string, any> = {}
  status: StreamStatus = 'open'
  readStatus: ReadStatus = 'ready'
  writeStatus: WriteStatus = 'ready'
  source: AsyncGenerator<Uint8ArrayList> = this._source()

  constructor(init: QuicStreamInit) {
    this.#stream = init.stream
    this.id = `(${init.connId} ${this.#stream.id()})`
    this.direction = init.direction
    this.log = init.logger.forComponent('libp2p:quic:stream')

    this.log('new', this.direction, this.id)
  }

  async close(options?: AbortOptions): Promise<void> {
    if (this.status !== 'open') {
      return
    }

    this.status = 'closing'
    this.readStatus = 'closing'
    this.writeStatus = 'closing'
    await Promise.all([
      this.closeRead(options),
      this.closeWrite(options),
    ])
    this.status = 'closed'
    this.readStatus = 'closed'
    this.writeStatus = 'closed'

    this.log('%s closed', this.id)
  }
  async closeRead(options?: AbortOptions): Promise<void> {
    if (this.readStatus !== 'ready') {
      return
    }

    this.#stream.stopRead()
    this.readStatus = 'closed'
    if (this.writeStatus === 'closed') {
      this.status = 'closed'
    }
    this.log('%s close read', this.id)
  }
  async closeWrite(options?: AbortOptions): Promise<void> {
    if (this.writeStatus !== 'ready') {
      return
    }

    this.#stream.finishWrite()
    this.writeStatus = 'closed'
    if (this.readStatus === 'closed') {
      this.status = 'closed'
    }
    this.log('%s close write', this.id)
  }
  abort(err: Error): void {
    if (this.status === 'closed') {
      return
    }

    this.#stream.resetWrite()
    this.#stream.stopRead()
    this.status = 'aborted'
    this.readStatus = 'closed'
    this.writeStatus = 'closed'

    this.log('%s aborted', this.id)
  }
  async * _source (): AsyncGenerator<Uint8ArrayList> {
    try {
      while (true) {
        this.log.trace('', this.id, 'reading')
        // const chunk = Buffer.allocUnsafe(CHUNK_SIZE)
        // const length = await this.#stream.read(chunk)
        // if (length == null) {
        //   this.log.trace('', this.id, 'no more data')
        //   break
        // }
        // yield new Uint8ArrayList(chunk.subarray(0, length))
        const chunk = await this.#stream.read2()
        if (chunk == null) {
          this.log.trace('', this.id, 'no more data')
          break
        }
        yield new Uint8ArrayList(chunk)
        this.log.trace('', this.id, 'read', length, 'bytes')
      }
    } catch (e) {
      this.log.error('source error', this.id, e)
    } finally {
      await this.closeRead()
    }
  }
  sink: Sink<Source<Uint8Array | Uint8ArrayList>, Promise<void>> = async (source) => {
    try {
      for await (const chunk of source) {
        this.log.trace('', this.id, 'writing', chunk.length, 'bytes')
        if (chunk instanceof Uint8ArrayList) {
          await this.#stream.write2(chunk.subarray())
        } else {
          await this.#stream.write2(chunk)
        }
        this.log.trace('', this.id, 'wrote', chunk.length, 'bytes')
      }
    } catch (e) {
      this.log.error('sink error', this.id, e)
    } finally {
      await this.closeWrite()
    }
  }
}

export const CHUNK_SIZE = 4096

