# libp2p v3 Migration Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Migrate `@chainsafe/libp2p-quic` from libp2p v2 interfaces to libp2p v3, adopting the new EventTarget-based MessageStream model.

**Architecture:** Replace async iterable duplex streams (source/sink) with the new synchronous send() / event-based MessageStream API. Use `AbstractStream`, `AbstractMultiaddrConnection` from `@libp2p/utils` v7 as base classes. The QUIC stream muxer implements `StreamMuxer` directly (not `AbstractStreamMuxer`) since QUIC handles multiplexing natively in Rust/quinn. A multiaddr parsing helper replaces removed `nodeAddress()`/`toOptions()` methods.

**Tech Stack:** TypeScript, `@libp2p/interface` v3, `@libp2p/utils` v7, `@multiformats/multiaddr` v13, NAPI-RS native bindings

---

### Task 1: Update dependencies in package.json

**Files:**
- Modify: `package.json`

**Step 1: Update package.json dependencies**

```json
{
  "dependencies": {
    "@libp2p/crypto": "^5.1.7",
    "@libp2p/interface": "^3.1.0",
    "@libp2p/peer-id": "^6.0.4",
    "@libp2p/utils": "^7.0.11",
    "@multiformats/multiaddr": "^13.0.1",
    "@multiformats/multiaddr-matcher": "^3.0.1",
    "race-signal": "^1.1.3",
    "uint8arraylist": "^2.4.8"
  },
  "devDependencies": {
    "@chainsafe/is-ip": "^2.1.0",
    "@libp2p/interface-compliance-tests": "^7.0.12",
    "@libp2p/logger": "^6.2.2",
    "@napi-rs/cli": "^3.5.0",
    "@types/mocha": "^10.0.10",
    "@types/node": "^24.10.1",
    "aegir": "^47.0.19",
    "p-event": "^6.0.1",
    "sinon-ts": "^2.0.0",
    "wherearewe": "^2.0.1"
  }
}
```

Key changes:
- `@libp2p/interface`: `^2.10.5` → `^3.1.0`
- `@libp2p/utils`: `^6.7.1` → `^7.0.11`
- `@multiformats/multiaddr`: `^12.4.0` → `^13.0.1`
- `@multiformats/multiaddr-matcher`: `^2.0.1` → `^3.0.1`
- `@libp2p/peer-id`: **NEW** `^6.0.4`
- `@libp2p/interface-compliance-tests`: `^6.4.15` → `^7.0.12`
- `@libp2p/logger`: `^5.1.21` → `^6.2.2`
- **REMOVE** `it-stream-types` dependency

**Step 2: Install dependencies**

Run: `pnpm install`
Expected: Dependencies resolve and install successfully.

**Step 3: Commit**

```
feat: update dependencies for libp2p v3 migration
```

---

### Task 2: Add multiaddr parsing helper to `src/utils.ts`

Since `nodeAddress()` and `toOptions()` are removed in `@multiformats/multiaddr` v13,
we need a helper to extract IP address, family, and port from a multiaddr.

**Files:**
- Modify: `src/utils.ts`

**Step 1: Add the `nodeAddressFromMultiaddr` helper**

Add this to `src/utils.ts` alongside existing helpers:

```ts
export interface NodeAddress {
  family: 4 | 6
  address: string
  port: number
}

export function nodeAddressFromMultiaddr (ma: Multiaddr): NodeAddress {
  const components = ma.getComponents()
  const ip = components.find(c => c.name === 'ip4' || c.name === 'ip6')
  const udp = components.find(c => c.name === 'udp')
  return {
    family: ip?.name === 'ip4' ? 4 : 6,
    address: ip?.value ?? '',
    port: parseInt(udp?.value ?? '0', 10)
  }
}
```

Also add a helper to extract the host string from a multiaddr (replaces `ma.toOptions().host`):

```ts
export function hostFromMultiaddr (ma: Multiaddr): string {
  const components = ma.getComponents()
  const ip = components.find(c => c.name === 'ip4' || c.name === 'ip6')
  return ip?.value ?? ''
}

export function portFromMultiaddr (ma: Multiaddr): number {
  const components = ma.getComponents()
  const udp = components.find(c => c.name === 'udp')
  return parseInt(udp?.value ?? '0', 10)
}
```

