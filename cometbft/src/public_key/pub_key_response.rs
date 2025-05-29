use crate::prelude::*;
use crate::privval::RemoteSignerError;
use crate::PublicKey;

/// PubKeyResponse
#[derive(Clone, PartialEq, Eq, Debug)]
// TODO: either pub_key OR error is present
pub struct PubKeyResponse {
    /// Public key
    ///
    /// From CometBFT v1.0.0 onwards, use `pub_key_bytes` and `pub_key_type` instead.
    pub pub_key: Option<PublicKey>,
    /// Error
    pub error: Option<RemoteSignerError>,
    /// Public key bytes
    ///
    /// This field has been added in CometBFT 1.0.0 and will be ignored when
    /// encoding into earlier protocol versions.
    pub pub_key_bytes: bytes::Bytes,
    /// Public key type
    ///
    /// This field has been added in CometBFT 1.0.0 and will be ignored when
    /// encoding into earlier protocol versions.
    pub pub_key_type: String,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

cometbft_old_pb_modules! {
    use super::PubKeyResponse;
    use pb::privval::PubKeyResponse as RawPubKeyResponse;

    impl Protobuf<RawPubKeyResponse> for PubKeyResponse {}

    impl TryFrom<RawPubKeyResponse> for PubKeyResponse {
        type Error = crate::Error;

        fn try_from(value: RawPubKeyResponse) -> Result<Self, Self::Error> {
            Ok(PubKeyResponse {
                pub_key: value.pub_key.map(TryInto::try_into).transpose()?,
                error: value.error.map(TryInto::try_into).transpose()?,
                pub_key_bytes: Default::default(), // pub_key_bytes is not present in older
                                                   // versions
                pub_key_type: Default::default(), // pub_key_type is not present in older versions
            })
        }
    }

    impl From<PubKeyResponse> for RawPubKeyResponse {
        fn from(value: PubKeyResponse) -> Self {
            RawPubKeyResponse {
                pub_key: value.pub_key.map(Into::into),
                error: value.error.map(Into::into),
            }
        }
    }
}

mod v1 {
    use super::PubKeyResponse;
    use cometbft_proto::privval::v1::PubKeyResponse as RawPubKeyResponse;
    use cometbft_proto::Protobuf;

    impl Protobuf<RawPubKeyResponse> for PubKeyResponse {}

    impl TryFrom<RawPubKeyResponse> for PubKeyResponse {
        type Error = crate::Error;

        fn try_from(value: RawPubKeyResponse) -> Result<Self, Self::Error> {
            Ok(PubKeyResponse {
                pub_key: Default::default(), // pub_key is not present in v1
                error: value.error.map(TryInto::try_into).transpose()?,
                pub_key_bytes: bytes::Bytes::try_from(value.pub_key_bytes)?,
                pub_key_type: value.pub_key_type.try_into()?,
            })
        }
    }

    impl From<PubKeyResponse> for RawPubKeyResponse {
        fn from(value: PubKeyResponse) -> Self {
            RawPubKeyResponse {
                error: value.error.map(Into::into),
                pub_key_bytes: value.pub_key_bytes.into(),
                pub_key_type: value.pub_key_type.into(),
            }
        }
    }
}

mod v1beta1 {
    use super::PubKeyResponse;
    use cometbft_proto::privval::v1beta1::PubKeyResponse as RawPubKeyResponse;
    use cometbft_proto::Protobuf;

    impl Protobuf<RawPubKeyResponse> for PubKeyResponse {}

    impl TryFrom<RawPubKeyResponse> for PubKeyResponse {
        type Error = crate::Error;

        fn try_from(value: RawPubKeyResponse) -> Result<Self, Self::Error> {
            Ok(PubKeyResponse {
                pub_key: value.pub_key.map(TryInto::try_into).transpose()?,
                error: value.error.map(TryInto::try_into).transpose()?,
                pub_key_bytes: Default::default(), // pub_key_bytes is not present in v1beta1
                pub_key_type: Default::default(),  // pub_key_type is not present in v1beta1
            })
        }
    }

    impl From<PubKeyResponse> for RawPubKeyResponse {
        fn from(value: PubKeyResponse) -> Self {
            RawPubKeyResponse {
                pub_key: value.pub_key.map(Into::into),
                error: value.error.map(Into::into),
            }
        }
    }
}

// Todo: write unit test
