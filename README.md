# @chainsafe/libp2p-quic

[![libp2p.io](https://img.shields.io/badge/project-libp2p-yellow.svg?style=flat-square)](http://libp2p.io/)
[![CI](https://img.shields.io/github/actions/workflow/status/chainsafe/js-libp2p-quic/CI.yml?branch=main\&style=flat-square)](https://github.com/chainsafe/js-libp2p-quic/actions/workflows/CI.yml?query=branch%3Amain)

> A QUIC transport for libp2p

# About

<!--

!IMPORTANT!

Everything in this README between "# About" and "# Install" is automatically
generated and will be overwritten the next time the doc generator is run.

To make changes to this section, please update the @packageDocumentation section
of src/index.js or src/index.ts

To experiment with formatting, please run "npm run docs" from the root of this
repo and examine the changes made.

-->

A [libp2p transport](https://docs.libp2p.io/concepts/transports/overview/) based on the QUIC networking stack.

## Example

```TypeScript
import { createLibp2p } from 'libp2p'
import { quic } from '@chainsafe/libp2p-quic'
import { multiaddr } from '@multiformats/multiaddr'

const node = await createLibp2p({
  transports: [
    quic()
  ]
})

const ma = multiaddr('/ip4/123.123.123.123/udp/1234/quic-v1')

// dial a TCP connection, timing out after 10 seconds
const connection = await node.dial(ma, {
  signal: AbortSignal.timeout(10_000)
})

// use connection...
```

# Install

```console
$ npm i @chainsafe/libp2p-quic
```

# API Docs

- <https://chainsafe.github.io/js-libp2p-quic>

# License

MIT ([LICENSE](https://github.com/ChainSafe/js-libp2p-quic/blob/main/LICENSE) / <http://opensource.org/licenses/MIT>)
