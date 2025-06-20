import { AbstractStream, type AbstractStreamInit } from '@libp2p/utils/abstract-stream'
import { Uint8ArrayList } from 'uint8arraylist'
import type * as napi from './napi.js'
import type { AbortOptions } from '@libp2p/interface'

export interface QuicStreamInit extends AbstractStreamInit {
  stream: napi.Stream
}

export class QuicStream extends AbstractStream {
  readonly #stream: napi.Stream

  constructor (init: QuicStreamInit) {
    super(init)
    this.#stream = init.stream
    this.direction = init.direction

    this.log('new', this.direction, this.id)

    this.readyFromStream()
      .catch(err => {
        this.log.error('error reading from stream - %e', err)
      })
  }

  sendNewStream (options?: AbortOptions): void | Promise<void> {

  }

  async sendData (buf: Uint8ArrayList, options?: AbortOptions): Promise<void> {
    this.log.trace('writing %d bytes', buf.byteLength)
    await this.#stream.write(buf.subarray())
    this.log.trace('wrote %d bytes', buf.byteLength)
  }

  async sendReset (options?: AbortOptions): Promise<void> {
    await this.#stream.resetWrite()
    await this.#stream.stopRead()
  }

  async sendCloseWrite (options?: AbortOptions): Promise<void> {
    await this.#stream.finishWrite()
  }

  async sendCloseRead (options?: AbortOptions): Promise<void> {
    // TODO: how to do this?
  }

  async readyFromStream (): Promise<void> {
    try {
      while (true) {
        this.log.trace('reading')
        const chunk = await this.#stream.read2()

        if (chunk == null) {
          break
        }

        this.sourcePush(new Uint8ArrayList(chunk))

        this.log.trace('read %d bytes', length)
      }
    } catch (err: any) {
      this.log.error('source error - %e', err)

      if (err.code === 'Unknown') {
        // clean exit
        this.remoteCloseRead()
        return
      }

      this.abort(err)
    } finally {
      this.remoteCloseWrite()
    }
  }
}

export const CHUNK_SIZE = 4096