**Step 2: Verify it compiles**

Run: `npx tsc --noEmit src/utils.ts` (or rely on later build step)

**Step 3: Commit**

```
feat: add multiaddr parsing helpers for v13 compatibility
```

---

### Task 3: Rewrite `src/stream.ts`

The stream class must adapt from the v2 `AbstractStream` (async sendData, sourcePush, etc.)
to the v7 `AbstractStream` (synchronous sendData returning SendResult, onData, etc.)

**Files:**
- Modify: `src/stream.ts`

**Step 1: Rewrite the stream implementation**

```ts
import { AbstractStream } from '@libp2p/utils/abstract-stream'
import { Uint8ArrayList } from 'uint8arraylist'
import type * as napi from './napi.js'
import type { AbortOptions } from '@libp2p/interface'
import type { AbstractStreamInit, SendResult } from '@libp2p/utils/abstract-message-stream'

export interface QuicStreamInit extends AbstractStreamInit {
  stream: napi.Stream
}

export class QuicStream extends AbstractStream {
  readonly #stream: napi.Stream

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
    // Fire-and-forget the async write — quinn buffers internally
    void this.#stream.write(data.subarray()).then(
      () => { this.log.trace('wrote %d bytes', data.byteLength) },
      (err) => { this.log.error('write error - %e', err) }
    )
    return { sentBytes: data.byteLength, canSendMore: true }
  }

  sendReset (_err: Error): void {
    void this.#stream.resetWrite()
    void this.#stream.stopRead()
  }

  async sendCloseWrite (_options?: AbortOptions): Promise<void> {
    await this.#stream.finishWrite()
  }

  async sendCloseRead (_options?: AbortOptions): Promise<void> {
    await this.#stream.stopRead()
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

        this.onData(new Uint8ArrayList(chunk))

        this.log.trace('read %d bytes', chunk.length)
      }
    } catch (err: any) {
      this.log.error('source error - %e', err)

      if (err.code === 'Unknown') {
        // clean exit
        this.onRemoteCloseRead()
        return
      }

      this.abort(err)
    } finally {
      this.onRemoteCloseWrite()
    }
  }
}

export const CHUNK_SIZE = 4096
```

**Step 2: Verify it compiles (may have errors until connection/muxer are updated)**

**Step 3: Commit**

```
feat: rewrite QuicStream for libp2p v3 MessageStream API
```

---

### Task 4: Rewrite `src/connection.ts`

Replace the manual `TypedEventEmitter`-based `MultiaddrConnection` with
`AbstractMultiaddrConnection` from `@libp2p/utils`.

**Files:**
- Modify: `src/connection.ts`

**Step 1: Rewrite the connection implementation**

```ts
import { AbstractMultiaddrConnection } from '@libp2p/utils/abstract-multiaddr-connection'
import { multiaddr } from '@multiformats/multiaddr'
import type * as napi from './napi.js'
import type { AbortOptions, CounterGroup, Logger, MessageStreamDirection } from '@libp2p/interface'
import type { SendResult } from '@libp2p/utils/abstract-message-stream'
import type { Multiaddr } from '@multiformats/multiaddr'
import type { Uint8ArrayList } from 'uint8arraylist'

interface QuicConnectionInit {
  connection: napi.Connection
  log: Logger
  direction: MessageStreamDirection
  metrics?: CounterGroup
  metricsPrefix?: string
}

export class QuicConnection extends AbstractMultiaddrConnection {
  readonly #connection: napi.Connection

  constructor (init: QuicConnectionInit) {
    const remoteAddr = multiaddr(init.connection.remoteMultiaddr())

    super({
      remoteAddr,
      direction: init.direction,
      log: init.log,
      metrics: init.metrics,
      metricPrefix: init.metricsPrefix
    })

    this.#connection = init.connection

    // close maconn when connection is closed by remote
    this.#connection.closed().then(() => {
      this.onTransportClosed()
    }, (err) => {
      this.onTransportClosed(err)
    })
  }

  sendData (data: Uint8ArrayList): SendResult {
    // QUIC maConn doesn't send byte-level data directly (streams handle that)
    // but the interface requires this method
    return { sentBytes: data.byteLength, canSendMore: true }
  }

  sendReset (_err: Error): void {
    this.#connection.abort()
  }

  sendPause (): void {
    // QUIC handles flow control at transport level
  }

  sendResume (): void {
    // QUIC handles flow control at transport level
  }

  async sendClose (_options?: AbortOptions): Promise<void> {
    this.#connection.abort()
  }
}
```

