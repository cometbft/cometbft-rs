[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]

See the [repo root] for build status, license, rust version, etc.

# Light-Client

Implementation of the [Light Client Verification][light-client-verification]
and [Attack Detection][light-client-detection] protocols.

## Documentation

See documentation on [docs.rs][docs-link].

## Testing

The CometBFT Light Client is primarily tested through unit tests.

### Core Verification

The logic for the core verification of light blocks is entirely self-contained in
the [`predicates`](./src/predicates.rs) module.
This code is exercised through unit tests which test each predicate in isolation
by giving it a set of data along with the expected outcome of each check.

The following command can be used to run only these tests:

```bash
cargo test -p cometbft-light-client predicates
```

#### Model-based tests

We started to employ model-based testing (MBT), which is currently limited 
to the core verification. In MBT, the testing procedure is based on the
[Light Client formal model](./tests/support/model_based/Lightclient_002_draft.tla),
and the tests themselves are simple assertions in the modeling language TLA+.
The current set of [TLA+ tests](./tests/support/model_based/LightTests.tla) is translated
automatically into the set of [JSON fixtures](./tests/support/model_based/single_step).

The following command can be used to run only these tests:

```bash
$ cargo test -p cometbft-light-client --test model_based -- --nocapture
```

Please refer to the [MBT Guide](./tests/support/model_based/README.md),
and the [MBT Abstract](./tests/support/model_based/Abstract.md) for further information.

### Bisection

Similarly to the core verification logic, the algorithm for performing bisecting
verification is exercised via a set of [JSON fixtures](./tests/support/bisection/single_peer)
which encode an initial trusted state, a target block to verify, a set of intermediary blocks,
and the expected result of the bisection algorithm.

These tests target the [`light_client`](./src/light_client.rs) module,
and can be found in the [`tests/light_client.rs`](./tests/light_client.rs) file.

To run the tests:

```bash
$ cargo test -p cometbft-light-client --test light_client bisection
```

### Attack Detection

Please see the [`light-client-detector` crate][detector-crate].

### Voting Power Calculator

The [voting power calculator](./src/operations/voting_power.rs) is exercised through
unit tests which rely on [JSON fixtures](./tests/support/voting_power/) to provide
the calculator with various types of *light blocks* together with the expected result
of the computation.

The following command can be used to run only these tests:

```bash
$ cargo test -p cometbft-light-client voting_power
```

### Integration Tests

This project also includes simple integration test which spawns a light client instance
against a single CometBFT full node which acts both as a primary peer and as its
own witness.

Because this test requires a running CometBFT node, it is ignored by default.
To run this test locally:

```bash
# In one terminal
$ mkdir -p /tmp/cometbft
$ docker run -it --rm -v "/tmp/cometbft:/cometbft" cometbft/cometbft init
$ docker run -it --rm -v "/tmp/cometbft:/cometbft" -p 26657:26657 cometbft/cometbft node --proxy_app=kvstore

# In another terminal
$ cargo test -p cometbft-light-client --test integration -- --ignored --nocapture
```

### Other tests

A few core datastructures, such as the [`PeerList`](./src/peer_list.rs) implementation,
come with unit tests located in the same module as the implementation.

To run these tests together with all tests described above:

```rust
$ cargo test -p cometbft-light-client --all-features
```

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/cometbft-light-client.svg
[crate-link]: https://crates.io/crates/cometbft-light-client
[docs-image]: https://docs.rs/cometbft-light-client/badge.svg
[docs-link]: https://docs.rs/cometbft-light-client/

[//]: # (general links)

[repo root]: https://github.com/cometbft/cometbft-rs
[quick start]: https://github.com/cometbft/cometbft/blob/main/docs/introduction/quick-start.md
[CometBFT]: https://github.com/cometbft/cometbft
[light-client-verification]: https://github.com/cometbft/cometbft-rs/blob/main/docs/spec/lightclient/verification/verification.md
[detector-crate]: https://github.com/cometbft/cometbft-rs/tree/main/light-client-detector
