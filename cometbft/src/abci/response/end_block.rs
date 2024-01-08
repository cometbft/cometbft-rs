use serde::{Deserialize, Serialize};

use crate::{abci::Event, consensus, prelude::*, serializers, validator};

#[doc = include_str!("../doc/response-endblock.md")]
#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct EndBlock {
    /// Changes to the validator set, if any.
    ///
    /// Setting the voting power to 0 removes a validator.
    #[serde(with = "serializers::nullable")]
    pub validator_updates: Vec<validator::Update>,
    /// Changes to consensus parameters (optional).
    pub consensus_param_updates: Option<consensus::Params>,
    /// Events that occurred while ending the block.
    #[serde(default)]
    pub events: Vec<Event>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1beta1 {
    use super::EndBlock;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<EndBlock> for pb::ResponseEndBlock {
        fn from(end_block: EndBlock) -> Self {
            Self {
                validator_updates: end_block
                    .validator_updates
                    .into_iter()
                    .map(Into::into)
                    .collect(),
                consensus_param_updates: end_block.consensus_param_updates.map(Into::into),
                events: end_block.events.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl TryFrom<pb::ResponseEndBlock> for EndBlock {
        type Error = crate::Error;

        fn try_from(end_block: pb::ResponseEndBlock) -> Result<Self, Self::Error> {
            Ok(Self {
                validator_updates: end_block
                    .validator_updates
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                consensus_param_updates: end_block
                    .consensus_param_updates
                    .map(TryInto::try_into)
                    .transpose()?,
                events: end_block
                    .events
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    impl Protobuf<pb::ResponseEndBlock> for EndBlock {}
}

mod v1beta2 {
    use super::EndBlock;
    use cometbft_proto::abci::v1beta2 as pb;
    use cometbft_proto::Protobuf;

    impl From<EndBlock> for pb::ResponseEndBlock {
        fn from(end_block: EndBlock) -> Self {
            Self {
                validator_updates: end_block
                    .validator_updates
                    .into_iter()
                    .map(Into::into)
                    .collect(),
                consensus_param_updates: end_block.consensus_param_updates.map(Into::into),
                events: end_block.events.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl TryFrom<pb::ResponseEndBlock> for EndBlock {
        type Error = crate::Error;

        fn try_from(end_block: pb::ResponseEndBlock) -> Result<Self, Self::Error> {
            Ok(Self {
                validator_updates: end_block
                    .validator_updates
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                consensus_param_updates: end_block
                    .consensus_param_updates
                    .map(TryInto::try_into)
                    .transpose()?,
                events: end_block
                    .events
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    impl Protobuf<pb::ResponseEndBlock> for EndBlock {}
}
