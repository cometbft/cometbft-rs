use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::abci::{Code, Event};
use crate::prelude::*;
use crate::serializers;

#[doc = include_str!("../doc/response-checktx.md")]
#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct CheckTx {
    /// The response code.
    ///
    /// Transactions where `code != 0` will be rejected; these transactions will
    /// not be broadcast to other nodes or included in a proposal block.
    /// CometBFT attributes no other value to the response code.
    pub code: Code,
    /// Result bytes, if any.
    #[serde(with = "serializers::nullable")]
    pub data: Bytes,
    /// The output of the application's logger.
    ///
    /// **May be non-deterministic**.
    pub log: String,
    /// Additional information.
    ///
    /// **May be non-deterministic**.
    pub info: String,
    /// Amount of gas requested for the transaction.
    #[serde(with = "serializers::from_str")]
    pub gas_wanted: i64,
    /// Amount of gas consumed by the transaction.
    #[serde(with = "serializers::from_str")]
    pub gas_used: i64,
    /// Events that occurred while checking the transaction.
    pub events: Vec<Event>,
    /// The namespace for the `code`.
    pub codespace: String,
    /// The transactions's sender. Not used since CometBFT 0.38.
    #[serde(default)]
    pub sender: String,
    /// Priority for the mempool. Not used since CometBFT 0.38.
    #[serde(default)]
    #[serde(with = "serializers::from_str")]
    pub priority: i64,
    /// Error reported for the mempool. Not used since CometBFT 0.38.
    #[serde(default)]
    pub mempool_error: String,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1beta1 {
    use super::CheckTx;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<CheckTx> for pb::ResponseCheckTx {
        fn from(check_tx: CheckTx) -> Self {
            Self {
                code: check_tx.code.into(),
                data: check_tx.data,
                log: check_tx.log,
                info: check_tx.info,
                gas_wanted: check_tx.gas_wanted,
                gas_used: check_tx.gas_used,
                events: check_tx.events.into_iter().map(Into::into).collect(),
                codespace: check_tx.codespace,
                sender: check_tx.sender,
                priority: check_tx.priority,
                mempool_error: check_tx.mempool_error,
            }
        }
    }

    impl TryFrom<pb::ResponseCheckTx> for CheckTx {
        type Error = crate::Error;

        fn try_from(check_tx: pb::ResponseCheckTx) -> Result<Self, Self::Error> {
            Ok(Self {
                code: check_tx.code.into(),
                data: check_tx.data,
                log: check_tx.log,
                info: check_tx.info,
                gas_wanted: check_tx.gas_wanted,
                gas_used: check_tx.gas_used,
                events: check_tx
                    .events
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                codespace: check_tx.codespace,
                sender: check_tx.sender,
                priority: check_tx.priority,
                mempool_error: check_tx.mempool_error,
            })
        }
    }

    impl Protobuf<pb::ResponseCheckTx> for CheckTx {}
}

mod v1beta2 {
    use super::CheckTx;
    use cometbft_proto::abci::v1beta2 as pb;
    use cometbft_proto::Protobuf;

    impl From<CheckTx> for pb::ResponseCheckTx {
        fn from(check_tx: CheckTx) -> Self {
            Self {
                code: check_tx.code.into(),
                data: check_tx.data,
                log: check_tx.log,
                info: check_tx.info,
                gas_wanted: check_tx.gas_wanted,
                gas_used: check_tx.gas_used,
                events: check_tx.events.into_iter().map(Into::into).collect(),
                codespace: check_tx.codespace,
                sender: check_tx.sender,
                priority: check_tx.priority,
                mempool_error: check_tx.mempool_error,
            }
        }
    }

    impl TryFrom<pb::ResponseCheckTx> for CheckTx {
        type Error = crate::Error;

        fn try_from(check_tx: pb::ResponseCheckTx) -> Result<Self, Self::Error> {
            Ok(Self {
                code: check_tx.code.into(),
                data: check_tx.data,
                log: check_tx.log,
                info: check_tx.info,
                gas_wanted: check_tx.gas_wanted,
                gas_used: check_tx.gas_used,
                events: check_tx
                    .events
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                codespace: check_tx.codespace,
                sender: check_tx.sender,
                priority: check_tx.priority,
                mempool_error: check_tx.mempool_error,
            })
        }
    }

    impl Protobuf<pb::ResponseCheckTx> for CheckTx {}
}

mod v1beta3 {
    use super::CheckTx;
    use cometbft_proto::abci::v1beta3 as pb;
    use cometbft_proto::Protobuf;

    impl From<CheckTx> for pb::ResponseCheckTx {
        fn from(check_tx: CheckTx) -> Self {
            Self {
                code: check_tx.code.into(),
                data: check_tx.data,
                log: check_tx.log,
                info: check_tx.info,
                gas_wanted: check_tx.gas_wanted,
                gas_used: check_tx.gas_used,
                events: check_tx.events.into_iter().map(Into::into).collect(),
                codespace: check_tx.codespace,
            }
        }
    }

    impl TryFrom<pb::ResponseCheckTx> for CheckTx {
        type Error = crate::Error;

        fn try_from(check_tx: pb::ResponseCheckTx) -> Result<Self, Self::Error> {
            Ok(Self {
                code: check_tx.code.into(),
                data: check_tx.data,
                log: check_tx.log,
                info: check_tx.info,
                gas_wanted: check_tx.gas_wanted,
                gas_used: check_tx.gas_used,
                events: check_tx
                    .events
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                codespace: check_tx.codespace,
                sender: Default::default(),
                priority: Default::default(),
                mempool_error: Default::default(),
            })
        }
    }

    impl Protobuf<pb::ResponseCheckTx> for CheckTx {}
}

mod v1 {
    use super::CheckTx;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<CheckTx> for pb::CheckTxResponse {
        fn from(check_tx: CheckTx) -> Self {
            Self {
                code: check_tx.code.into(),
                data: check_tx.data,
                log: check_tx.log,
                info: check_tx.info,
                gas_wanted: check_tx.gas_wanted,
                gas_used: check_tx.gas_used,
                events: check_tx.events.into_iter().map(Into::into).collect(),
                codespace: check_tx.codespace,
            }
        }
    }

    impl TryFrom<pb::CheckTxResponse> for CheckTx {
        type Error = crate::Error;

        fn try_from(check_tx: pb::CheckTxResponse) -> Result<Self, Self::Error> {
            Ok(Self {
                code: check_tx.code.into(),
                data: check_tx.data,
                log: check_tx.log,
                info: check_tx.info,
                gas_wanted: check_tx.gas_wanted,
                gas_used: check_tx.gas_used,
                events: check_tx
                    .events
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                codespace: check_tx.codespace,
                sender: Default::default(),
                priority: Default::default(),
                mempool_error: Default::default(),
            })
        }
    }

    impl Protobuf<pb::CheckTxResponse> for CheckTx {}
}
