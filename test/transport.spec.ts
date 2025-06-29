/* eslint-env mocha */

import { generateKeyPair } from '@libp2p/crypto/keys'
import { defaultLogger } from '@libp2p/logger'
import { multiaddr } from '@multiformats/multiaddr'
import { expect } from 'aegir/chai'
import { pEvent } from 'p-event'
import { stubInterface } from 'sinon-ts'
import { quic } from '../src/index.js'
import { createComponents } from './util.js'
import type { QuicComponents } from '../src/index.js'
import type { Listener, Upgrader } from '@libp2p/interface'
import type { Multiaddr } from '@multiformats/multiaddr'

describe('Quic Transport', () => {
  let components: QuicComponents
  let listener: Listener

  beforeEach(async () => {
    components = await createComponents()
  })

  afterEach(async () => {
    await listener?.close()
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

  async function testListenAddresses (ma: Multiaddr, wildcard: boolean): Promise<void> {
    const components = {
      logger: defaultLogger(),
      privateKey: await generateKeyPair('Ed25519')
    }

    const transport = quic()(components)
    listener = transport.createListener({
      upgrader: stubInterface<Upgrader>()
    })

    await Promise.all([
      pEvent(listener, 'listening'),
      listener.listen(ma)
    ])

    const addrs = listener.getAddrs()

    for (const addr of addrs) {
      expect(addr.nodeAddress().port).to.be.greaterThan(0, 'did not translate wildcard port')
    }

    if (wildcard) {
      const host = ma.toOptions().host

      for (const addr of addrs) {
        expect(addr.toOptions().host).to.not.equal(host, 'did not translate wildcard host')
      }
    } else {
      expect(addrs).to.have.lengthOf(1, 'listened on too many addresses')
    }
  }

  it('supports listening on ipv4 wildcards', async () => {
    await testListenAddresses(multiaddr('/ip4/0.0.0.0/udp/0/quic-v1'), true)
  })

  it('supports listening on specific ipv4 addresses', async () => {
    await testListenAddresses(multiaddr('/ip4/127.0.0.1/udp/0/quic-v1'), false)
  })

  it('supports listening on ipv6 wildcards', async () => {
    await testListenAddresses(multiaddr('/ip6/::/udp/0/quic-v1'), true)
  })

  it('supports listening on specific ipv6 addresses', async () => {
    await testListenAddresses(multiaddr('/ip6/::1/udp/0/quic-v1'), false)
  })
})
