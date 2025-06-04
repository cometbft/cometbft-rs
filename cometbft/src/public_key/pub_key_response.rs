use crate::prelude::*;
use crate::privval::RemoteSignerError;
use crate::PublicKey;

/// PubKeyResponse
#[derive(Clone, PartialEq, Eq, Debug)]
// TODO: either pub_key OR error is present
pub struct PubKeyResponse {
    /// Public key
    pub pub_key: Option<PublicKey>,
    /// Error
    pub error: Option<RemoteSignerError>,
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
    use crate::PublicKey;
    use cometbft_proto::privval::v1::PubKeyResponse as RawPubKeyResponse;
    use cometbft_proto::Protobuf;

    impl Protobuf<RawPubKeyResponse> for PubKeyResponse {}

    impl TryFrom<RawPubKeyResponse> for PubKeyResponse {
        type Error = crate::Error;

        fn try_from(value: RawPubKeyResponse) -> Result<Self, Self::Error> {
            Ok(PubKeyResponse {
                pub_key: Some(PublicKey::try_from_type_and_bytes(
                    &value.pub_key_type,
                    &value.pub_key_bytes,
                ))
                .transpose()?,
                error: value.error.map(TryInto::try_into).transpose()?,
            })
        }
    }

    impl From<PubKeyResponse> for RawPubKeyResponse {
        fn from(value: PubKeyResponse) -> Self {
            RawPubKeyResponse {
                error: value.error.map(Into::into),
                pub_key_bytes: match value.pub_key {
                    Some(pub_key) => pub_key.to_bytes(),
                    None => Default::default(),
                },
                pub_key_type: match value.pub_key {
                    Some(pub_key) => pub_key.type_str().into(),
                    None => Default::default(),
                },
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
