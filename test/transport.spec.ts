/* eslint-env mocha */

import { isIPv4, isIPv6 } from '@chainsafe/is-ip'
import { generateKeyPair } from '@libp2p/crypto/keys'
import { defaultLogger } from '@libp2p/logger'
import { multiaddr } from '@multiformats/multiaddr'
import { expect } from 'aegir/chai'
import { pEvent } from 'p-event'
import { stubInterface } from 'sinon-ts'
import { quic } from '../src/index.js'
import { nodeAddressFromMultiaddr } from '../src/utils.js'
import { createComponents } from './util.js'
import type { QuicComponents } from '../src/index.js'
import type { QuicListener } from '../src/listener.js'
import type { Connection, Listener, MultiaddrConnection, Startable, Transport, Upgrader } from '@libp2p/interface'
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

  it('can dial after stop and start', async () => {
    const transport = quic({ ipv6: false })(components)
    const startable = transport as typeof transport & {
      start(): Promise<void>
      stop(): Promise<void>
    }
    const upgrader = {
      upgradeInbound: async () => {},
      upgradeOutbound: async (conn: MultiaddrConnection) => conn as unknown as Connection
    } as unknown as Upgrader
    const listener = transport.createListener({ upgrader })
    listeners.push(listener)

    await Promise.all([
      pEvent(listener, 'listening'),
      listener.listen(multiaddr('/ip4/127.0.0.1/udp/0/quic-v1'))
    ])

    await startable.stop()
    await startable.start()

    const [addr] = listener.getAddrs()
    expect(addr).to.not.equal(undefined)

    const conn = await transport.dial(addr, { upgrader, signal: new AbortController().signal })
    await conn.close()
    await startable.stop()
  })

  it('dials from the listen port by default', async () => {
    let inbound: (remoteAddr: Multiaddr) => void = () => {}
    const remoteAddr = new Promise<Multiaddr>(resolve => { inbound = resolve })
    const upgrader = {
      upgradeInbound: async (conn: MultiaddrConnection) => { inbound(conn.remoteAddr) },
      upgradeOutbound: async (conn: MultiaddrConnection) => conn as unknown as Connection
    } as unknown as Upgrader

    const listen = async (reuseListenPort?: boolean): Promise<{ transport: Transport, addr: Multiaddr }> => {
      const transport = quic({ ipv6: false, reuseListenPort })(await createComponents())
      const listener = transport.createListener({ upgrader })
      listeners.push(listener)
      await listener.listen(multiaddr('/ip4/127.0.0.1/udp/0/quic-v1'))
      return { transport, addr: listener.getAddrs()[0] }
    }

    const target = await listen(false)
    const dialer = await listen()

    const conn = await dialer.transport.dial(target.addr, { upgrader, signal: AbortSignal.timeout(5_000) })

    expect(nodeAddressFromMultiaddr(await remoteAddr).port).to.equal(nodeAddressFromMultiaddr(dialer.addr).port)
    await conn.close()
  })

  it('reuses only an open listener that can reach the target', async () => {
    const listener = quic({ ipv6: false })(components).createListener({ upgrader: stubInterface<Upgrader>() }) as QuicListener
    listeners.push(listener)
    await listener.listen(multiaddr('/ip4/127.0.0.1/udp/0/quic-v1'))

    expect(listener.server({ family: 4, address: '127.0.0.1', port: 1 })).to.not.equal(undefined)
    expect(listener.server({ family: 6, address: '::1', port: 1 })).to.equal(undefined)
    expect(listener.server({ family: 4, address: '192.0.2.1', port: 1 })).to.equal(undefined)

    const closing = listener.close()
    expect(listener.server({ family: 4, address: '127.0.0.1', port: 1 })).to.equal(undefined)
    await closing
  })

  it('does not reuse a listener for a family without a client', async () => {
    const upgrader = stubInterface<Upgrader>()
    const target = quic({ ipv6: false })(components).createListener({ upgrader })
    listeners.push(target)
    await target.listen(multiaddr('/ip4/127.0.0.1/udp/0/quic-v1'))
    const [addr] = target.getAddrs()

    const ipv6Only = quic({ ipv4: false })(await createComponents())
    const ipv6OnlyListener = ipv6Only.createListener({ upgrader })
    listeners.push(ipv6OnlyListener)
    await ipv6OnlyListener.listen(multiaddr('/ip4/127.0.0.1/udp/0/quic-v1'))
    await expect(ipv6Only.dial(addr, { upgrader, signal: AbortSignal.timeout(5_000) })).to.eventually.be.rejectedWith('No QUIC client available for IPv4')

    const stopped = quic({ ipv6: false })(await createComponents()) as Transport & Startable
    const stoppedListener = stopped.createListener({ upgrader })
    listeners.push(stoppedListener)
    await stoppedListener.listen(multiaddr('/ip4/127.0.0.1/udp/0/quic-v1'))
    await stopped.stop()
    await expect(stopped.dial(addr, { upgrader, signal: AbortSignal.timeout(5_000) })).to.eventually.be.rejectedWith('No QUIC client available for IPv4')
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
      expect(nodeAddressFromMultiaddr(addr).port).to.be.greaterThan(0, 'did not translate wildcard port')
    }

    if (wildcard) {
      const host = nodeAddressFromMultiaddr(ma).address

      for (const addr of addrs) {
        expect(nodeAddressFromMultiaddr(addr).address).to.not.equal(host, 'did not translate wildcard host')
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
      const { address: host, port } = nodeAddressFromMultiaddr(addr)
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
      expect(nodeAddressFromMultiaddr(addr).port).to.equal(14000, 'did not listen on port')
    }
  })
})
