import os from 'node:os'
import { multiaddr } from '@multiformats/multiaddr'
import type { Multiaddr } from '@multiformats/multiaddr'

export interface NodeAddress {
  family: 4 | 6
  address: string
  port: number
}

export function nodeAddressFromMultiaddr (ma: Multiaddr): NodeAddress {
  const components = ma.getComponents()
  const ip = components.find(c => c.name === 'ip4' || c.name === 'ip6')
  const udp = components.find(c => c.name === 'udp')

  if (ip?.value == null) {
    throw new Error(`Multiaddr is missing an ip4/ip6 component: ${ma.toString()}`)
  }

  if (udp?.value == null) {
    throw new Error(`Multiaddr is missing a udp component: ${ma.toString()}`)
  }

  return {
    family: ip.name === 'ip4' ? 4 : 6,
    address: ip.value,
    // Multiaddr guarantees that udp port is a valid integer string
    port: parseInt(udp.value, 10)
  }
}

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
