use cometbft::evidence;
use cometbft_proto::types::v1 as pb;

use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// The Event serialization in the latest RPC dialect is the canonical
/// serialization for the ABCI domain type.
pub use cometbft::abci::Event;

#[derive(Default, Clone)]
pub struct Dialect;

impl crate::dialect::Dialect for Dialect {
    type Event = Event;
    type Evidence = Evidence;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(into = "pb::Evidence", try_from = "pb::Evidence")]
pub struct Evidence(evidence::Evidence);

impl From<Evidence> for pb::Evidence {
    fn from(evidence: Evidence) -> Self {
        evidence.0.into()
    }
}

impl TryFrom<pb::Evidence> for Evidence {
    type Error = <evidence::Evidence as TryFrom<pb::Evidence>>::Error;

    fn try_from(value: pb::Evidence) -> Result<Self, Self::Error> {
        Ok(Self(evidence::Evidence::try_from(value)?))
    }
}

impl From<evidence::Evidence> for Evidence {
    fn from(evidence: evidence::Evidence) -> Self {
        Self(evidence)
    }
}
