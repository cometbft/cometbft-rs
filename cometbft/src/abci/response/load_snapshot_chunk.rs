use bytes::Bytes;

use crate::prelude::*;

#[doc = include_str!("../doc/response-loadsnapshotchunk.md")]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct LoadSnapshotChunk {
    /// The binary chunk contents, in an arbitrary format.
    ///
    /// Chunk messages cannot be larger than 16MB *including metadata*, so 10MB
    /// is a good starting point.
    pub chunk: Bytes,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

cometbft_old_pb_modules! {
    use super::LoadSnapshotChunk;

    impl From<LoadSnapshotChunk> for pb::abci::ResponseLoadSnapshotChunk {
        fn from(load_snapshot_chunk: LoadSnapshotChunk) -> Self {
            Self {
                chunk: load_snapshot_chunk.chunk,
            }
        }
    }

    impl TryFrom<pb::abci::ResponseLoadSnapshotChunk> for LoadSnapshotChunk {
        type Error = crate::Error;

        fn try_from(load_snapshot_chunk: pb::abci::ResponseLoadSnapshotChunk) -> Result<Self, Self::Error> {
            Ok(Self {
                chunk: load_snapshot_chunk.chunk,
            })
        }
    }

    impl Protobuf<pb::abci::ResponseLoadSnapshotChunk> for LoadSnapshotChunk {}
}

mod v1 {
    use super::LoadSnapshotChunk;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<LoadSnapshotChunk> for pb::LoadSnapshotChunkResponse {
        fn from(load_snapshot_chunk: LoadSnapshotChunk) -> Self {
            Self {
                chunk: load_snapshot_chunk.chunk,
            }
        }
    }

    impl TryFrom<pb::LoadSnapshotChunkResponse> for LoadSnapshotChunk {
        type Error = crate::Error;

        fn try_from(
            load_snapshot_chunk: pb::LoadSnapshotChunkResponse,
        ) -> Result<Self, Self::Error> {
            Ok(Self {
                chunk: load_snapshot_chunk.chunk,
            })
        }
    }

    impl Protobuf<pb::LoadSnapshotChunkResponse> for LoadSnapshotChunk {}
}

mod v1beta1 {
    use super::LoadSnapshotChunk;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<LoadSnapshotChunk> for pb::ResponseLoadSnapshotChunk {
        fn from(load_snapshot_chunk: LoadSnapshotChunk) -> Self {
            Self {
                chunk: load_snapshot_chunk.chunk,
            }
        }
    }

    impl TryFrom<pb::ResponseLoadSnapshotChunk> for LoadSnapshotChunk {
        type Error = crate::Error;

        fn try_from(
            load_snapshot_chunk: pb::ResponseLoadSnapshotChunk,
        ) -> Result<Self, Self::Error> {
            Ok(Self {
                chunk: load_snapshot_chunk.chunk,
            })
        }
    }

    impl Protobuf<pb::ResponseLoadSnapshotChunk> for LoadSnapshotChunk {}
}