**Step 2: Verify it compiles (may have errors until transport/listener are updated)**

**Step 3: Commit**

```
feat: rewrite QuicConnection using AbstractMultiaddrConnection
```

---

### Task 5: Rewrite `src/stream-muxer.ts`

Replace the v2 `StreamMuxer` (source/sink, newStream, onIncomingStream callback) with
the v3 interface (TypedEventTarget, createStream, 'stream' events).

**Files:**
- Modify: `src/stream-muxer.ts`

**Step 1: Rewrite the stream muxer implementation**

```ts
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

  createStreamMuxer (_maConn: MessageStream): StreamMuxer {
    return new QuicStreamMuxer({
      connection: this.#connection,
      logger: this.init.logger
    })
  }
}

interface QuicStreamMuxerInit {
  connection: napi.Connection
  logger: ComponentLogger
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
    await Promise.all(this.streams.map(async (stream) => stream.close(options)))
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
```

**Step 2: Verify it compiles**

**Step 3: Commit**

```
feat: rewrite QuicStreamMuxer for libp2p v3 StreamMuxer API
```

---

### Task 6: Update `src/transport.ts`

Update the upgrader calls for v3 (`remotePeer` required when `skipEncryption: true`),
replace `ma.nodeAddress()` with the new helper, and handle `upgradeInbound` returning
`Promise<void>`.

**Files:**
- Modify: `src/transport.ts`

**Step 1: Update imports and transport implementation**

Key changes:
1. Import `peerIdFromString` from `@libp2p/peer-id`
2. Import `nodeAddressFromMultiaddr` from `./utils.js`
3. Replace `ma.nodeAddress()` with `nodeAddressFromMultiaddr(ma)`
4. Add `remotePeer` to upgrader options
5. Remove `signal` from upgrader options (use `createInboundAbortSignal` in listener)

