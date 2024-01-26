- `[cometbft-proto]` Change to versioned protobufs published by CometBFT 1.0
  ([\#7](https://github.com/cometbft/cometbft-rs/pull/7)):
  * Instead of side-by-side generated outputs in `v0_34`, `v0_37`, and `v0_38`
    produced from respective checkouts of the CometBFT repository, use the
    single release (currently at 1.0.0-alpha.1) and generate all protobuf
    structures at once.
  * In the new versioned module structure corresponding to the CometBFT 1.0
    protobuf packages, modules represending protocol domains come to the top
    level, e.g. `crate::abci`, `crate::types`. Each of these has a `v1` module
    corresponding to the protobufs released in 1.0.0, and possibly a series of
    `v1beta1`, `v1beta2`, ... modules with earlier revisions of the protocol,
    as documented in the [CometBFT protobuf README](https://github.com/cometbft/cometbft/blob/main/proto/README.md)
- `[cometbft]` Update to the new version structure in `cometbft-proto` ([\#7](https://github.com/cometbft/cometbft-rs/pull/7)):
  * Relocated the modules for version-targeted ABCI support. Now under the
    `abci` module, `v1` provides the `Request` and `Response` enum definitions
    with the variants valid per CometBFT 1.0, while `v1beta*` modules provide
    revisions corresponding to earlier ABCI versions.
- `[cometbft-rpc]` Update to changes in CometBFT 1.0 ([\#7](https://github.com/cometbft/cometbft-rs/pull/7)):
  * Renamed `dialect::v0_37` module to `dialect::v1`.
