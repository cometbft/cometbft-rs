//! `/status` endpoint JSON-RPC wrapper

use cometbft::{block, node, validator, AppHash, Hash, Time};
use serde::{Deserialize, Serialize};

use crate::{dialect::Dialect, request::RequestMessage};

/// Node status request
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request;

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::Status
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// Status responses
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// Node information
    pub node_info: node::Info,

    /// Sync information
    pub sync_info: SyncInfo,

    /// Validator information
    pub validator_info: validator::Info,
}

impl crate::Response for Response {}

/// Sync information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncInfo {
    /// Earliest block hash
    #[serde(with = "cometbft::serializers::hash")]
    pub earliest_block_hash: Hash,

    /// Earliest app hash
    #[serde(with = "cometbft::serializers::apphash")]
    pub earliest_app_hash: AppHash,

    /// Earliest block height
    pub earliest_block_height: block::Height,

    /// Earliest block time
    pub earliest_block_time: Time,

    /// Latest block hash
    #[serde(with = "cometbft::serializers::hash")]
    pub latest_block_hash: Hash,

    /// Latest app hash
    #[serde(with = "cometbft::serializers::apphash")]
    pub latest_app_hash: AppHash,

    /// Latest block height
    pub latest_block_height: block::Height,

    /// Latest block time
    pub latest_block_time: Time,

    /// Are we catching up?
    pub catching_up: bool,
}