```ts
import { peerIdFromString } from '@libp2p/peer-id'
import { privateKeyToProtobuf } from '@libp2p/crypto/keys'
import { AbortError, serviceCapabilities, transportSymbol } from '@libp2p/interface'
import { QuicConnection } from './connection.js'
import { dialFilter, listenFilter } from './filter.js'
import { QuicListener } from './listener.js'
import * as napi from './napi.js'
import { QuicStreamMuxerFactory } from './stream-muxer.js'
import { nodeAddressFromMultiaddr } from './utils.js'
import type { QuicComponents, QuicDialOptions, QuicOptions } from './index.js'
import type { QuicCreateListenerOptions } from './listener.js'
import type { Connection, CounterGroup, Listener, Logger, MultiaddrFilter, Transport } from '@libp2p/interface'
import type { Multiaddr } from '@multiformats/multiaddr'

interface QuicTransportMetrics {
  events?: CounterGroup
  errors?: CounterGroup
}

export class QuicTransport implements Transport {
  readonly [Symbol.toStringTag]: string = 'quic'
  readonly [transportSymbol] = true
  readonly [serviceCapabilities]: string[] = [
    '@libp2p/transport'
  ]

  readonly log: Logger
  readonly components: QuicComponents
  readonly metrics: QuicTransportMetrics

  readonly #config: napi.QuinnConfig

  readonly #clients: {
    ip4: napi.Client
    ip6: napi.Client
  }

  readonly listenFilter: MultiaddrFilter
  readonly dialFilter: MultiaddrFilter

  constructor (components: QuicComponents, options: QuicOptions) {
    const privateKeyProto = privateKeyToProtobuf(components.privateKey)
    const config = { ...options, privateKeyProto }

    this.log = components.logger.forComponent('libp2p:quic:transport')
    this.components = components

    this.#config = new napi.QuinnConfig(config)
    this.#clients = {
      ip4: new napi.Client(this.#config, 0),
      ip6: new napi.Client(this.#config, 1)
    }

    this.metrics = {
      events: this.components.metrics?.registerCounterGroup('libp2p_quic_dialer_events_total', {
        label: 'event',
        help: 'Total count of QUIC dialer events by type'
      }),
      errors: this.components.metrics?.registerCounterGroup('libp2p_quic_dialer_errors_total', {
        label: 'event',
        help: 'Total count of QUIC dialer errors by type'
      })
    }

    this.listenFilter = listenFilter
    this.dialFilter = dialFilter

    this.log('new')
  }

  async dial (ma: Multiaddr, options: QuicDialOptions): Promise<Connection> {
    if (options.signal?.aborted) {
      throw new AbortError()
    }

    this.log('dialing', ma.toString())
    const addr = nodeAddressFromMultiaddr(ma)
    const dialer = addr.family === 4 ? this.#clients.ip4 : this.#clients.ip6

    const dialPromise = dialer.outboundConnection(addr.address, addr.port)
    dialPromise
      .then(() => this.metrics.events?.increment({ connect: true }))
      .catch(() => this.metrics.events?.increment({ error: true }))
    const connection = await dialPromise

    let maConn: QuicConnection

    try {
      maConn = new QuicConnection({
        connection,
        log: this.components.logger.forComponent(`libp2p:quic:connection:${connection.id()}:outbound`),
        direction: 'outbound',
        metrics: this.metrics?.events
      })
    } catch (err) {
      this.metrics.errors?.increment({ outbound_to_connection: true })
      throw err
    }

    try {
      this.log('new outbound connection %a', maConn.remoteAddr)

      // Extract remote peer ID from the multiaddr's /p2p/ component
      const p2pComponent = maConn.remoteAddr.getComponents().find(c => c.name === 'p2p')
      if (p2pComponent?.value == null) {
        throw new Error('Remote multiaddr does not contain a peer ID')
      }
      const remotePeer = peerIdFromString(p2pComponent.value)

      return await options.upgrader.upgradeOutbound(maConn, {
        skipEncryption: true,
        skipProtection: true,
        remotePeer,
        muxerFactory: new QuicStreamMuxerFactory({
          connection,
          logger: this.components.logger
        }),
        signal: options.signal
      })
    } catch (err: any) {
      this.metrics.errors?.increment({ outbound_upgrade: true })
      this.log.error('error upgrading outbound connection - %e', err)
      maConn.abort(err)
      throw err
    }
  }

  createListener (options: QuicCreateListenerOptions): Listener {
    return new QuicListener({
      options,
      config: this.#config,
      logger: this.components.logger,
      metrics: this.components.metrics
    })
  }
}
```

**Step 2: Verify it compiles**

**Step 3: Commit**

```
feat: update QuicTransport for libp2p v3 upgrader API
```

---

### Task 7: Update `src/listener.ts`

Replace `ma.nodeAddress()` with the helper, update `upgradeInbound` to handle
`Promise<void>` return, and fix `multiaddr.toString()` → `ma.toString()`.

**Files:**
- Modify: `src/listener.ts`

**Step 1: Update listener implementation**

Key changes:
1. Import `nodeAddressFromMultiaddr` from `./utils.js`
2. Import `peerIdFromString` from `@libp2p/peer-id`
3. Replace `ma.nodeAddress()` calls with `nodeAddressFromMultiaddr(ma)`
4. Add `remotePeer` to `upgradeInbound` options
5. `upgradeInbound` returns `void` now — remove result usage
6. Fix line 139: `multiaddr.toString()` → `ma.toString()`

The `getComponents()` API is still compatible (returns `{ code, name, value }`) so the
port replacement logic in `listen()` (lines 119-127) stays largely the same.

