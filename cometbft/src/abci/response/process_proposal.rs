use crate::prelude::*;

#[doc = include_str!("../doc/response-processproposal.md")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
#[derive(Default)]
pub enum ProcessProposal {
    #[default]
    Unknown = 0,
    Accept = 1,
    Reject = 2,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1 {
    use super::ProcessProposal;
    use crate::Error;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<ProcessProposal> for pb::ProcessProposalResponse {
        fn from(value: ProcessProposal) -> Self {
            Self {
                status: value as i32,
            }
        }
    }

    impl TryFrom<pb::ProcessProposalResponse> for ProcessProposal {
        type Error = Error;

        fn try_from(message: pb::ProcessProposalResponse) -> Result<Self, Self::Error> {
            use pb::ProcessProposalStatus;

            let status = message
                .status
                .try_into()
                .map_err(|_| Error::unsupported_process_proposal_status())?;

            let value = match status {
                ProcessProposalStatus::Unknown => ProcessProposal::Unknown,
                ProcessProposalStatus::Accept => ProcessProposal::Accept,
                ProcessProposalStatus::Reject => ProcessProposal::Reject,
            };
            Ok(value)
        }
    }

    impl Protobuf<pb::ProcessProposalResponse> for ProcessProposal {}
}

mod v1beta2 {
    use super::ProcessProposal;
    use crate::Error;
    use cometbft_proto::abci::v1beta2 as pb;
    use cometbft_proto::Protobuf;

    impl From<ProcessProposal> for pb::ResponseProcessProposal {
        fn from(value: ProcessProposal) -> Self {
            Self {
                status: value as i32,
            }
        }
    }

    impl TryFrom<pb::ResponseProcessProposal> for ProcessProposal {
        type Error = Error;

        fn try_from(message: pb::ResponseProcessProposal) -> Result<Self, Self::Error> {
            use pb::response_process_proposal::ProposalStatus;

            let status = message
                .status
                .try_into()
                .map_err(|_| Error::unsupported_process_proposal_status())?;

            let value = match status {
                ProposalStatus::Unknown => ProcessProposal::Unknown,
                ProposalStatus::Accept => ProcessProposal::Accept,
                ProposalStatus::Reject => ProcessProposal::Reject,
            };
            Ok(value)
        }
    }

    impl Protobuf<pb::ResponseProcessProposal> for ProcessProposal {}
}
