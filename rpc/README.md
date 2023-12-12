[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]

See the [repo root] for build status, license, rust version, etc.

# cometbft-rpc

A Rust implementation of the core types returned by a CometBFT node's RPC 
endpoint. These can be used to deserialize JSON-RPC responses.

All networking related features will be feature guarded to keep the
dependencies small in cases where only the core types are needed.

## Documentation

See documentation on [crates.io][docs-link].

## Client

This crate optionally provides access to different types of RPC client
functionality and different client transports based on which features you
select when using it.

Several client-related features are provided at present:

* `http-client` - Provides `HttpClient`, which is a basic RPC client that
  interacts with remote CometBFT nodes via **JSON-RPC over HTTP or
  HTTPS**. This client does not provide `Event` subscription
  functionality. See the [CometBFT RPC] for more details.
* `websocket-client` - Provides `WebSocketClient`, which provides full
  client functionality, including general RPC functionality as well as
  `Event`] subscription functionality. Can be used over secure
  (`wss://`) and unsecure (`ws://`) connections.

### CLI

A `cometbft-rpc` console application is provided for testing/experimentation
purposes. To build this application:

```bash
# From the cometbft-rpc crate's directory
cd rpc
cargo build --bin cometbft-rpc --features cli

# To run directly and show usage information
cargo run --bin cometbft-rpc --features cli -- --help

# To install the binary to your Cargo binaries path
# (should be globally accessible)
cargo install --bin cometbft-rpc --features cli --path .
```

The application sends its logs to **stderr** and its output to **stdout**, so
it's relatively easy to capture RPC output.

**Usage examples:** (assuming you've installed the binary)

```bash
# Check which RPC commands/endpoints are supported.
cometbft-rpc --help

# Query the status of the CometBFT node bound to tcp://127.0.0.1:26657
cometbft-rpc status

# Submit a transaction to the key/value store ABCI app via a CometBFT node
# bound to tcp://127.0.0.1:26657
cometbft-rpc broadcast-tx-async somekey=somevalue

# Query the value associated with key "somekey" (still assuming a key/value
# store ABCI app)
cometbft-rpc abci-query somekey

# To use an HTTP/S proxy to access your RPC endpoint
cometbft-rpc --proxy-url http://yourproxy:8080 abci-query somekey

# To set your HTTP/S proxy for multiple subsequent queries
export HTTP_PROXY=http://yourproxy:8080
cometbft-rpc abci-query somekey

# Subscribe to receive new blocks (must use the WebSocket endpoint)
# Prints out all incoming events
cometbft-rpc -u ws://127.0.0.1:26657/websocket subscribe "tm.event='NewBlock'"

# If you want to execute a number of queries against a specific endpoint and
# don't feel like re-typing the URL over and over again, just set the
# COMETBFT_RPC_URL environment variable
export COMETBFT_RPC_URL=ws://127.0.0.1:26657/websocket
cometbft-rpc subscribe "tm.event='Tx'"
```

### Mock Clients

Mock clients are included when either of the `http-client` or
`websocket-client` features are enabled to aid in testing. This includes
`MockClient`, which implements both `Client` and `SubscriptionClient`
traits.

### Related

- RPC [core types] in golang
  
- RPC endpoints REST interface documentation:
  <https://docs.cometbft.com/v1/rpc/>

## Testing

The RPC types are directly tested through the [integration
tests](./tests/integration.rs). These tests use fixtures taken from running
CometBFT nodes to ensure compatibility without needing access to a running
node during testing. All of these fixtures were generated manually, and
automatic regeneration of the fixtures is [on our roadmap][autogen-fixtures].

To run these tests locally:

```bash
# From within the rpc crate
cargo test --all-features
```

The RPC client is also indirectly tested through the [CometBFT integration
tests](../cometbft/tests/integration.rs), which happens during
[CI](../.github/workflows/test.yml). All of these tests require a running
CometBFT node, and are therefore ignored by default. To run these tests
locally:

```bash
# In one terminal, spin up a CometBFT node
docker pull cometbft/cometbft:latest
docker run -it --rm -v "/tmp/cometbft:/cometbft" \
    cometbft/cometbft init
docker run -it --rm -v "/tmp/cometbft:/cometbft" \
    -p 26657:26657 \
    cometbft/cometbft node --proxy_app=kvstore

# In another terminal, run the ignored CometBFT tests to connect to the node
# running at tcp://127.0.0.1:26657
cd ../cometbft
cargo test --all-features -- --ignored
```

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/cometbft-rpc.svg
[crate-link]: https://crates.io/crates/cometbft-rpc
[docs-image]: https://docs.rs/cometbft-rpc/badge.svg
[docs-link]: https://docs.rs/cometbft-rpc/

[//]: # (general links)

[repo root]: https://github.com/cometbft/cometbft-rs
[cometbft]: https://github.com/cometbft/cometbft
[core types]: https://github.com/cometbft/cometbft/blob/8b4a30fada85fccd8f0cb15009344f1cbd8de616/rpc/core/types/responses.go#L1
[cometbft.rs]: https://crates.io/crates/cometbft
[CometBFT RPC]: https://docs.cometbft.com/v1/rpc/
[`/subscribe` endpoint]: https://docs.cometbft.com/v1/rpc/#/Websocket/subscribe
[autogen-fixtures]: https://github.com/informalsystems/tendermint-rs/issues/612