```ts
import { peerIdFromString } from '@libp2p/peer-id'
import { setMaxListeners, TypedEventEmitter } from '@libp2p/interface'
import { multiaddr } from '@multiformats/multiaddr'
import { raceSignal } from 'race-signal'
import { QuicConnection } from './connection.js'
import * as napi from './napi.js'
import { QuicStreamMuxerFactory } from './stream-muxer.js'
import { getMultiaddrs, nodeAddressFromMultiaddr } from './utils.js'
import type { ComponentLogger, CounterGroup, CreateListenerOptions, Listener, ListenerEvents, Logger, Metrics } from '@libp2p/interface'
import type { Multiaddr } from '@multiformats/multiaddr'

export interface QuicCreateListenerOptions extends CreateListenerOptions {

}

export interface QuicListenerMetrics {
  events?: CounterGroup
  errors?: CounterGroup
}

interface QuicListenerInit {
  options: QuicCreateListenerOptions
  config: napi.QuinnConfig
  logger: ComponentLogger
  metrics?: Metrics
}

type QuicListenerState = {
  status: 'ready'
} | {
  status: 'listening'
  listener: napi.Server
  listenAddr: Multiaddr
  controller: AbortController
  connections: Set<QuicConnection>
} | {
  status: 'closed'
}

export class QuicListener extends TypedEventEmitter<ListenerEvents> implements Listener {
  readonly #config: napi.QuinnConfig
  readonly init: QuicListenerInit
  readonly options: QuicCreateListenerOptions
  readonly log: Logger
  readonly metrics: QuicListenerMetrics
  private readonly shutdownController: AbortController
  private addr: string

  state: QuicListenerState = { status: 'ready' }

  constructor (init: QuicListenerInit) {
    super()
    this.#config = init.config
    this.init = init
    this.options = init.options
    this.log = init.logger.forComponent('libp2p:quic:listener')
    this.addr = 'unknown'

    this.shutdownController = new AbortController()
    setMaxListeners(Infinity, this.shutdownController.signal)

    init.metrics?.registerMetricGroup('libp2p_quic_inbound_connections_total', {
      label: 'address',
      help: 'Current active connections in QUIC listener',
      calculate: () => {
        if (this.state.status !== 'listening') {
          return {
            [this.addr]: 0
          }
        }

        return {
          [this.addr]: this.state.connections.size
        }
      }
    })

    this.metrics = {
      events: init.metrics?.registerMetricGroup('libp2p_quic_listener_events_total', {
        label: 'address',
        help: 'Total count of QUIC listener events by type'
      }),
      errors: init.metrics?.registerMetricGroup('libp2p_quic_listener_errors_total', {
        label: 'address',
        help: 'Total count of QUIC listener errors by type'
      })
    }

    this.log('new')
  }

  updateAnnounceAddrs (_addrs: Multiaddr[]): void {

  }

  getAddrs (): Multiaddr[] {
    if (this.state.status === 'listening') {
      const ma = this.state.listenAddr
      const addr = nodeAddressFromMultiaddr(ma)

      if (addr.address === '0.0.0.0') {
        return getMultiaddrs('ip4', addr.address, addr.port)
      } else if (addr.address === '::') {
        return getMultiaddrs('ip6', addr.address, addr.port)
      }

      return [this.state.listenAddr]
    }
    return []
  }

  async listen (ma: Multiaddr): Promise<void> {
    const addr = nodeAddressFromMultiaddr(ma)
    const controller = new AbortController()
    const listener = new napi.Server(this.#config, addr.address, addr.port)
    this.addr = `${addr.address}:${addr.port === 0 ? listener.port() : addr.port}`

    // replace wildcard port with actual listening port
    if (addr.port === 0) {
      const components = ma.getComponents()

      for (const component of components) {
        if (component.name === 'udp') {
          component.value = `${listener.port()}`
        }
      }

      ma = multiaddr(components)
    }

    this.state = {
      status: 'listening',
      listener,
      listenAddr: ma,
      controller,
      connections: new Set()
    }
    void this.awaitInboundConnections()
    this.safeDispatchEvent('listening')
    this.log('listening', ma.toString())
  }

  async close (): Promise<void> {
    if (this.state.status === 'listening') {
      this.state.controller.abort()
      for (const conn of this.state.connections) {
        conn.abort(new Error('listener closed'))
      }
      this.state.connections.clear()
      await this.state.listener.abort()
      const listenAddr = this.state.listenAddr
      this.state = { status: 'closed' }
      // stop any in-progress connection upgrades
      this.shutdownController.abort()
      this.safeDispatchEvent('close')
      this.log('closed', listenAddr.toString())
    }
  }

  async awaitInboundConnections (): Promise<void> {
    if (this.state.status === 'listening') {
      const signal = this.state.controller.signal
      const listenAddr = this.state.listenAddr

      while (true) {
        try {
          const listenerPromise = this.state.listener.inboundConnection()
          listenerPromise
            .then(() => this.metrics.events?.increment({ [`${this.addr} connect`]: true }))
            .catch((err) => {
              this.log.error('%a error awaiting inbound connection - %e', listenAddr, err)
              this.metrics.events?.increment({ [`${this.addr} error`]: true })
            })

          const connection = await raceSignal(listenerPromise, signal)
          this.onInboundConnection(connection).catch((e) => {
            this.log.error('%a error handling inbound connection - %e', listenAddr, e)
          })
        } catch (e) {
          this.log.error('%a error accepting connection - %e', listenAddr, e)

          if (signal.aborted) {
            break
          }
        }
      }

      this.log('%s no longer awaiting inbound connections', listenAddr.toString())
    }
  }

  async onInboundConnection (connection: napi.Connection): Promise<void> {
    if (this.state.status !== 'listening') {
      this.log.error('ignoring inbound connection after listener closed')
      connection.abort()
      return
    }

    let maConn: QuicConnection
    try {
      maConn = new QuicConnection({
        connection,
        log: this.init.logger.forComponent(`libp2p:quic:connection:${connection.id()}:inbound`),
        direction: 'inbound',
        metrics: this.metrics?.events,
        metricsPrefix: `${this.addr} `
      })
    } catch (err) {
      this.metrics.errors?.increment({ [`${this.addr} inbound_to_connection`]: true })
      throw err
    }

    try {
      // Extract remote peer ID from the multiaddr's /p2p/ component
      const p2pComponent = maConn.remoteAddr.getComponents().find(c => c.name === 'p2p')
      if (p2pComponent?.value == null) {
        throw new Error('Remote multiaddr does not contain a peer ID')
      }
      const remotePeer = peerIdFromString(p2pComponent.value)

      await this.options.upgrader.upgradeInbound(maConn, {
        skipEncryption: true,
        skipProtection: true,
        remotePeer,
        muxerFactory: new QuicStreamMuxerFactory({
          connection,
          logger: this.init.logger
        }),
        signal: this.shutdownController.signal
      })

      this.state.connections.add(maConn)
      maConn.addEventListener('close', () => {
        if (this.state.status === 'listening') {
          this.state.connections.delete(maConn)
        }
      }, { once: true })
    } catch (err) {
      this.log.error('%s error handling inbound connection', this.state.listenAddr.toString(), err)
      this.metrics.errors?.increment({ [`${this.addr} inbound_upgrade`]: true })
      maConn.abort(err as Error)
    }
  }
}
```

