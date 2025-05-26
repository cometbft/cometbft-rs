use cometbft_proto::types::v1::TxProof as RawTxProof;
use serde::{Deserialize, Serialize};

use crate::{merkle, prelude::*, Hash};

/// Merkle proof of the presence of a transaction in the Merkle tree.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawTxProof", into = "RawTxProof")]
pub struct Proof {
    pub root_hash: Hash,
    pub data: Vec<u8>,
    pub proof: merkle::Proof,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

cometbft_old_pb_modules! {
    use super::Proof;
    use crate::Error;
    use pb::types::TxProof as RawTxProof;

    impl TryFrom<RawTxProof> for Proof {
        type Error = Error;

        fn try_from(message: RawTxProof) -> Result<Self, Self::Error> {
            Ok(Self {
                root_hash: message.root_hash.try_into()?,
                data: message.data,
                proof: message.proof.ok_or_else(Error::missing_data)?.try_into()?,
            })
        }
    }

    impl From<Proof> for RawTxProof {
        fn from(value: Proof) -> Self {
            Self {
                root_hash: value.root_hash.into(),
                data: value.data,
                proof: Some(value.proof.into()),
            }
        }
    }
}

mod v1 {
    use super::Proof;
    use crate::Error;
    use cometbft_proto::types::v1 as pb;

    impl TryFrom<pb::TxProof> for Proof {
        type Error = Error;

        fn try_from(message: pb::TxProof) -> Result<Self, Self::Error> {
            Ok(Self {
                root_hash: message.root_hash.try_into()?,
                data: message.data,
                proof: message.proof.ok_or_else(Error::missing_data)?.try_into()?,
            })
        }
    }

    impl From<Proof> for pb::TxProof {
        fn from(value: Proof) -> Self {
            Self {
                root_hash: value.root_hash.into(),
                data: value.data,
                proof: Some(value.proof.into()),
            }
        }
    }
}

mod v1beta1 {
    use super::Proof;
    use crate::Error;
    use cometbft_proto::types::v1beta1 as pb;

    impl TryFrom<pb::TxProof> for Proof {
        type Error = Error;

        fn try_from(message: pb::TxProof) -> Result<Self, Self::Error> {
            Ok(Self {
                root_hash: message.root_hash.try_into()?,
                data: message.data,
                proof: message.proof.ok_or_else(Error::missing_data)?.try_into()?,
            })
        }
    }

    impl From<Proof> for pb::TxProof {
        fn from(value: Proof) -> Self {
            Self {
                root_hash: value.root_hash.into(),
                data: value.data,
                proof: Some(value.proof.into()),
            }
        }
    }
}
