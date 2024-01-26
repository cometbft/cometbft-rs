# CHANGELOG

## v0.1.0-alpha.1

First pre-release of the crates under cometbft-rs naming.

The renamed project's crates are released with versioning restarted from 0.1.0.
The API naming has been changed from Tendermint to CometBFT where appropriate.
The new versioned structure of .proto files published in CometBFT 1.0 is used
to generate protobuf data marshalling structures in cometbft-proto.

### BREAKING CHANGES

- Bulk renaming to refer to CometBFT and cometbft-rs in crate names, APIs, and documentation,
  with the general pattern of `tendermint` changed to `cometbft`, `Tendermint` to `Cometbft`, etc.
  ([\#5](https://github.com/cometbft/cometbft-rs/issues/5))
- The following tendermint-rs crates have been discontinued with no cometbft-rs successors
  ([\#4](https://github.com/cometbft/cometbft-rs/pull/4)):
  - `tendermint-abci`
  - `tendermint-p2p`
  - `tendermint-std-ext`
  - `tendermint-test`
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
- Don’t enable `flex-error/eyre_tracer` feature in crates which don’t
  use eyre directly.  If you’re using eyre, and no other crate enables
  it, you may need to enable that explicitly.
  ([\#1371](https://github.com/informalsystems/tendermint-rs/pull/1371))
- `[cometbft]` Allow null values in `key` and `value` fields of
  `EventAttribute` when deserializing. The serialization schema for the fields
  is changed to `Option<String>`
  ([tendermint-rs#1375](https://github.com/informalsystems/tendermint-rs/issues/1375)).

### IMPROVEMENTS

- `[cometbft-rpc]` Support CometBFT versions 1.x as recognized by
  `CompatMode::from_version`
  ([\#12](https://github.com/cometbft/cometbft-rs/pull/12))
- `[cometbft-rpc]` Export the `http`, `websocket`
  modules under `client`, each with the public `Builder` type
  ([tendermint-rs#1378](https://github.com/informalsystems/tendermint-rs/pull/1378)).

---

cometbft-rs is a fork of [tendermint-rs](https://github.com/informalsystems/tendermint-rs)
as of December 2023.

## Previous changes

For changes released before the creation of cometbft-rs, please refer to the tendermint-rs [CHANGELOG.md](https://github.com/informalsystems/tendermint-rs/blob/a21b24510e331426175ce0782fc581d046aa8413/CHANGELOG.md).

