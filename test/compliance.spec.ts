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
          dialMultiaddrMatcher: QUIC_V1,
          listenMultiaddrMatcher: QUIC_V1
        }
      },
      async teardown () {}
    })
  })
}
