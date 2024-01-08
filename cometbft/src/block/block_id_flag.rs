/// Indicates whether the validator voted for a block, nil, or did not vote at all
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BlockIdFlag {
    /// The vote was not received.
    Absent,
    /// Voted for a block.
    Commit,
    /// Voted for nil.
    Nil,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1 {
    use super::BlockIdFlag;
    use crate::{error::Error, prelude::*};
    use cometbft_proto::types::v1 as pb;

    impl TryFrom<pb::BlockIdFlag> for BlockIdFlag {
        type Error = Error;

        fn try_from(value: pb::BlockIdFlag) -> Result<Self, Self::Error> {
            match value {
                pb::BlockIdFlag::Absent => Ok(BlockIdFlag::Absent),
                pb::BlockIdFlag::Commit => Ok(BlockIdFlag::Commit),
                pb::BlockIdFlag::Nil => Ok(BlockIdFlag::Nil),
                _ => Err(Error::block_id_flag()),
            }
        }
    }

    impl From<BlockIdFlag> for pb::BlockIdFlag {
        fn from(value: BlockIdFlag) -> pb::BlockIdFlag {
            match value {
                BlockIdFlag::Absent => pb::BlockIdFlag::Absent,
                BlockIdFlag::Commit => pb::BlockIdFlag::Commit,
                BlockIdFlag::Nil => pb::BlockIdFlag::Nil,
            }
        }
    }
}

mod v1beta1 {
    use super::BlockIdFlag;
    use crate::{error::Error, prelude::*};
    use cometbft_proto::types::v1beta1 as pb;

    impl TryFrom<pb::BlockIdFlag> for BlockIdFlag {
        type Error = Error;

        fn try_from(value: pb::BlockIdFlag) -> Result<Self, Self::Error> {
            match value {
                pb::BlockIdFlag::Absent => Ok(BlockIdFlag::Absent),
                pb::BlockIdFlag::Commit => Ok(BlockIdFlag::Commit),
                pb::BlockIdFlag::Nil => Ok(BlockIdFlag::Nil),
                _ => Err(Error::block_id_flag()),
            }
        }
    }

    impl From<BlockIdFlag> for pb::BlockIdFlag {
        fn from(value: BlockIdFlag) -> pb::BlockIdFlag {
            match value {
                BlockIdFlag::Absent => pb::BlockIdFlag::Absent,
                BlockIdFlag::Commit => pb::BlockIdFlag::Commit,
                BlockIdFlag::Nil => pb::BlockIdFlag::Nil,
            }
        }
    }
}
