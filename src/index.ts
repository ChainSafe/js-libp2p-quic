import { AbortError, transportSymbol, TypedEventEmitter, type AbortOptions, type ComponentLogger, type Connection, type ConnectionStatus, type ConnectionTimeline, type CreateListenerOptions, type DialTransportOptions, type Direction, type Listener, type ListenerEvents, type Logger, type Metrics, type MultiaddrConnection, type MultiaddrConnectionTimeline, type MultiaddrFilter, type NewStreamOptions, type PeerId, type PrivateKey, type ReadStatus, type Stream, type StreamMuxer, type StreamMuxerFactory, type StreamMuxerInit, type StreamStatus, type StreamTimeline, type Transport, type WriteStatus } from '@libp2p/interface'
import { multiaddr, type Multiaddr } from '@multiformats/multiaddr'
import { marshalPrivateKey } from '@libp2p/crypto/keys'
import type { Sink, Source } from 'it-stream-types'
import { Uint8ArrayList } from 'uint8arraylist'
import { dialFilter, listenFilter } from './filter.js'

import * as napi from './napi.js'

export function quic(options?: Partial<QuicOptions>): (components: QuicComponents) => Transport {
  return (components) => new QuicTransport(components, {...defaultOptions, ...options})
}

export const defaultOptions: QuicOptions = {
  handshakeTimeout: 5_000,
  maxIdleTimeout: 10_000,
  keepAliveInterval: 5_000,
  maxConcurrentStreamLimit: 256,
  maxStreamData: 10_000_000,
  maxConnectionData: 15_000_000, 
}

export type QuicOptions = Omit<napi.Config, "privateKeyProto"> & {}
export type QuicComponents = {
  metrics?: Metrics
  logger: ComponentLogger
  privateKey: PrivateKey
}

export type QuicDialOptions = DialTransportOptions & {}
export type QuicCreateListenerOptions = CreateListenerOptions & {}

export class QuicTransport implements Transport {
  readonly [Symbol.toStringTag]: string = "quic"
  readonly [transportSymbol] = true

  readonly log: Logger
  readonly components: QuicComponents

  readonly #config: napi.QuinnConfig

  readonly #clients: {
    ip4: napi.Client
    ip6: napi.Client
  }

  readonly listenFilter: MultiaddrFilter
  readonly dialFilter: MultiaddrFilter

  constructor(components: QuicComponents, options: QuicOptions) {
    const privateKeyProto = marshalPrivateKey(components.privateKey);
    const config = { ...options, privateKeyProto };

    this.log = components.logger.forComponent('libp2p:quic:transport')
    this.components = components

    this.#config = new napi.QuinnConfig(config);
    this.#clients = {
      ip4: new napi.Client(this.#config, 0),
      ip6: new napi.Client(this.#config, 1),
    }

    this.listenFilter = listenFilter
    this.dialFilter = dialFilter

    this.log('new')
  }

  async dial(ma: Multiaddr, options: QuicDialOptions): Promise<Connection> {
    if (options.signal?.aborted) {
      throw new AbortError()
    }

    this.log('dialing', ma.toString())
    const addr = ma.nodeAddress()
    const dialer = addr.family === 4 ? this.#clients.ip4 : this.#clients.ip6
    const connection = await dialer?.outboundConnection(addr.address, addr.port)
    const maConn = new QuicConnection({
      connection,
      logger: this.components.logger,
      direction: 'outbound',
    })
    return options.upgrader.upgradeOutbound(maConn, {
      skipEncryption: true,
      skipProtection: true,
      muxerFactory: new QuicStreamMuxerFactory({
        connection,
        logger: this.components.logger,
      }),
    })
  }

  createListener(options: QuicCreateListenerOptions): Listener {
    return new QuicListener({
      options,
      config: this.#config,
      logger: this.components.logger,
    })
  }
}

type QuicListenerInit = {
  options: QuicCreateListenerOptions
  config: napi.QuinnConfig
  logger: ComponentLogger
}

type QuicListenerState = {
  status: 'ready'
} | {
  status: 'listening'
  listener: napi.Server
  listenAddr: Multiaddr
  controller: AbortController
  connections: Set<Connection>
} | {
  status: 'closed'
}

export class QuicListener extends TypedEventEmitter<ListenerEvents> implements Listener {
  readonly #config: napi.QuinnConfig
  readonly init: QuicListenerInit
  readonly options: QuicCreateListenerOptions
  readonly log: Logger

