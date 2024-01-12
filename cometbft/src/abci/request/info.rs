use crate::prelude::*;

#[doc = include_str!("../doc/request-info.md")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Info {
    /// The CometBFT software semantic version.
    pub version: String,
    /// The CometBFT block protocol version.
    pub block_version: u64,
    /// The CometBFT p2p protocol version.
    pub p2p_version: u64,
    /// The ABCI protocol version.
    pub abci_version: String,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1beta1 {
    use super::Info;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<Info> for pb::RequestInfo {
        fn from(info: Info) -> Self {
            Self {
                version: info.version,
                block_version: info.block_version,
                p2p_version: info.p2p_version,
            }
        }
    }

    impl TryFrom<pb::RequestInfo> for Info {
        type Error = crate::Error;

        fn try_from(info: pb::RequestInfo) -> Result<Self, Self::Error> {
            Ok(Self {
                version: info.version,
                block_version: info.block_version,
                p2p_version: info.p2p_version,
                abci_version: Default::default(),
            })
        }
    }

    impl Protobuf<pb::RequestInfo> for Info {}
}

mod v1beta2 {
    use super::Info;
    use cometbft_proto::abci::v1beta2 as pb;
    use cometbft_proto::Protobuf;

    impl From<Info> for pb::RequestInfo {
        fn from(info: Info) -> Self {
            Self {
                version: info.version,
                block_version: info.block_version,
                p2p_version: info.p2p_version,
                abci_version: info.abci_version,
            }
        }
    }

    impl TryFrom<pb::RequestInfo> for Info {
        type Error = crate::Error;

        fn try_from(info: pb::RequestInfo) -> Result<Self, Self::Error> {
            Ok(Self {
                version: info.version,
                block_version: info.block_version,
                p2p_version: info.p2p_version,
                abci_version: info.abci_version,
            })
        }
    }

    impl Protobuf<pb::RequestInfo> for Info {}
}

mod v1 {
    use super::Info;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<Info> for pb::InfoRequest {
        fn from(info: Info) -> Self {
            Self {
                version: info.version,
                block_version: info.block_version,
                p2p_version: info.p2p_version,
                abci_version: info.abci_version,
            }
        }
    }

    impl TryFrom<pb::InfoRequest> for Info {
        type Error = crate::Error;

        fn try_from(info: pb::InfoRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                version: info.version,
                block_version: info.block_version,
                p2p_version: info.p2p_version,
                abci_version: info.abci_version,
            })
        }
    }

    impl Protobuf<pb::InfoRequest> for Info {}
}
