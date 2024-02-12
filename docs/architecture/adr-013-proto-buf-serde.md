# ADR 013: Change proto generation tool to buf

## Changelog

* 2024-02-14: Initial draft

## Context

Currently, Rust binding code for CometBFT proto files, which makes up most of
the `cometbft-proto` crate, is generated with a custom tool found in `tools/proto-compiler`, working on a Git checkout of the [cometbft][cometbft-git] repository where the proto files are maintained.
Instances of [protobuf well-known types][wkt] are represented with custom types
defined in the crate, due to historical problems with `prost-types`.

Additional trait implementations are derived for some of the generated
message structures, mainly to provide JSON serialization for RPC and possibly
other non-Protobuf uses. This is done with custom attributes added through code
generator options.
While separating the unrelated concerns by e.g. implementing the traits directly on
the domain types in the `cometbft` crate would be cleaner architecturally, doing so
on the protoc-generated types is convenient because the protobuf and RPC
implementations in the CometBFT Go codebase reuse the same data structures and therefore,
the serialization schema. The different versions of protobuf specifications
across past CometBFT releases are only fully exposed in `cometbft-proto`
and some library users might prefer to work at this level and eschew our domain types,
so it seems prudent to maintain version-accurate serialization as a secondary
feature of the protobuf bindings.

Rust projects which reuse protocol types from CometBFT and generate code from
proto files that include `tendermint.*` or `cometbft.*` proto packages as a
dependency currently use the same approach, with some local arrangements about
mapping the WKTs and partial reuse of the serde implementations.
This results in [problems][ibc-proto-rs#194] in dependency management that
are easy to overlook, and duplication of types representing the protobuf WKTs.

[cometbft-git]: https://github.com/cometbft/cometbft
[wkt]: https://protobuf.dev/reference/protobuf/google.protobuf/
[ibc-proto-rs#194]: https://github.com/cosmos/ibc-proto-rs/issues/194

## Decision

Switch to [buf](https://buf.build/) for generating `cometbft-proto` Rust
bindings for the CometBFT protos that are published to the Buf Schema Registry
under the [`cometbft/cometbft`](https://buf.build/cometbft/cometbft) module.

Attempt to make use of the following plugins:

* [protoc-gen-prost] to generate the core data structures.
* [protoc-gen-prost-serde] (or our fork targeting `informalsystems-pbjson`)
  to generate the bulk of the serde implementations for the same data structures.
* [protoc-gen-tonic] for gRPC services.

[protoc-gen-prost]: https://github.com/neoeinstein/protoc-gen-prost/blob/main/protoc-gen-prost/README.md
[protoc-gen-prost-serde]: https://github.com/neoeinstein/protoc-gen-prost/blob/main/protoc-gen-prost-serde/README.md
[protoc-gen-tonic]: https://github.com/neoeinstein/protoc-gen-prost/blob/main/protoc-gen-tonic/README.md

If the plugins are found lacking necessary features, try an [alternative approach][alt-gen]
with `buf build --as-file-descriptor-set` and feeding the output to `tonic-build`. The serde
implementations would need to be custom-derived like with the current approach.

[alt-gen]: https://github.com/cometbft/cometbft-rs/issues/3#issuecomment-1916766090

### API changes for protobuf/gRPC users

The prost and tonic plugins invoke `prost-build` and `tonic-build`, respectively,
so the code produced by them should not principally differ from the output of the
current proto-compiler tool.

### Changes in serialization

The prost-serde plugin produces code depending on `pbjson`. This is
an implementation detail not affecting the public API of the generated code,
which exposes the regular serde trait implementations. The serialization schema
produced by `pbjson-build` is compliant with the [JSON mapping for Protobuf][proto-json],
to which the marshalling used by `encoding/json` in Go is mostly equivalent;
notably, though, the JSON serialization currently used in CometBFT is implemented
differently.

[proto-json]: https://protobuf.dev/programming-guides/proto3/#json

Our attempt to use the `protoc-gen-prost-serde` plugin will aim to provide serialization
for most of the types generated from proto files; exceptions will be treated with
setting custom attributes to `prost-build` via the buf plugin, much like in the
current approach. A current limitation is that the `exclude` option of
`pbjson-build`, needed to enable such overrides, is [not yet supported][protoc-gen-prost#84]
by the buf plugin.

[protoc-gen-prost#84]: https://github.com/neoeinstein/protoc-gen-prost/issues/84

Another obstacle is that the `pbjson` crate does not currently support `no_std`.
This is needed for `ibc-proto` bindings if they are to reuse types from `cometbft-proto`.
The developers of ibc-rs have already [forked][informalsystems-pbjson] `pbjson`, so
a short-term fix would be to fork the buf plugin as well to make the generated code
use the forked `informalsystem-pbjson` crate (plus, we get to support the `exclude`
option at our own pace this way).
In the long term, we should try to upstream the `no_std` changes to `pbjson`
and eliminate the forks.

[informalsystems-pbjson]: https://github.com/informalsystems/pbjson

### Well-known types

Where the CometBFT proto files make use of the well-known message types from
the `google.protobuf` package, equivalent Rust types need to be substituted
in the code generated for `cometbft-proto`. Developers of downstream crates
may be interested in converging on the same types to avoid duplication.
The CometBFT protobufs only use the `Duration` and `Timestamp` WKTs; protobufs
in IBC, Cosmos-SDK, Interchain Security, and NFT transfer, also use `Any`.

Three alternative WKT mapping approaches have been used across the protocol stack:

1. [prost-types]. This is the most common mapping used by code generated with `prost-build`.
   The crate does not provide serde implementations for the WKTs, so any message structs
   including fields of WKTs either need overrides for their serde implementation or must omit
   serde altogether.
2. [pbjson-types]. The serde implementations provided by this WKT mapping crate is compliant
   with the standard [protobuf JSON mapping][proto-json]. However, the JSON schema used by
   CometBFT does not use the same format for `Duration`, so the crate cannot be substituted
   for all uses. Note that the decision on whether to map WKTs to `pbjson-types` is orthogonal to the use of the `protoc-gen-prost-serde` buf plugin for the protocol messages;
   the build script can opt to use either or both.
3. Roll-our-own types defined in `cometbft-proto` (the current approach).
   These types provide serialization impls interoperable with CometBFT RPC out of the box.
   The custom type mapping creates obstacles for reuse with other crates, which is the entire
   purpose of the protobuf WKTs. This can be bridged by providing `From` conversions
   from and to the workalikes in the other WKT mapping crates.

As long as RPC-compatible serialization is seen as an important feature, the approach 3 appears
to be the least problematic option. Protobuf generation scripts for crates up the Cosmos stack can
choose to map `google.protobuf.Timestamp` and `google.protobuf.Duration` to these public types
as well.

[prost-types]: https://crates.io/crates/prost-types
[pbjson-types]: https://crates.io/crates/pbjson-types

## Status

Proposed

## Consequences

### Positive

By switching to Buf for generating code in cometbft-rs and downstream,
we will use a consistent tool set and share the benefits of its ecosystem,
such as lints, plugins and best practices of dependency management.

Going forward, more developers working with protobuf are expected to be familiar
with the Buf tool and its associated schema registry.

### Negative

The changes to representation of the well-known data types will break the established API.

## References

* [cometbft-rs#3](https://github.com/cometbft/cometbft-rs/issues/3) (tracking issue with some discussion).
