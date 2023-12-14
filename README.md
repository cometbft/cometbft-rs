# cometbft-rs

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Audit Status][audit-image]][audit-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust Stable][rustc-image]

[CometBFT] client framework in Rust with [TLA+ specifications](/docs/spec).

CometBFT is a high-performance blockchain consensus engine for Byzantine fault
tolerant applications written in any programming language.

## CometBFT Compatibility

cometbft-rs has been tested for compatibility with CometBFT v0.34.x, v0.37.x and v0.38.x.

## Requirements

Tested against the latest stable version of Rust. May work with older versions.

### Semantic Versioning

We do our best to follow [Semantic Versioning](https://semver.org/). However, as
we are pre-v1.0.0, we use the MINOR version to refer to breaking changes and the
PATCH version for features, improvements, and fixes.

We use the same version for all crates and release them collectively.

## Documentation

See each component for the relevant documentation.

Libraries:

- [cometbft](./cometbft) - CometBFT
- [cometbft-light-client](./light-client) - CometBFT light client library
  for verifying signed headers and tracking validator set changes
- [cometbft-light-client-detector](./light-client-detector) - Library for
  detecting and reporting attacks against the CometBFT light client
- [cometbft-light-client-cli](./light-client-cli) - CLI for the light client,
  for verifying headers, detecting attacks and reporting them.
- [cometbft-light-client-js](./light-client-js) - Low-level WASM interface for
  interacting with the CometBFT light client verification functionality
- [cometbft-proto](./proto) - Protobuf data structures (generated using Prost)
  for wire-level interaction with CometBFT
- [cometbft-rpc](./rpc) - CometBFT RPC client and response types

## Releases

Release tags can be found on
[GitHub](https://github.com/cometbft/cometbft-rs/releases).

Crates are released on [crates.io](https://crates.io).

## Contributing

The CometBFT protocols are specified in English in the [cometbft/cometbft
repo](https://github.com/cometbft/cometbft/tree/main/spec). Any protocol
changes or clarifications should be contributed there.

This repo contains the TLA+ specifications and Rust implementations for various
components of CometBFT. See the [CONTRIBUTING.md][contributing] to start
contributing.


## Resources

Software, Specs, and Documentation

- [CometBFT Datastructures Spec](https://github.com/cometbft/cometbft/tree/main/spec)
- [CometBFT in Go](https://github.com/cometbft/cometbft)
- [Docs for CometBFT in Go](http://docs.cometbft.com/)

Papers

- [The latest gossip on BFT consensus](https://arxiv.org/abs/1807.04938)
- [Ethan Buchman's Master's Thesis on Tendermint](https://atrium.lib.uoguelph.ca/xmlui/handle/10214/9769)

## License

Copyright © 2020 Informal Systems and contributors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use the files in this repository except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/cometbft.svg
[crate-link]: https://crates.io/crates/cometbft
[docs-image]: https://docs.rs/cometbft/badge.svg
[docs-link]: https://docs.rs/cometbft/
[build-image]: https://github.com/cometbft/cometbft-rs/workflows/Rust/badge.svg
[build-link]: https://github.com/cometbft/cometbft-rs/actions?query=workflow%3ARust
[audit-image]: https://github.com/cometbft/cometbft-rs/workflows/Audit-Check/badge.svg
[audit-link]: https://github.com/cometbft/cometbft-rs/actions?query=workflow%3AAudit-Check
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/interchainio/cometbft-rs/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-stable-blue.svg

[//]: # (general links)

[cometbft-docs-link]: https://docs.rs/cometbft/
[cometbft-rpc-docs-link]: https://docs.rs/cometbft-rpc/
[CometBFT]: https://github.com/cometbft/cometbft
[cometbft-light-client-docs-link]: https://docs.rs/cometbft-light-client/
[cometbft-secret-conn]: https://github.com/cometbft/cometbft/blob/v0.34.x/spec/p2p/peer.md#authenticated-encryption-handshake
[contributing]: ./CONTRIBUTING.md
