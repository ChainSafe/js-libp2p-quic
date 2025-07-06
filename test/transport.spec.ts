/* eslint-env mocha */

import { isIPv4, isIPv6 } from '@chainsafe/is-ip'
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
  let listeners: Listener[]

  beforeEach(async () => {
    listeners = []
    components = await createComponents()
  })

  afterEach(async () => {
    await Promise.all(
      listeners.map(l => l.close())
    )
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
    const listener = transport.createListener({
      upgrader: stubInterface<Upgrader>()
    })
    listeners.push(listener)

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

  it('supports listening on multiple wildcards', async () => {
    const components = {
      logger: defaultLogger(),
      privateKey: await generateKeyPair('Ed25519')
    }

    const transport = quic()(components)
    const ip4Listener = transport.createListener({
      upgrader: stubInterface<Upgrader>()
    })
    listeners.push(ip4Listener)
    const ip6Listener = transport.createListener({
      upgrader: stubInterface<Upgrader>()
    })
    listeners.push(ip6Listener)

    await Promise.all([
      pEvent(ip4Listener, 'listening'),
      ip4Listener.listen(multiaddr('/ip4/0.0.0.0/udp/0/quic-v1')),
      pEvent(ip6Listener, 'listening'),
      ip6Listener.listen(multiaddr('/ip6/::/udp/0/quic-v1'))
    ])

    const addrs = [
      ...ip4Listener.getAddrs(),
      ...ip6Listener.getAddrs()
    ]

    let hadIp4 = false
    let hadIp6 = false

    for (const addr of addrs) {
      const { host, port } = addr.toOptions()
      expect(port).to.be.greaterThan(0, 'did not translate wildcard port')

      if (isIPv4(host)) {
        hadIp4 = true
        expect(host).to.not.equal('0.0.0.0', 'did not translate wildcard host')
      } else if (isIPv6(host)) {
        hadIp6 = true
        expect(host).to.not.equal('::', 'did not translate wildcard host')
      } else {
        throw new Error(`Host "${host}" was neither IPv4 nor IPv6`)
      }
    }

    expect(hadIp4).to.be.true('did not listen on IPv4 addresses')
    expect(hadIp6).to.be.true('did not listen on IPv6 addresses')
  })

  it('supports listening the same port for different families', async () => {
    const components = {
      logger: defaultLogger(),
      privateKey: await generateKeyPair('Ed25519')
    }

    const transport = quic()(components)
    const ip4Listener = transport.createListener({
      upgrader: stubInterface<Upgrader>()
    })
    listeners.push(ip4Listener)
    const ip6Listener = transport.createListener({
      upgrader: stubInterface<Upgrader>()
    })
    listeners.push(ip6Listener)

    await Promise.all([
      pEvent(ip4Listener, 'listening'),
      ip4Listener.listen(multiaddr('/ip4/127.0.0.1/udp/14000/quic-v1')),
      pEvent(ip6Listener, 'listening'),
      ip6Listener.listen(multiaddr('/ip6/::1/udp/14000/quic-v1'))
    ])

    const addrs = [
      ...ip4Listener.getAddrs(),
      ...ip6Listener.getAddrs()
    ]

    expect(addrs).to.have.lengthOf(2, 'did not listen on correct amount of addresses')

    for (const addr of addrs) {
      const { host, port } = addr.toOptions()
      expect(port).to.equal(14000, 'did not listen on port')
    }
  })
})
