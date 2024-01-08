use super::super::types::Snapshot;
use crate::prelude::*;

#[doc = include_str!("../doc/response-listsnapshots.md")]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ListSnapshots {
    /// A list of local state snapshots.
    pub snapshots: Vec<Snapshot>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1 {
    use super::ListSnapshots;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<ListSnapshots> for pb::ListSnapshotsResponse {
        fn from(list_snapshots: ListSnapshots) -> Self {
            Self {
                snapshots: list_snapshots
                    .snapshots
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            }
        }
    }

    impl TryFrom<pb::ListSnapshotsResponse> for ListSnapshots {
        type Error = crate::Error;

        fn try_from(list_snapshots: pb::ListSnapshotsResponse) -> Result<Self, Self::Error> {
            Ok(Self {
                snapshots: list_snapshots
                    .snapshots
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    impl Protobuf<pb::ListSnapshotsResponse> for ListSnapshots {}
}

mod v1beta1 {
    use super::ListSnapshots;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<ListSnapshots> for pb::ResponseListSnapshots {
        fn from(list_snapshots: ListSnapshots) -> Self {
            Self {
                snapshots: list_snapshots
                    .snapshots
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            }
        }
    }

    impl TryFrom<pb::ResponseListSnapshots> for ListSnapshots {
        type Error = crate::Error;

        fn try_from(list_snapshots: pb::ResponseListSnapshots) -> Result<Self, Self::Error> {
            Ok(Self {
                snapshots: list_snapshots
                    .snapshots
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    impl Protobuf<pb::ResponseListSnapshots> for ListSnapshots {}
}
