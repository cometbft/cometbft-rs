use crate::{account, block, Hash};
use bytes::Bytes;

#[doc = include_str!("../doc/request-verifyvoteextension.md")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VerifyVoteExtension {
    pub hash: Hash,
    pub validator_address: account::Id,
    pub height: block::Height,
    pub vote_extension: Bytes,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1 {
    use super::VerifyVoteExtension;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<VerifyVoteExtension> for pb::VerifyVoteExtensionRequest {
        fn from(value: VerifyVoteExtension) -> Self {
            Self {
                hash: value.hash.into(),
                validator_address: value.validator_address.into(),
                height: value.height.into(),
                vote_extension: value.vote_extension,
            }
        }
    }

    impl TryFrom<pb::VerifyVoteExtensionRequest> for VerifyVoteExtension {
        type Error = crate::Error;

        fn try_from(message: pb::VerifyVoteExtensionRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                hash: message.hash.try_into()?,
                validator_address: message.validator_address.try_into()?,
                height: message.height.try_into()?,
                vote_extension: message.vote_extension,
            })
        }
    }

    impl Protobuf<pb::VerifyVoteExtensionRequest> for VerifyVoteExtension {}
}

mod v1beta3 {
    use super::VerifyVoteExtension;
    use cometbft_proto::abci::v1beta3 as pb;
    use cometbft_proto::Protobuf;

    impl From<VerifyVoteExtension> for pb::RequestVerifyVoteExtension {
        fn from(value: VerifyVoteExtension) -> Self {
            Self {
                hash: value.hash.into(),
                validator_address: value.validator_address.into(),
                height: value.height.into(),
                vote_extension: value.vote_extension,
            }
        }
    }

    impl TryFrom<pb::RequestVerifyVoteExtension> for VerifyVoteExtension {
        type Error = crate::Error;

        fn try_from(message: pb::RequestVerifyVoteExtension) -> Result<Self, Self::Error> {
            Ok(Self {
                hash: message.hash.try_into()?,
                validator_address: message.validator_address.try_into()?,
                height: message.height.try_into()?,
                vote_extension: message.vote_extension,
            })
        }
    }

    impl Protobuf<pb::RequestVerifyVoteExtension> for VerifyVoteExtension {}
}
