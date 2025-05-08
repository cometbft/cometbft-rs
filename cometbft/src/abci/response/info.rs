use crate::{block, prelude::*, AppHash};
use cometbft_proto::abci::v1 as pb;

use serde::{Deserialize, Serialize};

#[doc = include_str!("../doc/response-info.md")]
#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
#[serde(default, try_from = "pb::InfoResponse", into = "pb::InfoResponse")]
pub struct Info {
    /// Some arbitrary information.
    pub data: String,
    /// The application software semantic version.
    pub version: String,
    /// The application protocol version.
    pub app_version: u64,
    /// The latest block for which the app has called [`Commit`](super::super::Request::Commit).
    pub last_block_height: block::Height,
    /// The latest result of [`Commit`](super::super::Request::Commit).
    pub last_block_app_hash: AppHash,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

cometbft_old_pb_modules!(abci, {
    use super::Info;

    impl From<Info> for pb::ResponseInfo {
        fn from(info: Info) -> Self {
            Self {
                data: info.data,
                version: info.version,
                app_version: info.app_version,
                last_block_height: info.last_block_height.into(),
                last_block_app_hash: info.last_block_app_hash.into(),
            }
        }
    }

    impl TryFrom<pb::ResponseInfo> for Info {
        type Error = crate::Error;

        fn try_from(info: pb::ResponseInfo) -> Result<Self, Self::Error> {
            Ok(Self {
                data: info.data,
                version: info.version,
                app_version: info.app_version,
                last_block_height: info.last_block_height.try_into()?,
                last_block_app_hash: info.last_block_app_hash.try_into()?,
            })
        }
    }

    impl Protobuf<pb::ResponseInfo> for Info {}
});

mod v1 {
    use super::Info;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<Info> for pb::InfoResponse {
        fn from(info: Info) -> Self {
            Self {
                data: info.data,
                version: info.version,
                app_version: info.app_version,
                last_block_height: info.last_block_height.into(),
                last_block_app_hash: info.last_block_app_hash.into(),
            }
        }
    }

    impl TryFrom<pb::InfoResponse> for Info {
        type Error = crate::Error;

        fn try_from(info: pb::InfoResponse) -> Result<Self, Self::Error> {
            Ok(Self {
                data: info.data,
                version: info.version,
                app_version: info.app_version,
                last_block_height: info.last_block_height.try_into()?,
                last_block_app_hash: info.last_block_app_hash.try_into()?,
            })
        }
    }

    impl Protobuf<pb::InfoResponse> for Info {}
}

mod v1beta1 {
    use super::Info;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<Info> for pb::ResponseInfo {
        fn from(info: Info) -> Self {
            Self {
                data: info.data,
                version: info.version,
                app_version: info.app_version,
                last_block_height: info.last_block_height.into(),
                last_block_app_hash: info.last_block_app_hash.into(),
            }
        }
    }

    impl TryFrom<pb::ResponseInfo> for Info {
        type Error = crate::Error;

        fn try_from(info: pb::ResponseInfo) -> Result<Self, Self::Error> {
            Ok(Self {
                data: info.data,
                version: info.version,
                app_version: info.app_version,
                last_block_height: info.last_block_height.try_into()?,
                last_block_app_hash: info.last_block_app_hash.try_into()?,
            })
        }
    }

    impl Protobuf<pb::ResponseInfo> for Info {}
}
