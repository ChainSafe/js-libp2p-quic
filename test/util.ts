import { generateKeyPair } from '@libp2p/crypto/keys'
import { defaultLogger } from '@libp2p/logger'
import type { QuicComponents } from '../src/index.js'

export async function createComponents (): Promise<QuicComponents> {
  return {
    privateKey: await generateKeyPair('Ed25519'),
    logger: defaultLogger()
  }
}
