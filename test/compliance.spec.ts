import transportCompliance from '@libp2p/interface-compliance-tests/transport'
import { QUICV1 } from '@multiformats/multiaddr-matcher'
import { quic } from '../src/index.js'
import { hostSupportsIpv6 } from './util.ts'

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
        dialMultiaddrMatcher: QUICV1,
        listenMultiaddrMatcher: QUICV1
      }
    },
    async teardown () {}
  })
})

describe('Interface compliance tests (IPv6)', function () {
  if (!hostSupportsIpv6()) {
    it.skip('Host does not support IPv6', () => {})
    return
  }

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
              '/ip6/::/udp/0/quic-v1',
              '/ip6/::/udp/0/quic-v1'
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
