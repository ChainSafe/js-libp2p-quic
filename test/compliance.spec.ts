import transportCompliance from '@libp2p/interface-compliance-tests/transport'
import { multiaddr } from '@multiformats/multiaddr'

import { quic } from '../src/index.js'
import { QuicTransport } from '../src/transport.js'
import { createComponents } from './util.js'

describe.only('Interface compliance tests', () => {
  // Fails these listener tests:
  // - close listener with connections, through timeout
  // - should not handle connection if upgradeInbound throws
  transportCompliance({
    async setup () {
      const transport = quic()(await createComponents())

      const addrs = [
        multiaddr('/ip4/127.0.0.1/udp/9091/quic-v1'),
        multiaddr('/ip4/127.0.0.1/udp/9092/quic-v1'),
        multiaddr('/ip4/127.0.0.1/udp/9093/quic-v1'),
        multiaddr('/ip6/::/udp/9094/quic-v1'),
      ]

      const dial = QuicTransport.prototype.dial
      // Used by the dial tests to simulate a delayed connect
      const connector = {
        delay (delayMs: number) {
          QuicTransport.prototype.dial = async function (...args) {
            await new Promise((resolve) => setTimeout(resolve, delayMs))
            return dial.bind(this)(...args)
          }
        },
        restore () {
          QuicTransport.prototype.dial = dial
        }
      }

      return { dialer: transport, listener: transport, listenAddrs: addrs, dialAddrs: addrs, connector }
    },
    async teardown () {}
  })
})