  state: QuicListenerState = { status: 'ready' }

  constructor(init: QuicListenerInit) {
    super()
    this.#config = init.config
    this.init = init
    this.options = init.options
    this.log = init.logger.forComponent('libp2p:quic:listener')

    this.log('new')
  }
  getAddrs(): Multiaddr[] {
    if (this.state.status === 'listening') {
      return [this.state.listenAddr]
    }
    return []
  }

  async listen(multiaddr: Multiaddr): Promise<void> {
    const addr = multiaddr.nodeAddress()
    const controller = new AbortController()
    const listener = new napi.Server(this.#config, addr.address, addr.port)
    this.state = {
      status: 'listening',
      listener,
      listenAddr: multiaddr,
      controller,
      connections: new Set(),
    }
    void this.awaitInboundConnections()
    this.safeDispatchEvent('listening')
    this.log('listening', multiaddr.toString())
  }

  async close(): Promise<void> {
    if (this.state.status === 'listening') {
      this.state.controller.abort()
      for (const conn of this.state.connections) {
        conn.abort(new Error('listener closed'));
      }
      this.state.connections.clear()
      await this.state.listener.abort()
      const listenAddr = this.state.listenAddr
      this.state = { status: 'closed' }
      this.safeDispatchEvent('close')
      this.log('closed', listenAddr.toString())
    }
  }

  async awaitInboundConnections(): Promise<void> {
    if (this.state.status === 'listening') {
      const signal = this.state.controller.signal
      const listenAddr = this.state.listenAddr
      const aborted = new Promise((resolve) => {
        signal.addEventListener('abort', () => resolve(undefined), { once: true })
      })
      while (true) {
        try {
          const connection = await Promise.race([
            aborted,
            this.state.listener.inboundConnection(),
          ]) as napi.Connection | undefined
          if (connection == null) {
            break
          }

          this.onInboundConnection(connection).catch((e) => {
            this.log.error('%s error handling inbound connection', listenAddr.toString(), e)
          })
        } catch (e) {
          this.log.error('%s error accepting connection', listenAddr.toString(), e)
        }
      }
      this.log('%s no longer awaiting inbound connections', listenAddr.toString())
    }
  }

  async onInboundConnection(connection: napi.Connection): Promise<void> {
    if (this.state.status !== 'listening') {
      this.log.error('ignoring inbound connection after listener closed')
      connection.abort()
      return
    }

    const maConn = new QuicConnection({
      connection,
      logger: this.init.logger,
      direction: 'inbound',
    })

    try {
      const conn = await this.options.upgrader.upgradeInbound(maConn, {
        skipEncryption: true,
        skipProtection: true,
        muxerFactory: new QuicStreamMuxerFactory({
          connection,
          logger: this.init.logger,
        }),
      })

      this.state.connections.add(conn)
      maConn.addEventListener('close', () => {
        if (this.state.status === 'listening') {
          this.state.connections.delete(conn)
        }
      }, { once: true })

      this.safeDispatchEvent('connection', { detail: conn })
      this.options.handler?.(conn)
    } catch (err) {
      this.log.error('%s error handling inbound connection', this.state.listenAddr.toString(), err)
      maConn.abort(err as Error)
    }
  }
}

type QuicConnectionInit = {
  connection: napi.Connection
  logger: ComponentLogger
  direction: Direction
}

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

type QuicConnectionEvents = {
  'close': CustomEvent
}

export class QuicConnection extends TypedEventEmitter<QuicConnectionEvents> implements MultiaddrConnection {
  readonly #connection: napi.Connection

  readonly log: Logger
  readonly remoteAddr: Multiaddr

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
    this.safeDispatchEvent('close')
  }
  abort(err: Error): void {
    this.#connection.abort()

    this.timeline.close = Date.now()
    this.log('%s aborted', this.#connection.id())
    this.safeDispatchEvent('close')
  }
}

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
        const chunk = Buffer.allocUnsafe(CHUNK_SIZE)
        const length = await this.#stream.read(chunk)
        if (length == null) {
          this.log.trace('', this.id, 'no more data')
          break
        }
        yield new Uint8ArrayList(chunk.subarray(0, length))
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
          await this.#stream.write(chunk.subarray())
        } else {
          await this.#stream.write(chunk)
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

