import { AbstractStream } from '@libp2p/utils'
import { Uint8ArrayList } from 'uint8arraylist'
import type * as napi from './napi.js'
import type { AbortOptions } from '@libp2p/interface'
import type { AbstractStreamInit, SendResult } from '@libp2p/utils'

export interface QuicStreamInit extends AbstractStreamInit {
  stream: napi.Stream
}

export class QuicStream extends AbstractStream {
  readonly #stream: napi.Stream
  #pendingWrite: Promise<void> = Promise.resolve()
  #readClosed = false

  constructor (init: QuicStreamInit) {
    super(init)
    this.#stream = init.stream

    this.log('new', this.direction, this.id)

    this.readFromStream()
      .catch(err => {
        this.log.error('error reading from stream - %e', err)
      })
  }

  sendData (data: Uint8ArrayList): SendResult {
    this.log.trace('writing %d bytes', data.byteLength)
    const buf = data.subarray()
    // Chain writes to ensure ordering — each write completes before the next starts
    this.#pendingWrite = this.#pendingWrite.then(
      () => this.#stream.write(buf)
    ).then(
      () => { this.log.trace('wrote %d bytes', buf.byteLength) },
      (err) => { this.log.error('write error - %e', err) }
    )
    return { sentBytes: data.byteLength, canSendMore: true }
  }

  sendReset (_err: Error): void {
    void this.#stream.resetWrite()
    void this.#stream.stopRead()
  }

  async sendCloseWrite (_options?: AbortOptions): Promise<void> {
    // Wait for all pending writes to complete before sending FIN
    await this.#pendingWrite
    await this.#stream.finishWrite()
  }

  async sendCloseRead (_options?: AbortOptions): Promise<void> {
    // Don't call native stopRead() here — the read loop must stay active
    // to detect when the remote closes their write end (FIN).
    // QUIC flow control handles backpressure at the transport level.
    this.#readClosed = true
  }

  sendPause (): void {
    // QUIC handles flow control at the transport level
  }

  sendResume (): void {
    // QUIC handles flow control at the transport level
  }

  async readFromStream (): Promise<void> {
    try {
      while (true) {
        this.log.trace('reading')
        const chunk = await this.#stream.read(CHUNK_SIZE)

        if (chunk == null) {
          break
        }

        if (!this.#readClosed) {
          this.onData(new Uint8ArrayList(chunk))
        }

        this.log.trace('read %d bytes', chunk.length)
      }
    } catch (err: any) {
      this.log.error('source error - %e', err)

      if (err.code === 'Unknown') {
        // Stream error from connection close
        return
      }

      this.abort(err)
    } finally {
      this.onRemoteCloseWrite()
    }
  }
}

export const CHUNK_SIZE = 4096
