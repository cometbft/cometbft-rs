//! Types used in the Privval protocol (Tendermint Core [ADR-063])
//!
//! [ADR-063]: https://github.com/cometbft/cometbft/blob/main/docs/architecture/adr-063-privval-grpc.md

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemoteSignerError {
    pub code: i32,
    pub description: String,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

cometbft_old_pb_modules! {
    use super::RemoteSignerError;
    use pb::privval::RemoteSignerError as RawRemoteSignerError;

    impl TryFrom<RawRemoteSignerError> for RemoteSignerError {
        type Error = crate::Error;

        fn try_from(message: RawRemoteSignerError) -> Result<Self, Self::Error> {
            Ok(Self {
                code: message.code,
                description: message.description,
            })
        }
    }

    impl From<RemoteSignerError> for RawRemoteSignerError {
        fn from(value: RemoteSignerError) -> Self {
            Self {
                code: value.code,
                description: value.description,
            }
        }
    }
}

mod v1 {
    use super::RemoteSignerError;
    use cometbft_proto::privval::v1::RemoteSignerError as RawRemoteSignerError;

    impl TryFrom<RawRemoteSignerError> for RemoteSignerError {
        type Error = crate::Error;

        fn try_from(message: RawRemoteSignerError) -> Result<Self, Self::Error> {
            Ok(Self {
                code: message.code,
                description: message.description,
            })
        }
    }

    impl From<RemoteSignerError> for RawRemoteSignerError {
        fn from(value: RemoteSignerError) -> Self {
            Self {
                code: value.code,
                description: value.description,
            }
        }
    }
}

mod v1beta1 {
    use super::RemoteSignerError;
    use cometbft_proto::privval::v1beta1::RemoteSignerError as RawRemoteSignerError;

    impl TryFrom<RawRemoteSignerError> for RemoteSignerError {
        type Error = crate::Error;

        fn try_from(message: RawRemoteSignerError) -> Result<Self, Self::Error> {
            Ok(Self {
                code: message.code,
                description: message.description,
            })
        }
    }

    impl From<RemoteSignerError> for RawRemoteSignerError {
        fn from(value: RemoteSignerError) -> Self {
            Self {
                code: value.code,
                description: value.description,
            }
        }
    }
}
