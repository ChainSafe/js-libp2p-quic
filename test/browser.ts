import { generateKeyPair } from '@libp2p/crypto/keys'
import { defaultLogger } from '@libp2p/logger'
import { expect } from 'aegir/chai'
import { isBrowser, isWebWorker } from 'wherearewe'
import { quic } from '../src/index.js'
import type { PrivateKey } from '@libp2p/interface'

describe('browser non-support', () => {
  let privateKey: PrivateKey

  beforeEach(async () => {
    privateKey = await generateKeyPair('Ed25519')
  })

  it('should throw in browsers', async function () {
    if (!isBrowser && !isWebWorker) {
      return this.skip()
    }

    expect(() => {
      quic()({
        logger: defaultLogger(),
        privateKey
      })
    }).to.be.throw()
  })
})
