use bytes::Bytes;

#[doc = include_str!("../doc/response-extendvote.md")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtendVote {
    pub vote_extension: Bytes,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1 {
    use super::ExtendVote;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<ExtendVote> for pb::ExtendVoteResponse {
        fn from(value: ExtendVote) -> Self {
            Self {
                vote_extension: value.vote_extension,
            }
        }
    }

    impl TryFrom<pb::ExtendVoteResponse> for ExtendVote {
        type Error = crate::Error;

        fn try_from(message: pb::ExtendVoteResponse) -> Result<Self, Self::Error> {
            Ok(Self {
                vote_extension: message.vote_extension,
            })
        }
    }

    impl Protobuf<pb::ExtendVoteResponse> for ExtendVote {}
}

mod v1beta3 {
    use super::ExtendVote;
    use cometbft_proto::abci::v1beta3 as pb;
    use cometbft_proto::Protobuf;

    impl From<ExtendVote> for pb::ResponseExtendVote {
        fn from(value: ExtendVote) -> Self {
            Self {
                vote_extension: value.vote_extension,
            }
        }
    }

    impl TryFrom<pb::ResponseExtendVote> for ExtendVote {
        type Error = crate::Error;

        fn try_from(message: pb::ResponseExtendVote) -> Result<Self, Self::Error> {
            Ok(Self {
                vote_extension: message.vote_extension,
            })
        }
    }

    impl Protobuf<pb::ResponseExtendVote> for ExtendVote {}
}
