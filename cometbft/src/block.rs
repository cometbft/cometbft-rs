//! Blocks within the chains of a CometBFT network

mod block_id_flag;
mod commit;
pub mod commit_sig;
pub mod header;
mod height;
mod id;
mod meta;
pub mod parts;
mod round;
pub mod signed_header;
mod size;

use cometbft_proto::types::v1::Block as RawBlock;
use serde::{Deserialize, Serialize};

pub use self::{
    block_id_flag::BlockIdFlag,
    commit::*,
    commit_sig::*,
    header::Header,
    height::*,
    id::{Id, ParseId},
    meta::Meta,
    round::*,
    size::Size,
};
use crate::{evidence, prelude::*};

/// Blocks consist of a header, transactions, votes (the commit), and a list of
/// evidence of malfeasance (i.e. signing conflicting votes).
///
/// <https://github.com/cometbft/cometbft/blob/v1.0.0-alpha.1/spec/core/data_structures.md#block>
// Default serialization - all fields serialize; used by /block endpoint
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[serde(try_from = "RawBlock", into = "RawBlock")]
pub struct Block {
    /// Block header
    pub header: Header,

    /// Transaction data
    pub data: Vec<Vec<u8>>,

    /// Evidence of malfeasance
    pub evidence: evidence::List,

    /// Last commit, should be `None` for the initial block.
    pub last_commit: Option<Commit>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

cometbft_old_pb_modules! {
    use super::{Block, Header, Commit};
    use crate::{Error, prelude::*};
    use pb::types::Block as RawBlock;

    impl Protobuf<RawBlock> for Block {}

    impl TryFrom<RawBlock> for Block {
        type Error = Error;

        fn try_from(value: RawBlock) -> Result<Self, Self::Error> {
            let header: Header = value.header.ok_or_else(Error::missing_header)?.try_into()?;
            // If last_commit is the default Commit, it is considered nil by Go.
            let last_commit = value
                .last_commit
                .map(TryInto::try_into)
                .transpose()?
                .filter(|c| c != &Commit::default());

            Ok(Block::new(
                header,
                value.data.ok_or_else(Error::missing_data)?.txs,
                value
                    .evidence
                    .map(TryInto::try_into)
                    .transpose()?
                    .unwrap_or_default(),
                last_commit,
            ))
        }
    }

    impl From<Block> for RawBlock {
        fn from(value: Block) -> Self {
            use pb::types::Data as RawData;
            RawBlock {
                header: Some(value.header.into()),
                data: Some(RawData { txs: value.data }),
                evidence: Some(value.evidence.into()),
                last_commit: value.last_commit.map(Into::into),
            }
        }
    }
}

mod v1 {
    use super::{Block, Commit, Header};
    use crate::{prelude::*, Error};
    use cometbft_proto::types::v1 as pb;
    use cometbft_proto::Protobuf;

    impl Protobuf<pb::Block> for Block {}

    impl TryFrom<pb::Block> for Block {
        type Error = Error;

        fn try_from(value: pb::Block) -> Result<Self, Self::Error> {
            let header: Header = value.header.ok_or_else(Error::missing_header)?.try_into()?;
            // If last_commit is the default Commit, it is considered nil by Go.
            let last_commit = value
                .last_commit
                .map(TryInto::try_into)
                .transpose()?
                .filter(|c| c != &Commit::default());

            Ok(Block::new(
                header,
                value.data.ok_or_else(Error::missing_data)?.txs,
                value
                    .evidence
                    .map(TryInto::try_into)
                    .transpose()?
                    .unwrap_or_default(),
                last_commit,
            ))
        }
    }

    impl From<Block> for pb::Block {
        fn from(value: Block) -> Self {
            pb::Block {
                header: Some(value.header.into()),
                data: Some(pb::Data { txs: value.data }),
                evidence: Some(value.evidence.into()),
                last_commit: value.last_commit.map(Into::into),
            }
        }
    }
}

mod v1beta1 {
    use super::{Block, Commit, Header};
    use crate::{prelude::*, Error};
    use cometbft_proto::types::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl Protobuf<pb::Block> for Block {}

    impl TryFrom<pb::Block> for Block {
        type Error = Error;

        fn try_from(value: pb::Block) -> Result<Self, Self::Error> {
            let header: Header = value.header.ok_or_else(Error::missing_header)?.try_into()?;
            // If last_commit is the default Commit, it is considered nil by Go.
            let last_commit = value
                .last_commit
                .map(TryInto::try_into)
                .transpose()?
                .filter(|c| c != &Commit::default());

            Ok(Block::new(
                header,
                value.data.ok_or_else(Error::missing_data)?.txs,
                value
                    .evidence
                    .map(TryInto::try_into)
                    .transpose()?
                    .unwrap_or_default(),
                last_commit,
            ))
        }
    }

    impl From<Block> for pb::Block {
        fn from(value: Block) -> Self {
            pb::Block {
                header: Some(value.header.into()),
                data: Some(pb::Data { txs: value.data }),
                evidence: Some(value.evidence.into()),
                last_commit: value.last_commit.map(Into::into),
            }
        }
    }
}

impl Block {
    /// Builds a new [`Block`], based on the given [`Header`], data, evidence, and last commit.
    pub fn new(
        header: Header,
        data: Vec<Vec<u8>>,
        evidence: evidence::List,
        last_commit: Option<Commit>,
    ) -> Self {
        Self {
            header,
            data,
            evidence,
            last_commit,
        }
    }

    /// Get header
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Get data
    pub fn data(&self) -> &Vec<Vec<u8>> {
        &self.data
    }

    /// Get evidence
    pub fn evidence(&self) -> &evidence::List {
        &self.evidence
    }

    /// Get last commit
    pub fn last_commit(&self) -> &Option<Commit> {
        &self.last_commit
    }
}
