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

    // Start the async native write and signal backpressure so the base class
    // waits for a drain event before sending more data — this serializes
    // writes through the base class's own send queue.
    this.#stream.write(buf).then(
      () => {
        this.log.trace('wrote %d bytes', buf.byteLength)
        this.safeDispatchEvent('drain')
      },
      (err) => {
        this.log.error('write error - %e', err)
        this.abort(err)
      }
    )

    return { sentBytes: data.byteLength, canSendMore: false }
  }

  sendReset (_err: Error): void {
    void this.#stream.resetWrite()
    void this.#stream.stopRead()
  }

  async sendCloseWrite (_options?: AbortOptions): Promise<void> {
    // By the time the base class calls this, it has already waited for
    // the write buffer to drain (idle + drain), so all writes are complete.
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
      if (err.code === 'Unknown') {
        // Expected: native stream closed (connection close, stream reset, etc.)
        this.log.trace('read ended - %e', err)
        return
      }

      this.log.error('source error - %e', err)
      this.abort(err)
    } finally {
      this.onRemoteCloseWrite()
    }
  }
}

export const CHUNK_SIZE = 4096
