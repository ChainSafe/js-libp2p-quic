import transportCompliance from '@libp2p/interface-compliance-tests/transport'
import { QUICV1 } from '@multiformats/multiaddr-matcher'
import { quic } from '../src/index.js'

describe('Interface compliance tests (IPv4)', () => {
  // Fails these listener tests:
  // - close listener with connections, through timeout
  // - should not handle connection if upgradeInbound throws
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
              '/ip4/127.0.0.1/udp/9091/quic-v1',
              '/ip4/127.0.0.1/udp/9092/quic-v1'
            ]
          },
          ...dialer
        },
        dialMultiaddrMatcher: QUICV1,
        listenMultiaddrMatcher: QUICV1
      }
    },
    async teardown () {}
  })
})

describe('Interface compliance tests (IPv6)', () => {
  // Fails these listener tests:
  // - close listener with connections, through timeout
  // - should not handle connection if upgradeInbound throws
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
              '/ip6/::/udp/9091/quic-v1',
              '/ip6/::/udp/9092/quic-v1'
            ]
          },
          ...dialer
        },
        dialMultiaddrMatcher: QUICV1,
        listenMultiaddrMatcher: QUICV1
      }
    },
    async teardown () {}
  })
})
