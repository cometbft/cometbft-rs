use crate::prelude::*;

#[doc = include_str!("../doc/response-echo.md")]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Echo {
    /// The message sent in the request.
    pub message: String,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

cometbft_old_pb_modules! {
    use super::Echo;

    impl From<Echo> for pb::abci::ResponseEcho {
        fn from(echo: Echo) -> Self {
            Self {
                message: echo.message,
            }
        }
    }

    impl TryFrom<pb::abci::ResponseEcho> for Echo {
        type Error = crate::Error;

        fn try_from(echo: pb::abci::ResponseEcho) -> Result<Self, Self::Error> {
            Ok(Self {
                message: echo.message,
            })
        }
    }

    impl Protobuf<pb::abci::ResponseEcho> for Echo {}
}

mod v1 {
    use super::Echo;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<Echo> for pb::EchoResponse {
        fn from(echo: Echo) -> Self {
            Self {
                message: echo.message,
            }
        }
    }

    impl TryFrom<pb::EchoResponse> for Echo {
        type Error = crate::Error;

        fn try_from(echo: pb::EchoResponse) -> Result<Self, Self::Error> {
            Ok(Self {
                message: echo.message,
            })
        }
    }

    impl Protobuf<pb::EchoResponse> for Echo {}
}

mod v1beta1 {
    use super::Echo;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<Echo> for pb::ResponseEcho {
        fn from(echo: Echo) -> Self {
            Self {
                message: echo.message,
            }
        }
    }

    impl TryFrom<pb::ResponseEcho> for Echo {
        type Error = crate::Error;

        fn try_from(echo: pb::ResponseEcho) -> Result<Self, Self::Error> {
            Ok(Self {
                message: echo.message,
            })
        }
    }

    impl Protobuf<pb::ResponseEcho> for Echo {}
}
