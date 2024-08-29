import transportCompliance from '@libp2p/interface-compliance-tests/transport'
import { multiaddr } from '@multiformats/multiaddr'

import { quic } from '../src/index.js'
import { createComponents } from './util.js'

describe('Interface compliance tests', () => {
  transportCompliance({
    async setup () {
      const transport = quic()(await createComponents())

      const addrs = [
        multiaddr('/ip4/127.0.0.1/udp/9091/quic-v1'),
        multiaddr('/ip4/127.0.0.1/udp/9092/quic-v1'),
        multiaddr('/ip4/127.0.0.1/udp/9093/quic-v1'),
        multiaddr('/ip6/::/udp/9094/quic-v1'),
      ]

      // Used by the dial tests to simulate a delayed connect
      const connector = {
        delay (delayMs: number) {},
        restore () {}
      }

      return { dialer: transport, listener: transport, listenAddrs: addrs, dialAddrs: addrs, connector }
    },
    async teardown () {}
  })
})
