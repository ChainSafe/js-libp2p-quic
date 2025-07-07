import { generateKeyPair } from '@libp2p/crypto/keys'
import { defaultLogger } from '@libp2p/logger'
import type { QuicComponents } from '../src/index.js'
import { isIPv6 } from '@chainsafe/is-ip'
import os from 'node:os'

export async function createComponents (): Promise<QuicComponents> {
  return {
    privateKey: await generateKeyPair('Ed25519'),
    logger: defaultLogger()
  }
}

export function hostSupportsIpv6 (): boolean {
  for (const interfaces of Object.values(os.networkInterfaces())) {
    if (interfaces == null) {
      continue
    }

    for (const info of interfaces) {
      if (isIPv6(info.address)) {
        return true
      }
    }
  }

  return false
}
