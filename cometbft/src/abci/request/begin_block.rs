// bring into scope for doc links
#[allow(unused)]
use super::DeliverTx;
use crate::{
    abci::types::{CommitInfo, Misbehavior},
    block,
    prelude::*,
    Hash,
};

#[doc = include_str!("../doc/request-beginblock.md")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BeginBlock {
    /// The block's hash.
    ///
    /// This can be derived from the block header.
    pub hash: Hash,
    /// The block header.
    pub header: block::Header,
    /// Information about the last commit.
    ///
    /// This includes the round, the list of validators, and which validators
    /// signed the last block.
    pub last_commit_info: CommitInfo,
    /// Evidence of validator misbehavior.
    pub byzantine_validators: Vec<Misbehavior>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1beta1 {
    use super::BeginBlock;
    use crate::Error;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<BeginBlock> for pb::RequestBeginBlock {
        fn from(begin_block: BeginBlock) -> Self {
            Self {
                hash: begin_block.hash.into(),
                header: Some(begin_block.header.into()),
                last_commit_info: Some(begin_block.last_commit_info.into()),
                byzantine_validators: begin_block
                    .byzantine_validators
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            }
        }
    }

    impl TryFrom<pb::RequestBeginBlock> for BeginBlock {
        type Error = Error;

        fn try_from(begin_block: pb::RequestBeginBlock) -> Result<Self, Self::Error> {
            Ok(Self {
                hash: begin_block.hash.try_into()?,
                header: begin_block
                    .header
                    .ok_or_else(Error::missing_header)?
                    .try_into()?,
                last_commit_info: begin_block
                    .last_commit_info
                    .ok_or_else(Error::missing_last_commit_info)?
                    .try_into()?,
                byzantine_validators: begin_block
                    .byzantine_validators
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    impl Protobuf<pb::RequestBeginBlock> for BeginBlock {}
}

mod v1beta2 {
    use super::BeginBlock;
    use crate::Error;
    use cometbft_proto::abci::v1beta2 as pb;
    use cometbft_proto::Protobuf;

    impl From<BeginBlock> for pb::RequestBeginBlock {
        fn from(begin_block: BeginBlock) -> Self {
            Self {
                hash: begin_block.hash.into(),
                header: Some(begin_block.header.into()),
                last_commit_info: Some(begin_block.last_commit_info.into()),
                byzantine_validators: begin_block
                    .byzantine_validators
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            }
        }
    }

    impl TryFrom<pb::RequestBeginBlock> for BeginBlock {
        type Error = Error;

        fn try_from(begin_block: pb::RequestBeginBlock) -> Result<Self, Self::Error> {
            Ok(Self {
                hash: begin_block.hash.try_into()?,
                header: begin_block
                    .header
                    .ok_or_else(Error::missing_header)?
                    .try_into()?,
                last_commit_info: begin_block
                    .last_commit_info
                    .ok_or_else(Error::missing_last_commit_info)?
                    .try_into()?,
                byzantine_validators: begin_block
                    .byzantine_validators
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    impl Protobuf<pb::RequestBeginBlock> for BeginBlock {}
}
