import transportCompliance from '@libp2p/interface-compliance-tests/transport'
import { QUICV1 } from '@multiformats/multiaddr-matcher'
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
        dialMultiaddrMatcher: QUICV1,
        listenMultiaddrMatcher: QUICV1
      }
    },
    async teardown () {}
  })
})

// IPv6 isn't always available in CI
if (!process.env.CI) {
  describe('Interface compliance tests (IPv6)', function () {
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
}
