import type { Multiaddr } from "@multiformats/multiaddr"
import * as mafmt from '@multiformats/mafmt'

// p2p multi-address code
export const CODE_P2P = 421
export const CODE_CIRCUIT = 290
export const CODE_UNIX = 400

export function listenFilter(multiaddrs: Multiaddr[]): Multiaddr[] {
  return multiaddrs.filter((ma) => {
    if (ma.protoCodes().includes(CODE_CIRCUIT)) {
    return false
    }

    return mafmt.QUICV1.matches(ma.decapsulateCode(CODE_P2P))
  })
}

export function dialFilter(multiaddrs: Multiaddr[]): Multiaddr[] {
  return listenFilter(multiaddrs)
}
