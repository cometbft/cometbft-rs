//! Signing arbitrary bytes.

use crate::{prelude::*, privval::RemoteSignerError};

/// A request to sign arbitrary application-specific bytes (e.g. consensus metadata, hashes).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SignBytesRequest {
    /// Bytes to sign
    pub bytes: bytes::Bytes,
}

/// Response to a `SignBytesRequest`, containing a signature if successful or an error otherwise.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SignBytesResponse {
    /// Signature
    pub signature: bytes::Bytes,
    /// Response error
    pub error: Option<RemoteSignerError>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1 {
    use super::{SignBytesRequest, SignBytesResponse};
    use crate::{prelude::*, Error};
    use cometbft_proto::privval::v1::{
        SignBytesRequest as RawSignBytesRequest, SignBytesResponse as RawSignBytesResponse,
    };
    use cometbft_proto::Protobuf;

    impl Protobuf<RawSignBytesRequest> for SignBytesRequest {}
    impl Protobuf<RawSignBytesResponse> for SignBytesResponse {}

    impl TryFrom<RawSignBytesRequest> for SignBytesRequest {
        type Error = Error;

        fn try_from(req: RawSignBytesRequest) -> Result<Self, Self::Error> {
            Ok(SignBytesRequest {
                bytes: bytes::Bytes::try_from(req.value)?,
            })
        }
    }

    impl From<SignBytesRequest> for RawSignBytesRequest {
        fn from(req: SignBytesRequest) -> Self {
            RawSignBytesRequest {
                value: req.bytes.into(),
            }
        }
    }

    impl TryFrom<RawSignBytesResponse> for SignBytesResponse {
        type Error = Error;

        fn try_from(resp: RawSignBytesResponse) -> Result<Self, Self::Error> {
            Ok(SignBytesResponse {
                signature: bytes::Bytes::try_from(resp.signature)?,
                error: resp.error.map(TryInto::try_into).transpose()?,
            })
        }
    }

    impl From<SignBytesResponse> for RawSignBytesResponse {
        fn from(resp: SignBytesResponse) -> Self {
            RawSignBytesResponse {
                signature: resp.signature.into(),
                error: resp.error.map(Into::into),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SignBytesRequest, SignBytesResponse};

    mod v1 {
        use super::*;
        use crate::privval::RemoteSignerError;
        use cometbft_proto::privval::v1::{
            SignBytesRequest as RawSignBytesRequest, SignBytesResponse as RawSignBytesResponse,
        };

        #[test]
        fn test_protobuf_conversion_request_round_trip() {
            let original = SignBytesRequest {
                bytes: bytes::Bytes::from_static(b"test bytes"),
            };
            let raw: RawSignBytesRequest = original.clone().into();
            let decoded = SignBytesRequest::try_from(raw).unwrap();
            assert_eq!(original, decoded);
        }

        #[test]
        fn test_protobuf_conversion_response_with_signature() {
            let original = SignBytesResponse {
                signature: bytes::Bytes::from_static(b"test signature"),
                error: None,
            };
            let raw: RawSignBytesResponse = original.clone().into();
            let decoded = SignBytesResponse::try_from(raw).unwrap();
            assert_eq!(original, decoded);
        }

        #[test]
        fn test_protobuf_conversion_response_with_error() {
            let original = SignBytesResponse {
                signature: bytes::Bytes::from_static(b"test signature"),
                error: Some(RemoteSignerError {
                    code: 0,
                    description: "test".into(),
                }),
            };
            let raw: RawSignBytesResponse = original.clone().into();
            let decoded = SignBytesResponse::try_from(raw).unwrap();
            assert_eq!(original, decoded);
        }
    }
}
