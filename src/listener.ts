import { TypedEventEmitter } from '@libp2p/interface'
import type { ComponentLogger, Connection, CounterGroup, CreateListenerOptions, Listener, ListenerEvents, Logger, Metrics } from '@libp2p/interface'
import type { Multiaddr } from '@multiformats/multiaddr'

import * as napi from './napi.js'
import { QuicConnection } from './connection.js'
import { QuicStreamMuxerFactory } from './stream-muxer.js'

export type QuicCreateListenerOptions = CreateListenerOptions & {}

export type QuicListenerMetrics = {
  events: CounterGroup
}

type QuicListenerInit = {
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
  connections: Set<Connection>
} | {
  status: 'closed'
}

export class QuicListener extends TypedEventEmitter<ListenerEvents> implements Listener {
  readonly #config: napi.QuinnConfig
  readonly init: QuicListenerInit
  readonly options: QuicCreateListenerOptions
  readonly log: Logger
  readonly metrics?: QuicListenerMetrics

  state: QuicListenerState = { status: 'ready' }

  constructor(init: QuicListenerInit) {
    super()
    this.#config = init.config
    this.init = init
    this.options = init.options
    this.log = init.logger.forComponent('libp2p:quic:listener')

    if (init.metrics != null) {
      this.metrics = {
        events: init.metrics.registerMetricGroup('libp2p_quic_listener_events_total', {
          label: 'address',
          help: 'Total count of QUIC listener events by type'
        })
      }
    }

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
          const listenerPromise = this.state.listener.inboundConnection()
          listenerPromise
            .then(() => this.metrics?.events.increment({ connect: true }))
            .catch(() => this.metrics?.events.increment({ error: true }))
          const connection = await Promise.race([
            aborted,
            listenerPromise,
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
      metrics: this.metrics?.events,
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
