//! Application BlockChain Interface ([ABCI]) is the interface between CometBFT
//! (a consensus engine for Byzantine-fault-tolerant replication of a state
//! machine) and an application (the state machine to be replicated).
//!
//! Using ABCI involves writing an application driven by ABCI methods, exposing
//! that application as an ABCI server, and having CometBFT connect to the
//! server as an ABCI client.
//!
//! This module does not include an ABCI server implementation itself. Instead,
//! it provides a common set of Rust domain types that model the ABCI protocol,
//! which can be used by both ABCI applications and ABCI server implementations.
//!
//! One ABCI server implementation is provided by the [`cometbft_abci`][tmabci]
//! crate.
//!
//! Each ABCI method corresponds to a request/response pair. ABCI requests are
//! modeled by the [`Request`] enum, and responses are modeled by the
//! [`Response`] enum.  As described in the [methods and types][mat] page, ABCI
//! methods are split into four categories. CometBFT opens one ABCI connection
//! for each category of messages. These categories are modeled by the
//! [`MethodKind`] enum and by per-category request and response enums:
//!
//! * [`ConsensusRequest`] /  [`ConsensusResponse`] for [`MethodKind::Consensus`] methods;
//! * [`MempoolRequest`] /  [`MempoolResponse`] for [`MethodKind::Mempool`] methods;
//! * [`InfoRequest`] /  [`InfoResponse`] for [`MethodKind::Info`] methods;
//! * [`SnapshotRequest`] /  [`SnapshotResponse`] for [`MethodKind::Snapshot`] methods.
//!
//! The domain types in this module have conversions to and from the Protobuf
//! types defined in the [`cometbft_proto`] crate. These conversions are
//! required for ABCI server implementations, which use the protobufs to
//! communicate with CometBFT, but should not be required for ABCI
//! applications, which should use the domain types in an interface defined by
//! their choice of ABCI server implementation.
//!
//! [ABCI]: https://docs.cometbft.com/v1.0/spec/abci/
//! [mat]: https://docs.cometbft.com/v1.0/spec/abci/abci.html
//! [tmabci]: https://github.com/cometbft/cometbft-rs/tree/master/abci

mod code;
mod event;
mod kind;

pub mod request;
pub mod response;
pub mod types;

pub mod v0_34;
pub mod v1;
pub mod v1beta1;
pub mod v1beta2;
pub mod v1beta3;

pub use v1::request::Request;
pub use v1::request::{ConsensusRequest, InfoRequest, MempoolRequest, SnapshotRequest};
pub use v1::response::Response;
pub use v1::response::{ConsensusResponse, InfoResponse, MempoolResponse, SnapshotResponse};

pub use event::{Event, EventAttribute, EventAttributeIndexExt, TypedEvent};

#[doc(inline)]
pub use self::{code::Code, kind::MethodKind};