**Step 2: Verify it compiles**

**Step 3: Commit**

```
feat: update QuicListener for libp2p v3
```

---

### Task 8: Update `src/index.ts` and `src/transport.browser.ts`

Minor type import changes.

**Files:**
- Modify: `src/index.ts`
- Modify: `src/transport.browser.ts`

**Step 1: Update `src/index.ts`**

The types `DialTransportOptions`, `ComponentLogger`, `Metrics`, `PrivateKey`, `Transport`
still exist in `@libp2p/interface` v3 — verify and update if needed. The main change is
removing any `it-stream-types` imports.

Check that the current `src/index.ts` compiles as-is. It likely needs no changes since
the types are compatible.

**Step 2: Update `src/transport.browser.ts`**

Check that the browser shim compiles. The imports `serviceCapabilities`, `transportSymbol`
and types `Connection`, `Listener`, `MultiaddrFilter`, `Transport` still exist in v3.

**Step 3: Commit (if any changes were needed)**

```
feat: update index and browser transport for libp2p v3
```

---

### Task 9: Update `src/filter.ts`

Verify the filter still works with multiaddr-matcher v3.

**Files:**
- Modify: `src/filter.ts` (likely no changes needed)

**Step 1: Verify `src/filter.ts` compiles**

`QUIC_V1` is still exported from `@multiformats/multiaddr-matcher` v3, and `exactMatch`
still exists on the matcher. The code should work as-is.

