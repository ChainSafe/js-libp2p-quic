import os from 'node:os'
import { multiaddr } from '@multiformats/multiaddr'
import type { Multiaddr } from '@multiformats/multiaddr'

const ProtoFamily = { ip4: 'IPv4', ip6: 'IPv6' }

export function getMultiaddrs (proto: 'ip4' | 'ip6', ip: string, port: number): Multiaddr[] {
  const toMa = (ip: string): Multiaddr => multiaddr(`/${proto}/${ip}/udp/${port}/quic-v1`)
  return (isAnyAddr(ip) ? getNetworkAddrs(ProtoFamily[proto]) : [ip]).map(toMa)
}

export function isAnyAddr (ip: string): boolean {
  return ['0.0.0.0', '::'].includes(ip)
}

const networks = os.networkInterfaces()

function getNetworkAddrs (family: string): string[] {
  const addresses: string[] = []

  for (const [, netAddrs] of Object.entries(networks)) {
    if (netAddrs != null) {
      for (const netAddr of netAddrs) {
        if (netAddr.family === family) {
          addresses.push(netAddr.address)
        }
      }
    }
  }

  return addresses
}
