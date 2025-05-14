#[doc = include_str!("../doc/response-verifyvoteextension.md")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum VerifyVoteExtension {
    Unknown = 0,
    Accept = 1,
    Reject = 2,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v0_38 {
    use super::VerifyVoteExtension;
    use crate::Error;
    use cometbft_proto::v0_38::abci as pb;
    use cometbft_proto::Protobuf;

    impl From<VerifyVoteExtension> for pb::ResponseVerifyVoteExtension {
        fn from(value: VerifyVoteExtension) -> Self {
            Self {
                status: value as i32,
            }
        }
    }

    impl TryFrom<pb::ResponseVerifyVoteExtension> for VerifyVoteExtension {
        type Error = Error;

        fn try_from(message: pb::ResponseVerifyVoteExtension) -> Result<Self, Self::Error> {
            use pb::response_verify_vote_extension::VerifyStatus;

            let status = message
                .status
                .try_into()
                .map_err(|_| Error::unsupported_verify_vote_extension_status())?;

            let value = match status {
                VerifyStatus::Unknown => VerifyVoteExtension::Unknown,
                VerifyStatus::Accept => VerifyVoteExtension::Accept,
                VerifyStatus::Reject => VerifyVoteExtension::Reject,
            };
            Ok(value)
        }
    }

    impl Protobuf<pb::ResponseVerifyVoteExtension> for VerifyVoteExtension {}
}

mod v1 {
    use super::VerifyVoteExtension;
    use crate::Error;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<VerifyVoteExtension> for pb::VerifyVoteExtensionResponse {
        fn from(value: VerifyVoteExtension) -> Self {
            Self {
                status: value as i32,
            }
        }
    }

    impl TryFrom<pb::VerifyVoteExtensionResponse> for VerifyVoteExtension {
        type Error = Error;

        fn try_from(message: pb::VerifyVoteExtensionResponse) -> Result<Self, Self::Error> {
            use pb::VerifyVoteExtensionStatus as VerifyStatus;

            let status = message
                .status
                .try_into()
                .map_err(|_| Error::unsupported_verify_vote_extension_status())?;

            let value = match status {
                VerifyStatus::Unknown => VerifyVoteExtension::Unknown,
                VerifyStatus::Accept => VerifyVoteExtension::Accept,
                VerifyStatus::Reject => VerifyVoteExtension::Reject,
            };
            Ok(value)
        }
    }

    impl Protobuf<pb::VerifyVoteExtensionResponse> for VerifyVoteExtension {}
}

mod v1beta3 {
    use super::VerifyVoteExtension;
    use crate::Error;
    use cometbft_proto::abci::v1beta3 as pb;
    use cometbft_proto::Protobuf;

    impl From<VerifyVoteExtension> for pb::ResponseVerifyVoteExtension {
        fn from(value: VerifyVoteExtension) -> Self {
            Self {
                status: value as i32,
            }
        }
    }

    impl TryFrom<pb::ResponseVerifyVoteExtension> for VerifyVoteExtension {
        type Error = Error;

        fn try_from(message: pb::ResponseVerifyVoteExtension) -> Result<Self, Self::Error> {
            use pb::response_verify_vote_extension::VerifyStatus;

            let status = message
                .status
                .try_into()
                .map_err(|_| Error::unsupported_verify_vote_extension_status())?;

            let value = match status {
                VerifyStatus::Unknown => VerifyVoteExtension::Unknown,
                VerifyStatus::Accept => VerifyVoteExtension::Accept,
                VerifyStatus::Reject => VerifyVoteExtension::Reject,
            };
            Ok(value)
        }
    }

    impl Protobuf<pb::ResponseVerifyVoteExtension> for VerifyVoteExtension {}
}
