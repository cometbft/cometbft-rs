use crate::{chain::Id as ChainId, prelude::*};

/// PubKeyRequest requests the consensus public key from the remote signer.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PubKeyRequest {
    /// Chain ID
    pub chain_id: ChainId,
}

mod v1beta1 {
    use super::PubKeyRequest;
    use crate::{chain::Id as ChainId, prelude::*};
    use cometbft_proto::privval::v1beta1::PubKeyRequest as RawPubKeyRequest;
    use cometbft_proto::Protobuf;

    impl Protobuf<RawPubKeyRequest> for PubKeyRequest {}

    impl TryFrom<RawPubKeyRequest> for PubKeyRequest {
        type Error = crate::Error;

        fn try_from(value: RawPubKeyRequest) -> Result<Self, Self::Error> {
            Ok(PubKeyRequest {
                chain_id: ChainId::try_from(value.chain_id)?,
            })
        }
    }

    impl From<PubKeyRequest> for RawPubKeyRequest {
        fn from(value: PubKeyRequest) -> Self {
            RawPubKeyRequest {
                chain_id: value.chain_id.as_str().to_string(),
            }
        }
    }
}

mod v1 {
    use super::PubKeyRequest;
    use crate::{chain::Id as ChainId, prelude::*};
    use cometbft_proto::privval::v1::PubKeyRequest as RawPubKeyRequest;
    use cometbft_proto::Protobuf;

    impl Protobuf<RawPubKeyRequest> for PubKeyRequest {}

    impl TryFrom<RawPubKeyRequest> for PubKeyRequest {
        type Error = crate::Error;

        fn try_from(value: RawPubKeyRequest) -> Result<Self, Self::Error> {
            Ok(PubKeyRequest {
                chain_id: ChainId::try_from(value.chain_id)?,
            })
        }
    }

    impl From<PubKeyRequest> for RawPubKeyRequest {
        fn from(value: PubKeyRequest) -> Self {
            RawPubKeyRequest {
                chain_id: value.chain_id.as_str().to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod v1 {
        use super::super::PubKeyRequest;
        use crate::{chain::Id as ChainId, prelude::*};
        use cometbft_proto::privval::v1::PubKeyRequest as RawPubKeyRequest;
        use cometbft_proto::Protobuf;
        use core::str::FromStr;

        #[test]
        fn test_empty_pubkey_msg() {
            // test-vector generated via the following go code:
            // import (
            // "fmt"
            // "github.com/cometbft/cometbft/proto/cometbft/privval"
            // )
            // func ed25519_empty() {
            // pkr := &privval.PubKeyRequest{
            // ChainId: "A",
            // }
            // pbpk, _ := pkr.Marshal()
            // fmt.Printf("%#v\n", pbpk)
            //
            // }

            let want: Vec<u8> = vec![10, 1, 65];
            let msg = PubKeyRequest {
                chain_id: ChainId::from_str("A").unwrap(),
            };
            let mut got = vec![];
            Protobuf::<RawPubKeyRequest>::encode(msg.clone(), &mut got).unwrap();

            assert_eq!(got, want);

            match <PubKeyRequest as Protobuf<RawPubKeyRequest>>::decode(want.as_ref()) {
                Ok(have) => assert_eq!(have, msg),
                Err(err) => panic!("{}", err.to_string()),
            }
        }
    }

    mod v1beta1 {
        use super::super::PubKeyRequest;
        use crate::{chain::Id as ChainId, prelude::*};
        use cometbft_proto::privval::v1beta1::PubKeyRequest as RawPubKeyRequest;
        use cometbft_proto::Protobuf;
        use core::str::FromStr;

        #[test]
        fn test_empty_pubkey_msg() {
            // test-vector generated via the following go code:
            // import (
            // "fmt"
            // "github.com/cometbft/cometbft/proto/cometbft/privval"
            // )
            // func ed25519_empty() {
            // pkr := &privval.PubKeyRequest{
            // ChainId: "A",
            // }
            // pbpk, _ := pkr.Marshal()
            // fmt.Printf("%#v\n", pbpk)
            //
            // }

            let want: Vec<u8> = vec![10, 1, 65];
            let msg = PubKeyRequest {
                chain_id: ChainId::from_str("A").unwrap(),
            };
            let mut got = vec![];
            Protobuf::<RawPubKeyRequest>::encode(msg.clone(), &mut got).unwrap();

            assert_eq!(got, want);

            match <PubKeyRequest as Protobuf<RawPubKeyRequest>>::decode(want.as_ref()) {
                Ok(have) => assert_eq!(have, msg),
                Err(err) => panic!("{}", err.to_string()),
            }
        }
    }
}
