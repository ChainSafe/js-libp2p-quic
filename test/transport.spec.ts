/* eslint-disable no-console */
/* eslint-env mocha */

import { defaultLogger } from '@libp2p/logger'
import { generateKeyPair } from '@libp2p/crypto/keys'
import { multiaddr } from '@multiformats/multiaddr'
import { expect } from 'aegir/chai'
import { quic, type QuicComponents } from '../src/index.js'

describe('Quic Transport', () => {
  let components: QuicComponents

  beforeEach(async () => {
    components = {
      privateKey: await generateKeyPair('Ed25519'),
      logger: defaultLogger()
    }
  })

  it('transport filter filters out invalid dial multiaddrs', async () => {
    const valid = [
      multiaddr('/ip4/1.2.3.4/udp/1234/quic-v1/p2p/12D3KooWGDMwwqrpcYKpKCgxuKT2NfqPqa94QnkoBBpqvCaiCzWd')
    ]
    const invalid = [
      multiaddr('/ip4/1.2.3.4/udp/1234/quic-v1/p2p/12D3KooWGDMwwqrpcYKpKCgxuKT2NfqPqa94QnkoBBpqvCaiCzWd/p2p-circuit/p2p/12D3KooWGDMwwqrpcYKpKCgxuKT2NfqPqa94QnkoBBpqvCaiCzWd'),
      multiaddr('/ip4/1.2.3.4/udp/1234/webrtc-direct/p2p/12D3KooWGDMwwqrpcYKpKCgxuKT2NfqPqa94QnkoBBpqvCaiCzWd')
    ]

    const t = quic()(components)

    expect(t.dialFilter([
      ...valid,
      ...invalid
    ])).to.deep.equal(valid)
  })
})
