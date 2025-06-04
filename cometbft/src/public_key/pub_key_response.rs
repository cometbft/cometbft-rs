use crate::prelude::*;
use crate::privval::RemoteSignerError;
use crate::PublicKey;

/// PubKeyResponse
#[derive(Clone, PartialEq, Eq, Debug)]
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
            let pk = value.pub_key.map(TryInto::try_into).transpose();
            match pk {
                Ok(Some(pub_key)) => {
                    Ok(PubKeyResponse {
                        pub_key: Some(pub_key),
                        error: None,
                    })
                },
                _ => Ok(PubKeyResponse {
                    pub_key: None,
                    error: value.error.map(TryInto::try_into).transpose()?,
                }),
            }
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
            let pk = PublicKey::try_from_type_and_bytes(&value.pub_key_type, &value.pub_key_bytes);
            match pk {
                Ok(pub_key) => Ok(PubKeyResponse {
                    pub_key: Some(pub_key),
                    error: None,
                }),
                _ => Ok(PubKeyResponse {
                    pub_key: None,
                    error: value.error.map(TryInto::try_into).transpose()?,
                }),
            }
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
            let pk = value.pub_key.map(TryInto::try_into).transpose();
            match pk {
                Ok(Some(pub_key)) => Ok(PubKeyResponse {
                    pub_key: Some(pub_key),
                    error: None,
                }),
                _ => Ok(PubKeyResponse {
                    pub_key: None,
                    error: value.error.map(TryInto::try_into).transpose()?,
                }),
            }
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

#[cfg(test)]
mod tests {
    cometbft_old_pb_modules! {
        use super::super::PubKeyResponse;
        use pb::privval::PubKeyResponse as RawPubKeyResponse;
        use crate::{privval::RemoteSignerError, prelude::*};

        #[test]
        fn test_error_pubkey_response() {
            // test-vector generated via Go:
            // pkr := &privval.PubKeyResponse{
            //   Error: &privval.RemoteSignerError{
            //     Code:        2,
            //     Description: "boom",
            //   },
            //   PubKeyBytes: []byte{},
            //   PubKeyType:  "",
            // }
            // pbpk, err := pkr.Marshal()
            // if err != nil {
            //   panic(err)
            // }
            // fmt.Printf("%#+v\n", pbpk)

            let want: Vec<u8> = vec![0x12, 0x8, 0x8, 0x2, 0x12, 0x4, 0x62, 0x6f, 0x6f, 0x6d];

            let msg = PubKeyResponse {
                pub_key: None,
                error: Some(RemoteSignerError {
                    code: 2,
                    description: "boom".to_string(),
                }),
            };

            let mut got = vec![];
            Protobuf::<RawPubKeyResponse>::encode(msg.clone(), &mut got).unwrap();
            assert_eq!(got, want);

            match <PubKeyResponse as Protobuf<RawPubKeyResponse>>::decode(want.as_ref()) {
                Ok(have) => assert_eq!(have, msg),
                Err(err) => panic!("{}", err.to_string()),
            }
        }
    }

    mod v1 {
        use super::super::PubKeyResponse;
        use crate::{prelude::*, privval::RemoteSignerError};
        use cometbft_proto::privval::v1::PubKeyResponse as RawPubKeyResponse;
        use cometbft_proto::Protobuf;

        #[test]
        fn test_error_pubkey_response() {
            // test-vector generated via Go:
            // pkr := &privval.PubKeyResponse{
            //   Error: &privval.RemoteSignerError{
            //     Code:        2,
            //     Description: "boom",
            //   },
            //   PubKeyBytes: []byte{},
            //   PubKeyType:  "",
            // }
            // pbpk, err := pkr.Marshal()
            // if err != nil {
            //   panic(err)
            // }
            // fmt.Printf("%#+v\n", pbpk)

            let want: Vec<u8> = vec![0x12, 0x8, 0x8, 0x2, 0x12, 0x4, 0x62, 0x6f, 0x6f, 0x6d];

            let msg = PubKeyResponse {
                pub_key: None,
                error: Some(RemoteSignerError {
                    code: 2,
                    description: "boom".to_string(),
                }),
            };

            let mut got = vec![];
            Protobuf::<RawPubKeyResponse>::encode(msg.clone(), &mut got).unwrap();
            assert_eq!(got, want);

            match <PubKeyResponse as Protobuf<RawPubKeyResponse>>::decode(want.as_ref()) {
                Ok(have) => assert_eq!(have, msg),
                Err(err) => panic!("{}", err.to_string()),
            }
        }
    }
}
