use bytes::Bytes;

use crate::prelude::*;

#[doc = include_str!("../doc/response-prepareproposal.md")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrepareProposal {
    pub txs: Vec<Bytes>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1beta2 {
    use super::PrepareProposal;
    use cometbft_proto::abci::v1beta2 as pb;
    use cometbft_proto::Protobuf;

    impl From<PrepareProposal> for pb::ResponsePrepareProposal {
        fn from(value: PrepareProposal) -> Self {
            Self { txs: value.txs }
        }
    }

    impl TryFrom<pb::ResponsePrepareProposal> for PrepareProposal {
        type Error = crate::Error;

        fn try_from(message: pb::ResponsePrepareProposal) -> Result<Self, Self::Error> {
            Ok(Self { txs: message.txs })
        }
    }

    impl Protobuf<pb::ResponsePrepareProposal> for PrepareProposal {}
}

mod v1 {
    use super::PrepareProposal;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<PrepareProposal> for pb::PrepareProposalResponse {
        fn from(value: PrepareProposal) -> Self {
            Self { txs: value.txs }
        }
    }

    impl TryFrom<pb::PrepareProposalResponse> for PrepareProposal {
        type Error = crate::Error;

        fn try_from(message: pb::PrepareProposalResponse) -> Result<Self, Self::Error> {
            Ok(Self { txs: message.txs })
        }
    }

    impl Protobuf<pb::PrepareProposalResponse> for PrepareProposal {}
}
