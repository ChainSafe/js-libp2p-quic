import { serviceCapabilities, transportSymbol } from '@libp2p/interface'
import type { Connection, Listener, MultiaddrFilter, Transport } from '@libp2p/interface'

export class QuicTransport implements Transport {
  readonly [Symbol.toStringTag]: string = 'quic'
  readonly [transportSymbol] = true
  readonly [serviceCapabilities]: string[] = [
    '@libp2p/transport'
  ]

  readonly listenFilter: MultiaddrFilter
  readonly dialFilter: MultiaddrFilter

  constructor () {
    throw new Error('QUIC connections are not possible in browsers')
  }

  async dial (): Promise<Connection> {
    throw new Error('QUIC connections are not possible in browsers')
  }

  createListener (): Listener {
    throw new Error('QUIC connections are not possible in browsers')
  }
}
