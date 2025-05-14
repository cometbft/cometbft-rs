use crate::prelude::*;

#[doc = include_str!("../doc/request-echo.md")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Echo {
    /// The message to send back.
    pub message: String,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

cometbft_old_pb_modules! {
    use super::Echo;

    impl From<Echo> for pb::abci::RequestEcho {
        fn from(echo: Echo) -> Self {
            Self {
                message: echo.message,
            }
        }
    }

    impl TryFrom<pb::abci::RequestEcho> for Echo {
        type Error = crate::Error;

        fn try_from(echo: pb::abci::RequestEcho) -> Result<Self, Self::Error> {
            Ok(Self {
                message: echo.message,
            })
        }
    }

    impl Protobuf<pb::abci::RequestEcho> for Echo {}
}

mod v1 {
    use super::Echo;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<Echo> for pb::EchoRequest {
        fn from(echo: Echo) -> Self {
            Self {
                message: echo.message,
            }
        }
    }

    impl TryFrom<pb::EchoRequest> for Echo {
        type Error = crate::Error;

        fn try_from(echo: pb::EchoRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                message: echo.message,
            })
        }
    }

    impl Protobuf<pb::EchoRequest> for Echo {}
}

mod v1beta1 {
    use super::Echo;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<Echo> for pb::RequestEcho {
        fn from(echo: Echo) -> Self {
            Self {
                message: echo.message,
            }
        }
    }

    impl TryFrom<pb::RequestEcho> for Echo {
        type Error = crate::Error;

        fn try_from(echo: pb::RequestEcho) -> Result<Self, Self::Error> {
            Ok(Self {
                message: echo.message,
            })
        }
    }

    impl Protobuf<pb::RequestEcho> for Echo {}
}
