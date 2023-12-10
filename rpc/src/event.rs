//! RPC subscription event-related data structures.

use alloc::collections::BTreeMap as HashMap;

use cometbft::{abci, block, Block};

use crate::{prelude::*, query::EventType};

/// An incoming event produced by a [`Subscription`].
///
/// [`Subscription`]: ../struct.Subscription.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// The query that produced the event.
    pub query: String,
    /// The data associated with the event.
    pub data: EventData,
    /// Event type and attributes map.
    pub events: Option<HashMap<String, Vec<String>>>,
}

impl Event {
    /// Returns the type associated with this event, if we recognize it.
    ///
    /// Returns `None` if we don't yet support this event type.
    pub fn event_type(&self) -> Option<EventType> {
        match self.data {
            EventData::NewBlock { .. } => Some(EventType::NewBlock),
            EventData::Tx { .. } => Some(EventType::Tx),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventData {
    /// Data of the newly committed block.
    ///
    /// Used since CometBFT 0.38.
    NewBlock {
        block: Option<Box<Block>>,
        block_id: block::Id,
        result_finalize_block: Option<abci::response::FinalizeBlock>,
    },
    /// Data of the newly committed block.
    ///
    /// Used in CometBFT versions before 0.38.
    LegacyNewBlock {
        block: Option<Box<Block>>,
        result_begin_block: Option<abci::response::BeginBlock>,
        result_end_block: Option<abci::response::EndBlock>,
    },
    Tx {
        tx_result: TxInfo,
    },
    GenericJsonEvent(serde_json::Value),
}

/// Transaction result info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxInfo {
    pub height: i64,
    pub index: Option<i64>,
    pub tx: Vec<u8>,
    pub result: TxResult,
}

/// Transaction result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxResult {
    pub log: Option<String>,
    pub gas_wanted: Option<String>,
    pub gas_used: Option<String>,
    pub events: Vec<abci::Event>,
}

/// Serialization helpers for CometBFT 0.34 RPC
pub mod v0_34 {
    use super::{Event, EventData, TxInfo, TxResult};
    use crate::dialect::v0_34::Event as RpcEvent;
    use crate::prelude::*;
    use crate::{dialect, serializers, Response};
    use alloc::collections::BTreeMap as HashMap;
    use cometbft::Block;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DialectEvent {
        /// The query that produced the event.
        pub query: String,
        /// The data associated with the event.
        pub data: DialectEventData,
        /// Event type and attributes map.
        pub events: Option<HashMap<String, Vec<String>>>,
    }

    pub type DeEvent = DialectEvent;
    pub type SerEvent = DialectEvent;

    impl Response for DialectEvent {}

    impl From<DialectEvent> for Event {
        fn from(msg: DialectEvent) -> Self {
            Event {
                query: msg.query,
                data: msg.data.into(),
                events: msg.events,
            }
        }
    }

    impl From<Event> for DialectEvent {
        fn from(msg: Event) -> Self {
            DialectEvent {
                query: msg.query,
                data: msg.data.into(),
                events: msg.events,
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "type", content = "value")]
    #[allow(clippy::large_enum_variant)]
    pub enum DialectEventData {
        #[serde(alias = "tendermint/event/NewBlock")]
        NewBlock {
            block: Option<Box<Block>>,
            result_begin_block: Option<dialect::BeginBlock<RpcEvent>>,
            result_end_block: Option<dialect::EndBlock<RpcEvent>>,
        },
        #[serde(alias = "tendermint/event/Tx")]
        Tx {
            #[serde(rename = "TxResult")]
            tx_result: DialectTxInfo,
        },
        GenericJsonEvent(serde_json::Value),
    }

