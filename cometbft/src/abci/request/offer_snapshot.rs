use super::super::types::Snapshot;
// bring into scope for doc links
#[allow(unused)]
use super::ApplySnapshotChunk;
use crate::{prelude::*, AppHash};

#[doc = include_str!("../doc/request-offersnapshot.md")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OfferSnapshot {
    /// The snapshot offered for restoration.
    pub snapshot: Snapshot,
    /// The light client verified app hash for this height.
    pub app_hash: AppHash,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1 {
    use super::OfferSnapshot;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<OfferSnapshot> for pb::OfferSnapshotRequest {
        fn from(offer_snapshot: OfferSnapshot) -> Self {
            Self {
                snapshot: Some(offer_snapshot.snapshot.into()),
                app_hash: offer_snapshot.app_hash.into(),
            }
        }
    }

    impl TryFrom<pb::OfferSnapshotRequest> for OfferSnapshot {
        type Error = crate::Error;

        fn try_from(offer_snapshot: pb::OfferSnapshotRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                snapshot: offer_snapshot
                    .snapshot
                    .ok_or_else(crate::Error::missing_data)?
                    .try_into()?,
                app_hash: offer_snapshot.app_hash.try_into()?,
            })
        }
    }

    impl Protobuf<pb::OfferSnapshotRequest> for OfferSnapshot {}
}

mod v1beta1 {
    use super::OfferSnapshot;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<OfferSnapshot> for pb::RequestOfferSnapshot {
        fn from(offer_snapshot: OfferSnapshot) -> Self {
            Self {
                snapshot: Some(offer_snapshot.snapshot.into()),
                app_hash: offer_snapshot.app_hash.into(),
            }
        }
    }

    impl TryFrom<pb::RequestOfferSnapshot> for OfferSnapshot {
        type Error = crate::Error;

        fn try_from(offer_snapshot: pb::RequestOfferSnapshot) -> Result<Self, Self::Error> {
            Ok(Self {
                snapshot: offer_snapshot
                    .snapshot
                    .ok_or_else(crate::Error::missing_data)?
                    .try_into()?,
                app_hash: offer_snapshot.app_hash.try_into()?,
            })
        }
    }

    impl Protobuf<pb::RequestOfferSnapshot> for OfferSnapshot {}
}
