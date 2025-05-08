use crate::prelude::*;

#[doc = include_str!("../doc/response-exception.md")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Exception {
    /// Undocumented.
    pub error: String,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

cometbft_old_pb_modules! {
    use super::Exception;

    impl From<Exception> for pb::abci::ResponseException {
        fn from(exception: Exception) -> Self {
            Self {
                error: exception.error,
            }
        }
    }

    impl TryFrom<pb::abci::ResponseException> for Exception {
        type Error = crate::Error;

        fn try_from(exception: pb::abci::ResponseException) -> Result<Self, Self::Error> {
            Ok(Self {
                error: exception.error,
            })
        }
    }

    impl Protobuf<pb::abci::ResponseException> for Exception {}
}

mod v1 {
    use super::Exception;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<Exception> for pb::ExceptionResponse {
        fn from(exception: Exception) -> Self {
            Self {
                error: exception.error,
            }
        }
    }

    impl TryFrom<pb::ExceptionResponse> for Exception {
        type Error = crate::Error;

        fn try_from(exception: pb::ExceptionResponse) -> Result<Self, Self::Error> {
            Ok(Self {
                error: exception.error,
            })
        }
    }

    impl Protobuf<pb::ExceptionResponse> for Exception {}
}

mod v1beta1 {
    use super::Exception;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<Exception> for pb::ResponseException {
        fn from(exception: Exception) -> Self {
            Self {
                error: exception.error,
            }
        }
    }

    impl TryFrom<pb::ResponseException> for Exception {
        type Error = crate::Error;

        fn try_from(exception: pb::ResponseException) -> Result<Self, Self::Error> {
            Ok(Self {
                error: exception.error,
            })
        }
    }

    impl Protobuf<pb::ResponseException> for Exception {}
}
