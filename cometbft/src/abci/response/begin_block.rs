use serde::{Deserialize, Serialize};

use crate::{abci::Event, prelude::*};

#[doc = include_str!("../doc/response-beginblock.md")]
#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct BeginBlock {
    /// Events that occurred while beginning the block.
    #[serde(default)]
    pub events: Vec<Event>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1beta1 {
    use super::BeginBlock;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<BeginBlock> for pb::ResponseBeginBlock {
        fn from(begin_block: BeginBlock) -> Self {
            Self {
                events: begin_block.events.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl TryFrom<pb::ResponseBeginBlock> for BeginBlock {
        type Error = crate::Error;

        fn try_from(begin_block: pb::ResponseBeginBlock) -> Result<Self, Self::Error> {
            Ok(Self {
                events: begin_block
                    .events
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    impl Protobuf<pb::ResponseBeginBlock> for BeginBlock {}
}

mod v1beta2 {
    use super::BeginBlock;
    use cometbft_proto::abci::v1beta2 as pb;
    use cometbft_proto::Protobuf;

    impl From<BeginBlock> for pb::ResponseBeginBlock {
        fn from(begin_block: BeginBlock) -> Self {
            Self {
                events: begin_block.events.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl TryFrom<pb::ResponseBeginBlock> for BeginBlock {
        type Error = crate::Error;

        fn try_from(begin_block: pb::ResponseBeginBlock) -> Result<Self, Self::Error> {
            Ok(Self {
                events: begin_block
                    .events
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    impl Protobuf<pb::ResponseBeginBlock> for BeginBlock {}
}
