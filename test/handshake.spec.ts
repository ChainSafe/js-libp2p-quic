/* eslint-env mocha */

import { createSocket } from 'node:dgram'
import { generateKeyPair, privateKeyToProtobuf } from '@libp2p/crypto/keys'
import { expect } from 'aegir/chai'
import { defaultOptions } from '../src/index.js'
import * as napi from '../src/napi.js'
import type { Socket } from 'node:dgram'

const HOST = '127.0.0.1'
const HANDSHAKE_TIMEOUT = 2_000

async function createConfig (): Promise<napi.QuinnConfig> {
  return new napi.QuinnConfig({
    ...defaultOptions,
    handshakeTimeout: HANDSHAKE_TIMEOUT,
    privateKeyProto: privateKeyToProtobuf(await generateKeyPair('Ed25519'))
  })
}

async function bind (socket: Socket): Promise<number> {
  await new Promise<void>(resolve => { socket.bind(0, HOST, resolve) })
  return socket.address().port
}

/**
 * Forwards datagrams to the target port and drops everything sent in reply,
 * so a handshake started through it never completes
 */
async function createOneWayRelay (targetPort: number): Promise<{ port: number, forwarded: Promise<void>, close(): void }> {
  const inbound = createSocket('udp4')
  const outbound = createSocket('udp4')
  let onForwarded: () => void = () => {}
  const forwarded = new Promise<void>(resolve => { onForwarded = resolve })

  inbound.on('message', (msg) => {
    outbound.send(msg, targetPort, HOST, () => { onForwarded() })
  })

  await bind(outbound)

  return {
    port: await bind(inbound),
    forwarded,
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
  let client: napi.Client
  let relay: Awaited<ReturnType<typeof createOneWayRelay>>

  beforeEach(async () => {
    server = new napi.Server(await createConfig(), HOST, 0)
    client = new napi.Client(await createConfig(), 0)
    relay = await createOneWayRelay(server.port())
  })

  afterEach(async () => {
    client.abort()
    await server.abort()
    relay.close()
  })

  it('accepts connections while another handshake is in progress', async () => {
    const stalled = client.outboundConnection(HOST, relay.port)
    stalled.catch(() => {})
    // the stalled handshake has reached the server first
    await relay.forwarded

    const [inbound, outbound] = await Promise.all([
      elapsed(server.inboundConnection()),
      elapsed(client.outboundConnection(HOST, server.port()))
    ])

    expect(inbound).to.be.lessThan(HANDSHAKE_TIMEOUT / 2)
    expect(outbound).to.be.lessThan(HANDSHAKE_TIMEOUT / 2)
  })

  it('times out a handshake that does not complete', async function () {
    this.timeout(defaultOptions.maxIdleTimeout)

    const stalled = client.outboundConnection(HOST, relay.port)
    const outbound = await elapsed(stalled.catch((err: Error) => {
      expect(err.message).to.include('handshake timed out')
    }))

    await expect(stalled).to.eventually.be.rejected()
    expect(outbound).to.be.lessThan(defaultOptions.maxIdleTimeout / 2)

    // the server started its handshake no later than the client gave up waiting
    while (server.stats().failedHandshakes === 0) {
      await new Promise(resolve => setTimeout(resolve, 50))
    }

    expect(server.stats().failedHandshakes).to.equal(1)
  })
})
