import { setMaxListeners, TypedEventEmitter } from '@libp2p/interface'
import { multiaddr } from '@multiformats/multiaddr'
import { raceSignal } from 'race-signal'
import { QuicConnection } from './connection.js'
import * as napi from './napi.js'
import { QuicStreamMuxerFactory } from './stream-muxer.js'
import { getMultiaddrs } from './utils.js'
import type { ComponentLogger, CounterGroup, CreateListenerOptions, Listener, ListenerEvents, Logger, Metrics } from '@libp2p/interface'
import type { Multiaddr } from '@multiformats/multiaddr'

export interface QuicCreateListenerOptions extends CreateListenerOptions {

}

export interface QuicListenerMetrics {
  events: CounterGroup
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
  readonly metrics?: QuicListenerMetrics
  private readonly shutdownController: AbortController

  state: QuicListenerState = { status: 'ready' }

  constructor (init: QuicListenerInit) {
    super()
    this.#config = init.config
    this.init = init
    this.options = init.options
    this.log = init.logger.forComponent('libp2p:quic:listener')

    this.shutdownController = new AbortController()
    setMaxListeners(Infinity, this.shutdownController.signal)

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

  updateAnnounceAddrs (addrs: Multiaddr[]): void {

  }

  getAddrs (): Multiaddr[] {
    if (this.state.status === 'listening') {
      const ma = this.state.listenAddr
      const addr = ma.nodeAddress()

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
    const addr = ma.nodeAddress()
    const controller = new AbortController()
    const listener = new napi.Server(this.#config, addr.address, addr.port)

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
    this.log('listening', multiaddr.toString())
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
            .then(() => this.metrics?.events.increment({ connect: true }))
            .catch(() => this.metrics?.events.increment({ error: true }))

          const connection = await raceSignal(listenerPromise, signal)
          this.onInboundConnection(connection).catch((e) => {
            this.log.error('%s error handling inbound connection', listenAddr.toString(), e)
          })
        } catch (e) {
          this.log.error('%s error accepting connection', listenAddr.toString(), e)

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

    const maConn = new QuicConnection({
      connection,
      logger: this.init.logger,
      direction: 'inbound',
      metrics: this.metrics?.events
    })

    try {
      await this.options.upgrader.upgradeInbound(maConn, {
        skipEncryption: true,
        skipProtection: true,
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
      maConn.abort(err as Error)
    }
  }
}
