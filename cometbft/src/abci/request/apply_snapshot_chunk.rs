use bytes::Bytes;

// bring into scope for doc links
#[allow(unused)]
use super::{super::types::Snapshot, Info, LoadSnapshotChunk};
use crate::prelude::*;

#[doc = include_str!("../doc/request-applysnapshotchunk.md")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ApplySnapshotChunk {
    /// The chunk index, starting from `0`.  CometBFT applies chunks sequentially.
    pub index: u32,
    /// The binary chunk contents, as returned by [`LoadSnapshotChunk`].
    pub chunk: Bytes,
    /// The P2P ID of the node who sent this chunk.
    pub sender: String,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

cometbft_old_pb_modules! {
    use super::ApplySnapshotChunk;

    impl From<ApplySnapshotChunk> for pb::abci::RequestApplySnapshotChunk {
        fn from(apply_snapshot_chunk: ApplySnapshotChunk) -> Self {
            Self {
                index: apply_snapshot_chunk.index,
                chunk: apply_snapshot_chunk.chunk,
                sender: apply_snapshot_chunk.sender,
            }
        }
    }

    impl TryFrom<pb::abci::RequestApplySnapshotChunk> for ApplySnapshotChunk {
        type Error = crate::Error;

        fn try_from(
            apply_snapshot_chunk: pb::abci::RequestApplySnapshotChunk,
        ) -> Result<Self, Self::Error> {
            Ok(Self {
                index: apply_snapshot_chunk.index,
                chunk: apply_snapshot_chunk.chunk,
                sender: apply_snapshot_chunk.sender,
            })
        }
    }

    impl Protobuf<pb::abci::RequestApplySnapshotChunk> for ApplySnapshotChunk {}
}

mod v1 {
    use super::ApplySnapshotChunk;
    use cometbft_proto::v1::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<ApplySnapshotChunk> for pb::ApplySnapshotChunkRequest {
        fn from(apply_snapshot_chunk: ApplySnapshotChunk) -> Self {
            Self {
                index: apply_snapshot_chunk.index,
                chunk: apply_snapshot_chunk.chunk,
                sender: apply_snapshot_chunk.sender,
            }
        }
    }

    impl TryFrom<pb::ApplySnapshotChunkRequest> for ApplySnapshotChunk {
        type Error = crate::Error;

        fn try_from(
            apply_snapshot_chunk: pb::ApplySnapshotChunkRequest,
        ) -> Result<Self, Self::Error> {
            Ok(Self {
                index: apply_snapshot_chunk.index,
                chunk: apply_snapshot_chunk.chunk,
                sender: apply_snapshot_chunk.sender,
            })
        }
    }

    impl Protobuf<pb::ApplySnapshotChunkRequest> for ApplySnapshotChunk {}
}

mod v1beta1 {
    use super::ApplySnapshotChunk;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<ApplySnapshotChunk> for pb::RequestApplySnapshotChunk {
        fn from(apply_snapshot_chunk: ApplySnapshotChunk) -> Self {
            Self {
                index: apply_snapshot_chunk.index,
                chunk: apply_snapshot_chunk.chunk,
                sender: apply_snapshot_chunk.sender,
            }
        }
    }

    impl TryFrom<pb::RequestApplySnapshotChunk> for ApplySnapshotChunk {
        type Error = crate::Error;

        fn try_from(
            apply_snapshot_chunk: pb::RequestApplySnapshotChunk,
        ) -> Result<Self, Self::Error> {
            Ok(Self {
                index: apply_snapshot_chunk.index,
                chunk: apply_snapshot_chunk.chunk,
                sender: apply_snapshot_chunk.sender,
            })
        }
    }

    impl Protobuf<pb::RequestApplySnapshotChunk> for ApplySnapshotChunk {}
}