    impl From<DialectEventData> for EventData {
        fn from(msg: DialectEventData) -> Self {
            match msg {
                DialectEventData::NewBlock {
                    block,
                    result_begin_block,
                    result_end_block,
                } => EventData::LegacyNewBlock {
                    block,
                    result_begin_block: result_begin_block.map(Into::into),
                    result_end_block: result_end_block.map(Into::into),
                },
                DialectEventData::Tx { tx_result } => EventData::Tx {
                    tx_result: tx_result.into(),
                },
                DialectEventData::GenericJsonEvent(v) => EventData::GenericJsonEvent(v),
            }
        }
    }

    impl From<EventData> for DialectEventData {
        fn from(msg: EventData) -> Self {
            match msg {
                EventData::LegacyNewBlock {
                    block,
                    result_begin_block,
                    result_end_block,
                } => DialectEventData::NewBlock {
                    block,
                    result_begin_block: result_begin_block.map(Into::into),
                    result_end_block: result_end_block.map(Into::into),
                },
                // This variant should not be used with 0.34, but we're using
                // this impl only for the mock server.
                EventData::NewBlock {
                    block,
                    block_id: _,
                    result_finalize_block: _,
                } => DialectEventData::NewBlock {
                    block,
                    result_begin_block: None,
                    result_end_block: None,
                },
                EventData::Tx { tx_result } => DialectEventData::Tx {
                    tx_result: tx_result.into(),
                },
                EventData::GenericJsonEvent(v) => DialectEventData::GenericJsonEvent(v),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DialectTxInfo {
        #[serde(with = "serializers::from_str")]
        pub height: i64,
        pub index: Option<i64>,
        #[serde(with = "serializers::bytes::base64string")]
        pub tx: Vec<u8>,
        pub result: DialectTxResult,
    }

    impl From<DialectTxInfo> for TxInfo {
        fn from(msg: DialectTxInfo) -> Self {
            TxInfo {
                height: msg.height,
                index: msg.index,
                tx: msg.tx,
                result: msg.result.into(),
            }
        }
    }

    impl From<TxInfo> for DialectTxInfo {
        fn from(msg: TxInfo) -> Self {
            DialectTxInfo {
                height: msg.height,
                index: msg.index,
                tx: msg.tx,
                result: msg.result.into(),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DialectTxResult {
        pub log: Option<String>,
        pub gas_wanted: Option<String>,
        pub gas_used: Option<String>,
        pub events: Vec<RpcEvent>,
    }

    impl From<DialectTxResult> for TxResult {
        fn from(msg: DialectTxResult) -> Self {
            TxResult {
                log: msg.log,
                gas_wanted: msg.gas_wanted,
                gas_used: msg.gas_used,
                events: msg.events.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<TxResult> for DialectTxResult {
        fn from(msg: TxResult) -> Self {
            DialectTxResult {
                log: msg.log,
                gas_wanted: msg.gas_wanted,
                gas_used: msg.gas_used,
                events: msg.events.into_iter().map(Into::into).collect(),
            }
        }
    }
}

/// Shared serialization/deserialization helpers for the RPC protocol used since CometBFT 0.37
mod latest {
    use super::{Event, EventData, TxInfo, TxResult};
    use crate::prelude::*;
    use crate::{serializers, Response};
    use alloc::collections::BTreeMap as HashMap;
    use cometbft::abci::Event as RpcEvent;
    use cometbft::{abci, block, Block};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Debug)]
    pub struct DeEvent {
        /// The query that produced the event.
        pub query: String,
        /// The data associated with the event.
        pub data: DeEventData,
        /// Event type and attributes map.
        pub events: Option<HashMap<String, Vec<String>>>,
    }

    impl Response for DeEvent {}

    impl From<DeEvent> for Event {
        fn from(msg: DeEvent) -> Self {
            Event {
                query: msg.query,
                data: msg.data.into(),
                events: msg.events,
            }
        }
    }

    /// Helper used to deserialize [`EventData`] for CometBFT RPC since 0.37
    #[derive(Deserialize, Debug)]
    #[serde(tag = "type", content = "value")]
    #[allow(clippy::large_enum_variant)]
    pub enum DeEventData {
        #[serde(alias = "tendermint/event/NewBlock")]
        NewBlock {
            block: Option<Box<Block>>,
            #[serde(default)]
            result_begin_block: Option<abci::response::BeginBlock>,
            #[serde(default)]
            result_end_block: Option<abci::response::EndBlock>,
            #[serde(default)]
            block_id: Option<block::Id>,
            #[serde(default)]
            result_finalize_block: Option<abci::response::FinalizeBlock>,
        },
        #[serde(alias = "tendermint/event/Tx")]
        Tx {
            #[serde(rename = "TxResult")]
            tx_result: DialectTxInfo,
        },
        GenericJsonEvent(serde_json::Value),
    }

    impl From<DeEventData> for EventData {
        fn from(msg: DeEventData) -> Self {
            match msg {
                DeEventData::NewBlock {
                    block,
                    block_id: Some(block_id),
                    result_finalize_block,
                    result_begin_block: _,
                    result_end_block: _,
                } => EventData::NewBlock {
                    block,
                    block_id,
                    result_finalize_block,
                },
                DeEventData::NewBlock {
                    block,
                    result_begin_block,
                    result_end_block,
                    block_id: None,
                    result_finalize_block: _,
                } => EventData::LegacyNewBlock {
                    block,
                    result_begin_block: result_begin_block.map(Into::into),
                    result_end_block: result_end_block.map(Into::into),
                },
                DeEventData::Tx { tx_result } => EventData::Tx {
                    tx_result: tx_result.into(),
                },
                DeEventData::GenericJsonEvent(v) => EventData::GenericJsonEvent(v),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DialectTxInfo {
        #[serde(with = "serializers::from_str")]
        pub height: i64,
        pub index: Option<i64>,
        #[serde(with = "serializers::bytes::base64string")]
        pub tx: Vec<u8>,
        pub result: DialectTxResult,
    }

    impl From<DialectTxInfo> for TxInfo {
        fn from(msg: DialectTxInfo) -> Self {
            TxInfo {
                height: msg.height,
                index: msg.index,
                tx: msg.tx,
                result: msg.result.into(),
            }
        }
    }

    impl From<TxInfo> for DialectTxInfo {
        fn from(msg: TxInfo) -> Self {
            DialectTxInfo {
                height: msg.height,
                index: msg.index,
                tx: msg.tx,
                result: msg.result.into(),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DialectTxResult {
        pub log: Option<String>,
        pub gas_wanted: Option<String>,
        pub gas_used: Option<String>,
        pub events: Vec<RpcEvent>,
    }

    impl From<DialectTxResult> for TxResult {
        fn from(msg: DialectTxResult) -> Self {
            TxResult {
                log: msg.log,
                gas_wanted: msg.gas_wanted,
                gas_used: msg.gas_used,
                events: msg.events.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<TxResult> for DialectTxResult {
        fn from(msg: TxResult) -> Self {
            DialectTxResult {
                log: msg.log,
                gas_wanted: msg.gas_wanted,
                gas_used: msg.gas_used,
                events: msg.events.into_iter().map(Into::into).collect(),
            }
        }
    }
}

/// Serialization helpers for the RPC protocol used in CometBFT 0.37
pub mod v0_37 {
    use super::{Event, EventData};
    use crate::prelude::*;
    use alloc::collections::BTreeMap as HashMap;
    use cometbft::{abci, Block};
    use serde::Serialize;

    pub use super::latest::*;

    #[derive(Serialize, Debug)]
    pub struct SerEvent {
        /// The query that produced the event.
        pub query: String,
        /// The data associated with the event.
        pub data: SerEventData,
        /// Event type and attributes map.
        pub events: Option<HashMap<String, Vec<String>>>,
    }

    impl From<Event> for SerEvent {
        fn from(msg: Event) -> Self {
            SerEvent {
                query: msg.query,
                data: msg.data.into(),
                events: msg.events,
            }
        }
    }

    #[derive(Serialize, Debug)]
    #[serde(tag = "type", content = "value")]
    pub enum SerEventData {
        #[serde(alias = "tendermint/event/NewBlock")]
        NewBlock {
            block: Option<Box<Block>>,
            result_begin_block: Option<abci::response::BeginBlock>,
            result_end_block: Option<abci::response::EndBlock>,
        },
        #[serde(alias = "tendermint/event/Tx")]
        Tx {
            #[serde(rename = "TxResult")]
            tx_result: DialectTxInfo,
        },
        GenericJsonEvent(serde_json::Value),
    }

    impl From<EventData> for SerEventData {
        fn from(msg: EventData) -> Self {
            match msg {
                // This variant should not be used in 0.37, but the conversion
                // must be infallible.
                EventData::NewBlock {
                    block,
                    block_id: _,
                    result_finalize_block: _,
                } => SerEventData::NewBlock {
                    block,
                    result_begin_block: None,
                    result_end_block: None,
                },
                EventData::LegacyNewBlock {
                    block,
                    result_begin_block,
                    result_end_block,
                } => SerEventData::NewBlock {
                    block,
                    result_begin_block: result_begin_block.map(Into::into),
                    result_end_block: result_end_block.map(Into::into),
                },
                EventData::Tx { tx_result } => SerEventData::Tx {
                    tx_result: tx_result.into(),
                },
                EventData::GenericJsonEvent(v) => SerEventData::GenericJsonEvent(v),
            }
        }
    }
}

/// Serialization helpers for the RPC protocol used in CometBFT 0.38
pub mod v0_38 {
    use super::{Event, EventData};
    use crate::prelude::*;
    use alloc::collections::BTreeMap as HashMap;
    use cometbft::{abci, block, Block};
    use serde::Serialize;

    pub use super::latest::*;

    #[derive(Serialize, Debug)]
    pub struct SerEvent {
        /// The query that produced the event.
        pub query: String,
        /// The data associated with the event.
        pub data: SerEventData,
        /// Event type and attributes map.
        pub events: Option<HashMap<String, Vec<String>>>,
    }

    impl From<Event> for SerEvent {
        fn from(msg: Event) -> Self {
            SerEvent {
                query: msg.query,
                data: msg.data.into(),
                events: msg.events,
            }
        }
    }

    #[derive(Serialize, Debug)]
    #[serde(tag = "type", content = "value")]
    pub enum SerEventData {
        #[serde(alias = "tendermint/event/NewBlock")]
        NewBlock {
            block: Option<Box<Block>>,
            block_id: block::Id,
            result_finalize_block: Option<abci::response::FinalizeBlock>,
        },
        #[serde(alias = "tendermint/event/Tx")]
        Tx {
            #[serde(rename = "TxResult")]
            tx_result: DialectTxInfo,
        },
        GenericJsonEvent(serde_json::Value),
    }

    impl From<EventData> for SerEventData {
        fn from(msg: EventData) -> Self {
            match msg {
                EventData::NewBlock {
                    block,
                    block_id,
                    result_finalize_block,
                } => SerEventData::NewBlock {
                    block,
                    block_id,
                    result_finalize_block,
                },
                // This variant should not be used in 0.38, but the conversion
                // must be infallible.
                EventData::LegacyNewBlock {
                    block,
                    result_begin_block: _,
                    result_end_block: _,
                } => SerEventData::NewBlock {
                    block,
                    block_id: Default::default(),
                    result_finalize_block: None,
                },
                EventData::Tx { tx_result } => SerEventData::Tx {
                    tx_result: tx_result.into(),
                },
                EventData::GenericJsonEvent(v) => SerEventData::GenericJsonEvent(v),
            }
        }
    }
}