**Step 2: Commit if needed**

---

### Task 10: Build and fix compilation errors

**Step 1: Run the TypeScript build**

Run: `pnpm run build`
Expected: Build succeeds. If there are errors, fix them iteratively.

Common issues to watch for:
- Import paths that changed
- Missing type exports from `@libp2p/interface` v3
- `AbstractStreamInit` vs `MessageStreamInit` — check exact import path
- `SendResult` import path — should be from `@libp2p/utils/abstract-message-stream`
- `CounterGroup` → may have been renamed to `MetricGroup` — check

**Step 2: Fix any compilation errors**

**Step 3: Commit**

```
fix: resolve compilation errors from libp2p v3 migration
```

---

### Task 11: Update tests

**Files:**
- Modify: `test/compliance.spec.ts`
- Modify: `test/transport.spec.ts`
- Modify: `test/util.ts`

**Step 1: Update `test/util.ts`**

Update `@libp2p/logger` import (v5 → v6). The API (`defaultLogger()`) should still work.

**Step 2: Update `test/transport.spec.ts`**

Replace `ma.nodeAddress()` and `ma.toOptions()` calls with the new helpers:
- `addr.nodeAddress().port` → `portFromMultiaddr(addr)`
- `ma.toOptions().host` → `hostFromMultiaddr(ma)`
- `addr.toOptions().host` → `hostFromMultiaddr(addr)`
- `const { host, port } = addr.toOptions()` → `const host = hostFromMultiaddr(addr); const port = portFromMultiaddr(addr)`

Import the helpers from `../src/utils.js`.

The `Upgrader` stub (`stubInterface<Upgrader>()`) should still work via sinon-ts
since it auto-generates stubs for all interface methods.

**Step 3: Update `test/compliance.spec.ts`**

The v7 compliance tests now use full `libp2p` instances. The setup API changed
significantly — now returns `Libp2pInit` config objects instead of raw transport/upgrader
config.

```ts
import transportCompliance from '@libp2p/interface-compliance-tests/transport'
import { QUIC_V1 } from '@multiformats/multiaddr-matcher'
import { quic } from '../src/index.js'

describe('Interface compliance tests (IPv4)', () => {
  transportCompliance({
    async setup () {
      const dialer = {
        transports: [
          quic()
        ],
        connectionMonitor: {
          enabled: false
        }
      }

      return {
        dialer,
        listener: {
          addresses: {
            listen: [
              '/ip4/0.0.0.0/udp/0/quic-v1',
              '/ip4/0.0.0.0/udp/0/quic-v1'
            ]
          },
          ...dialer
        },
        dialMultiaddrMatcher: QUIC_V1,
        listenMultiaddrMatcher: QUIC_V1
      }
    },
    async teardown () {}
  })
})
```

The compliance test format looks compatible — verify by running.

**Step 4: Commit**

```
feat: update tests for libp2p v3
```

---

### Task 12: Run tests and fix failures

**Step 1: Run the unit tests**

Run: `pnpm test`
Expected: Tests should pass. Fix any runtime failures.

**Step 2: Run compliance tests specifically**

Run: `pnpm test -- --grep "compliance"`

**Step 3: Fix any runtime issues**

Likely issues:
- Connection events not firing properly (close events, stream cleanup)
- Timing issues with fire-and-forget writes
- Multiaddr parsing edge cases

**Step 4: Commit fixes**

```
fix: resolve test failures from libp2p v3 migration
```

---

### Task 13: Clean up and final verification

**Step 1: Run linting**

Run: `pnpm run lint`

**Step 2: Run dep-check**

Run: `pnpm run dep-check`
Verify `it-stream-types` is no longer needed and no other deps are missing.

**Step 3: Final test run**

Run: `pnpm test`

**Step 4: Commit any cleanup**

```
chore: clean up after libp2p v3 migration
```
