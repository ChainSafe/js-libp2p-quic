import { QUIC_V1 } from '@multiformats/multiaddr-matcher'
import type { Multiaddr } from '@multiformats/multiaddr'

// p2p multi-address code
export const CODE_P2P = 421
export const CODE_CIRCUIT = 290
export const CODE_UNIX = 400

export function listenFilter (multiaddrs: Multiaddr[]): Multiaddr[] {
  return multiaddrs.filter((ma) => {
    return QUIC_V1.exactMatch(ma)
  })
}

export function dialFilter (multiaddrs: Multiaddr[]): Multiaddr[] {
  return listenFilter(multiaddrs)
}
