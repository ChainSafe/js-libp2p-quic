import { generateKeyPair, marshalPrivateKey } from '@libp2p/crypto/keys'
import { defaultOptions } from '../src/index.js'
import * as napi from '../src/napi.js'

describe('Sanity', () => {
  it.only('create a bunch of connections and streams', async () => {
    const privateKey = await generateKeyPair('Ed25519')
    const config: napi.Config = {
      ...defaultOptions,
      privateKeyProto: marshalPrivateKey(privateKey)
    }
    const quinnConfig = new napi.QuinnConfig(config)

    const client = new napi.Client(quinnConfig, 0)
    const ip = '127.0.0.1'
    const port = 4993
    const server = new napi.Server(quinnConfig, ip, port)

    const nConnections = 100
    const nStreams = 100
    const nData = 1000

    const connections = new Set<napi.Connection>()
    const streams = new Set<napi.Stream>()
    const dataIn = new Set<Buffer>()
    const dataOut = new Set<Buffer>()

    const log = (...args: any[]) => {
      // console.log(...args)
    }

    const onInboundConnection = async (conn: napi.Connection) => {
      connections.add(conn)
      for (let i = 0; i < nStreams; i++) {
        const stream = await conn.inboundStream()
        onInboundStream(stream, i)
      }
    }
    const onInboundStream = async (stream: napi.Stream, connIx: number) => {
      streams.add(stream)
      for (let i = 0; i < nData; i++) {
        const b = Buffer.allocUnsafe(4096)
        const len = await stream.read(b)
        if (len === null) {
          break
        }
        dataOut.add(b.subarray(0, len))
        log('read', connIx, i)
      }
    }
    const onOutboundConnection = async (conn: napi.Connection) => {
      connections.add(conn)
      for (let i = 0; i < nStreams; i++) {
        const stream = await conn.outboundStream()
        onOutboundStream(stream, i)
      }
    }
    const onOutboundStream = async (stream: napi.Stream, connIx: number) => {
      streams.add(stream)
      for (let i = 0; i < nData; i++) {
        const b = Buffer.alloc(nData, i)
        dataIn.add(b)
        log('write', connIx, i)
        await stream.write(b)
      }
    }
    const inbound = Promise.resolve().then(async () => {
      for (let i = 0; i < nConnections; i++) {
        const conn = await server.inboundConnection()
        onInboundConnection(conn)
      }
    })
    const outbound = Promise.resolve().then(async () => {
      for (let i = 0; i < nConnections; i++) {
        const conn = await client.outboundConnection(ip, port)
        onOutboundConnection(conn)
      }
    })
    await Promise.all([inbound, outbound])
    for (const stream of streams) {
      stream.finishWrite()
      stream.stopRead()
    }
    for (const conn of connections) {
      conn.abort()
    }
    for (const data of dataIn) {
      dataIn.delete(data)
    }
    // @ts-expect-error gc may be undefined
    global.gc()
    for (const data of dataOut) {
      dataOut.delete(data)
    }
    await server.abort()
    client.abort()
  })
})