[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]

See the [repo root] for build status, license, rust version, etc.

# Light Client Verifier

The verification component of the [Light Client]. This is extracted in order to
be able to make use of verification predicates without any of the I/O and
dependencies on the Rust standard library (i.e. to facilitate `no_std` support).

## Documentation

See documentation on [crates.io][docs-link].

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/cometbft-light-client-verifier.svg
[crate-link]: https://crates.io/crates/cometbft-light-client-verifier
[docs-image]: https://docs.rs/cometbft-light-client-verifier/badge.svg
[docs-link]: https://docs.rs/cometbft-light-client-verifier/

[//]: # (general links)

[repo root]: https://github.com/cometbft/cometbft-rs
[quick start]: https://github.com/cometbft/cometbft/blob/main/docs/introduction/quick-start.md
[CometBFT]: https://github.com/cometbft/cometbft
[Light Client]: https://github.com/cometbft/cometbft-rs/tree/main/light-client
