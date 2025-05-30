//! Commits to a CometBFT blockchain

use cometbft_proto::types::v1::Commit as RawCommit;
use serde::{Deserialize, Serialize};

use crate::{
    block::{commit_sig::CommitSig, Height, Id, Round},
    prelude::*,
};

/// Commit contains the justification (ie. a set of signatures) that a block was committed by a set
/// of validators.
/// TODO: Update links below!
/// <https://github.com/cometbft/cometbft/blob/51dc810d041eaac78320adc6d53ad8b160b06601/types/block.go#L486-L502>
/// <https://github.com/tendermint/spec/blob/d46cd7f573a2c6a2399fcab2cde981330aa63f37/spec/core/data_structures.md#lastcommit>
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(try_from = "RawCommit", into = "RawCommit")] // Used by testgen Generator trait
pub struct Commit {
    /// Block height
    pub height: Height,

    /// Round
    pub round: Round,

    /// Block ID
    pub block_id: Id,

    /// Signatures
    pub signatures: Vec<CommitSig>,
}

cometbft_old_pb_modules! {
    use super::Commit;
    use crate::{block::commit_sig::CommitSig, error::Error, prelude::*};
    use pb::types::Commit as RawCommit;

    impl TryFrom<RawCommit> for Commit {
        type Error = Error;

        fn try_from(value: RawCommit) -> Result<Self, Self::Error> {
            let signatures: Result<Vec<CommitSig>, Error> = value
                .signatures
                .into_iter()
                .map(TryFrom::try_from)
                .collect();
            Ok(Self {
                height: value.height.try_into()?,
                round: value.round.try_into()?,
                block_id: value
                    .block_id
                    .ok_or_else(|| Error::invalid_block("missing block id".to_string()))?
                    .try_into()?, // gogoproto.nullable = false
                signatures: signatures?,
            })
        }
    }

    impl From<Commit> for RawCommit {
        fn from(value: Commit) -> Self {
            RawCommit {
                height: value.height.into(),
                round: value.round.into(),
                block_id: Some(value.block_id.into()),
                signatures: value.signatures.into_iter().map(Into::into).collect(),
            }
        }
    }
}

mod v1 {
    use super::Commit;
    use crate::{block::commit_sig::CommitSig, error::Error, prelude::*};
    use cometbft_proto::types::v1 as pb;

    impl TryFrom<pb::Commit> for Commit {
        type Error = Error;

        fn try_from(value: pb::Commit) -> Result<Self, Self::Error> {
            let signatures: Result<Vec<CommitSig>, Error> = value
                .signatures
                .into_iter()
                .map(TryFrom::try_from)
                .collect();
            Ok(Self {
                height: value.height.try_into()?,
                round: value.round.try_into()?,
                block_id: value
                    .block_id
                    .ok_or_else(|| Error::invalid_block("missing block id".to_string()))?
                    .try_into()?, // gogoproto.nullable = false
                signatures: signatures?,
            })
        }
    }

    impl From<Commit> for pb::Commit {
        fn from(value: Commit) -> Self {
            pb::Commit {
                height: value.height.into(),
                round: value.round.into(),
                block_id: Some(value.block_id.into()),
                signatures: value.signatures.into_iter().map(Into::into).collect(),
            }
        }
    }
}

mod v1beta1 {
    use super::Commit;
    use crate::{block::commit_sig::CommitSig, error::Error, prelude::*};
    use cometbft_proto::types::v1beta1 as pb;

    impl TryFrom<pb::Commit> for Commit {
        type Error = Error;

        fn try_from(value: pb::Commit) -> Result<Self, Self::Error> {
            let signatures: Result<Vec<CommitSig>, Error> = value
                .signatures
                .into_iter()
                .map(TryFrom::try_from)
                .collect();
            Ok(Self {
                height: value.height.try_into()?,
                round: value.round.try_into()?,
                block_id: value
                    .block_id
                    .ok_or_else(|| Error::invalid_block("missing block id".to_string()))?
                    .try_into()?, // gogoproto.nullable = false
                signatures: signatures?,
            })
        }
    }

    impl From<Commit> for pb::Commit {
        fn from(value: Commit) -> Self {
            pb::Commit {
                height: value.height.into(),
                round: value.round.into(),
                block_id: Some(value.block_id.into()),
                signatures: value.signatures.into_iter().map(Into::into).collect(),
            }
        }
    }
}

impl Default for Commit {
    fn default() -> Self {
        Commit {
            // The default Height is 1, but the default commit is an empty commit with height = 0.
            height: Height::from(0_u32),
            round: Default::default(),
            block_id: Default::default(),
            signatures: vec![],
        }
    }
}
