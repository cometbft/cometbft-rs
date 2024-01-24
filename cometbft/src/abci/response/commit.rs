use crate::{block, prelude::*};
use bytes::Bytes;

#[doc = include_str!("../doc/response-commit.md")]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Commit {
    /// The Merkle root hash of the application state.
    ///
    /// This field is ignored since CometBFT 0.38.
    pub data: Bytes,
    /// Blocks below this height may be removed.
    pub retain_height: block::Height,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1beta1 {
    use super::Commit;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<Commit> for pb::ResponseCommit {
        fn from(commit: Commit) -> Self {
            Self {
                data: commit.data,
                retain_height: commit.retain_height.into(),
            }
        }
    }

    impl TryFrom<pb::ResponseCommit> for Commit {
        type Error = crate::Error;

        fn try_from(commit: pb::ResponseCommit) -> Result<Self, Self::Error> {
            Ok(Self {
                data: commit.data,
                retain_height: commit.retain_height.try_into()?,
            })
        }
    }

    impl Protobuf<pb::ResponseCommit> for Commit {}
}

mod v1beta3 {
    use super::Commit;
    use cometbft_proto::abci::v1beta3 as pb;
    use cometbft_proto::Protobuf;

    impl From<Commit> for pb::ResponseCommit {
        fn from(commit: Commit) -> Self {
            Self {
                retain_height: commit.retain_height.into(),
            }
        }
    }

    impl TryFrom<pb::ResponseCommit> for Commit {
        type Error = crate::Error;

        fn try_from(commit: pb::ResponseCommit) -> Result<Self, Self::Error> {
            Ok(Self {
                retain_height: commit.retain_height.try_into()?,
                data: Default::default(),
            })
        }
    }

    impl Protobuf<pb::ResponseCommit> for Commit {}
}

mod v1 {
    use super::Commit;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<Commit> for pb::CommitResponse {
        fn from(commit: Commit) -> Self {
            Self {
                retain_height: commit.retain_height.into(),
            }
        }
    }

    impl TryFrom<pb::CommitResponse> for Commit {
        type Error = crate::Error;

        fn try_from(commit: pb::CommitResponse) -> Result<Self, Self::Error> {
            Ok(Self {
                retain_height: commit.retain_height.try_into()?,
                data: Default::default(),
            })
        }
    }

    impl Protobuf<pb::CommitResponse> for Commit {}
}
