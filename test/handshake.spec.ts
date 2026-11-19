/* eslint-env mocha */

import { createSocket } from 'node:dgram'
import { generateKeyPair, privateKeyToProtobuf } from '@libp2p/crypto/keys'
import { expect } from 'aegir/chai'
import { defaultOptions } from '../src/index.js'
import * as napi from '../src/napi.js'
import type { Socket } from 'node:dgram'

const HOST = '127.0.0.1'
const SHORT_TIMEOUT = 1_000
const LONG_TIMEOUT = 8_000

async function createConfig (handshakeTimeout: number): Promise<napi.QuinnConfig> {
  return new napi.QuinnConfig({
    ...defaultOptions,
    handshakeTimeout,
    privateKeyProto: privateKeyToProtobuf(await generateKeyPair('Ed25519'))
  })
}

async function bind (socket: Socket): Promise<number> {
  await new Promise<void>(resolve => { socket.bind(0, HOST, resolve) })
  return socket.address().port
}

interface OneWayRelay {
  port: number
  /**
   * Resolves once datagrams from this many distinct sources have been forwarded
   */
  forwarded(sources: number): Promise<void>
  close(): void
}

/**
 * Forwards datagrams to the target port and drops everything sent in reply,
 * so a handshake started through it never completes
 */
async function createOneWayRelay (targetPort: number): Promise<OneWayRelay> {
  const inbound = createSocket('udp4')
  const outbound = createSocket('udp4')
  const sources = new Set<number>()

  inbound.on('message', (msg, rinfo) => {
    outbound.send(msg, targetPort, HOST, () => { sources.add(rinfo.port) })
  })

  await bind(outbound)

  return {
    port: await bind(inbound),
    forwarded: async (count) => {
      while (sources.size < count) {
        await new Promise(resolve => setTimeout(resolve, 10))
      }
    },
    close: () => {
      inbound.close()
      outbound.close()
    }
  }
}

async function elapsed (promise: Promise<unknown>): Promise<number> {
  const start = Date.now()
  await promise
  return Date.now() - start
}

describe('Handshake', () => {
  let server: napi.Server
  let clients: napi.Client[]
  let relay: OneWayRelay

  async function setup (timeouts: { server: number, client: number }, clientCount = 1): Promise<void> {
    server = new napi.Server(await createConfig(timeouts.server), HOST, 0)
    relay = await createOneWayRelay(server.port())

    for (let i = 0; i < clientCount; i++) {
      clients.push(new napi.Client(await createConfig(timeouts.client), 0))
    }
  }

  /**
   * Start a handshake from each client that reaches the server but never completes
   */
  async function stall (): Promise<Array<Promise<napi.Connection>>> {
    const stalled = clients.map(async client => client.outboundConnection(HOST, relay.port))
    stalled.forEach(promise => { promise.catch(() => {}) })
    await relay.forwarded(clients.length)

    return stalled
  }

  beforeEach(() => {
    clients = []
  })

  afterEach(async () => {
    clients.forEach(client => { client.abort() })
    await server.abort()
    relay.close()
  })

  it('accepts connections while another handshake is in progress', async () => {
    await setup({ server: LONG_TIMEOUT, client: LONG_TIMEOUT })
    await stall()

    const [inbound, outbound] = await Promise.all([
      elapsed(server.inboundConnection()),
      elapsed(clients[0].outboundConnection(HOST, server.port()))
    ])

    expect(inbound).to.be.lessThan(SHORT_TIMEOUT)
    expect(outbound).to.be.lessThan(SHORT_TIMEOUT)
    expect(server.stats().retriedHandshakes).to.equal(0)
  })

  it('accepts connections while many handshakes from the same address are in progress', async () => {
    // more than one address may have in progress at once
    await setup({ server: LONG_TIMEOUT, client: LONG_TIMEOUT }, 20)
    await stall()

    const [inbound, outbound] = await Promise.all([
      elapsed(server.inboundConnection()),
      elapsed(clients[0].outboundConnection(HOST, server.port()))
    ])

    expect(inbound).to.be.lessThan(SHORT_TIMEOUT)
    expect(outbound).to.be.lessThan(SHORT_TIMEOUT)
    // the address was over its share, so it was asked to validate itself first
    expect(server.stats().retriedHandshakes).to.be.greaterThan(0)
  })

  it('times out an inbound handshake that does not complete', async () => {
    // the client outlasts the server, so only the server's own timeout can end the handshake
    await setup({ server: SHORT_TIMEOUT, client: LONG_TIMEOUT })
    await stall()

    const waited = await elapsed((async () => {
      while (server.stats().failedHandshakes === 0) {
        await new Promise(resolve => setTimeout(resolve, 50))
      }
    })())

    expect(server.stats().failedHandshakes).to.equal(1)
    expect(waited).to.be.lessThan(LONG_TIMEOUT / 2)
  })

  it('times out an outbound handshake that does not complete', async () => {
    await setup({ server: LONG_TIMEOUT, client: SHORT_TIMEOUT })
    const [stalled] = await stall()

    const waited = await elapsed(stalled.catch(() => {}))

    await expect(stalled).to.eventually.be.rejected().with.property('message').that.include('handshake timed out')
    expect(waited).to.be.lessThan(LONG_TIMEOUT / 2)
  })
})
