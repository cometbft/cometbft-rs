//! Merkle proofs

use serde::{Deserialize, Serialize};
use tendermint_proto::v0_37::crypto::Proof as RawProof;

use crate::{prelude::*, serializers, Hash};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawProof", into = "RawProof")]
pub struct Proof {
    // Total number of items.
    pub total: u64,
    // Index of the item to prove.
    pub index: u64,
    // Hash of item value.
    pub leaf_hash: Hash,
    // Hashes from leaf's sibling to a root's child.
    pub aunts: Vec<Hash>,
}

/// Merkle proof defined by the list of ProofOps
/// <https://github.com/tendermint/tendermint/blob/c8483531d8e756f7fbb812db1dd16d841cdf298a/crypto/merkle/merkle.proto#L26>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ProofOps {
    /// The list of ProofOps
    pub ops: Vec<ProofOp>,
}

/// ProofOp defines an operation used for calculating Merkle root
/// The data could be arbitrary format, providing necessary data
/// for example neighbouring node hash
/// <https://github.com/tendermint/tendermint/blob/c8483531d8e756f7fbb812db1dd16d841cdf298a/crypto/merkle/merkle.proto#L19>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ProofOp {
    /// Type of the ProofOp
    #[serde(alias = "type")]
    pub field_type: String,
    /// Key of the ProofOp
    #[serde(default, with = "serializers::bytes::base64string")]
    pub key: Vec<u8>,
    /// Actual data
    #[serde(default, with = "serializers::bytes::base64string")]
    pub data: Vec<u8>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

tendermint_pb_modules! {
    use super::{Proof, ProofOp, ProofOps};
    use crate::{prelude::*, Error};
    use pb::{
        crypto::{Proof as RawProof, ProofOp as RawProofOp, ProofOps as RawProofOps},
    };

    impl Protobuf<RawProof> for Proof {}

    impl TryFrom<RawProof> for Proof {
        type Error = Error;

        fn try_from(message: RawProof) -> Result<Self, Self::Error> {
            Ok(Self {
                total: message
                    .total
                    .try_into()
                    .map_err(Error::negative_proof_total)?,
                index: message
                    .index
                    .try_into()
                    .map_err(Error::negative_proof_index)?,
                leaf_hash: message.leaf_hash.try_into()?,
                aunts: message
                    .aunts
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
            })
        }
    }

    impl From<Proof> for RawProof {
        fn from(value: Proof) -> Self {
            Self {
                total: value
                    .total
                    .try_into()
                    .expect("number of items is too large"),
                index: value.index.try_into().expect("index is too large"),
                leaf_hash: value.leaf_hash.into(),
                aunts: value.aunts.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl Protobuf<RawProofOp> for ProofOp {}

    impl TryFrom<RawProofOp> for ProofOp {
        type Error = Error;

        fn try_from(value: RawProofOp) -> Result<Self, Self::Error> {
            Ok(Self {
                field_type: value.r#type,
                key: value.key,
                data: value.data,
            })
        }
    }

    impl From<ProofOp> for RawProofOp {
        fn from(value: ProofOp) -> Self {
            RawProofOp {
                r#type: value.field_type,
                key: value.key,
                data: value.data,
            }
        }
    }

    impl Protobuf<RawProofOps> for ProofOps {}

    impl TryFrom<RawProofOps> for ProofOps {
        type Error = Error;

        fn try_from(value: RawProofOps) -> Result<Self, Self::Error> {
            let ops: Result<Vec<ProofOp>, _> = value.ops.into_iter().map(ProofOp::try_from).collect();

            Ok(Self { ops: ops? })
        }
    }

    impl From<ProofOps> for RawProofOps {
        fn from(value: ProofOps) -> Self {
            let ops: Vec<RawProofOp> = value.ops.into_iter().map(RawProofOp::from).collect();

            RawProofOps { ops }
        }
    }
}

#[cfg(test)]
mod test {
    use super::ProofOps;
    use crate::test::test_serialization_roundtrip;

    #[test]
    fn serialization_roundtrip() {
        let payload = r#"
        {
            "ops": [
                {
                    "type": "iavl:v",
                    "key": "Y29uc2Vuc3VzU3RhdGUvaWJjb25lY2xpZW50LzIy",
                    "data": "8QEK7gEKKAgIEAwYHCIgG9RAkJgHlxNjmyzOW6bUAidhiRSja0x6+GXCVENPG1oKKAgGEAUYFyIgwRns+dJvjf1Zk2BaFrXz8inPbvYHB7xx2HCy9ima5f8KKAgEEAMYFyogOr8EGajEV6fG5fzJ2fAAvVMgRLhdMJTzCPlogl9rxlIKKAgCEAIYFyIgcjzX/a+2bFbnNldpawQqZ+kYhIwz5r4wCUzuu1IFW04aRAoeY29uc2Vuc3VzU3RhdGUvaWJjb25lY2xpZW50LzIyEiAZ1uuG60K4NHJZZMuS9QX6o4eEhica5jIHYwflRiYkDBgX"
                },
                {
                    "type": "multistore",
                    "key": "aWJj",
                    "data": "CvEECjAKBGJhbmsSKAomCIjYAxIg2MEyyonbZButYnvSRkf2bPQg+nqA+Am1MeDxG6F4p1UKLwoDYWNjEigKJgiI2AMSIN2YHczeuXNvyetrSFQpkCcJzfB6PXVCw0i/XShMgPnIChEKB3VwZ3JhZGUSBgoECIjYAwovCgNnb3YSKAomCIjYAxIgYM0TfBli7KxhY4nWgDSDPykhUJwtKFql9RU5l86WinQKLwoDaWJjEigKJgiI2AMSIFp6aJASeInQKF8y824zjmgcFORN6M+ECbgFfJkobKs8CjAKBG1haW4SKAomCIjYAxIgsZzwmLQ7PH1UeZ/vCUSqlQmfgt3CGfoMgJLkUqKCv0EKMwoHc3Rha2luZxIoCiYIiNgDEiCiBZoBLyDGj5euy3n33ik+SpqYK9eB5xbI+iY8ycYVbwo0CghzbGFzaGluZxIoCiYIiNgDEiAJz3gEYuIhdensHU3b5qH5ons2quepd6EaRgCHXab6PQoyCgZzdXBwbHkSKAomCIjYAxIglWLA5/THPTiTxAlaLHOBYFIzEJTmKPznItUwAc8zD+AKEgoIZXZpZGVuY2USBgoECIjYAwowCgRtaW50EigKJgiI2AMSIMS8dZ1j8F6JVVv+hB1rHBZC+gIFJxHan2hM8qDC64n/CjIKBnBhcmFtcxIoCiYIiNgDEiB8VIzExUHX+SvHZFz/P9NM9THnw/gTDDLVReuZX8htLgo4CgxkaXN0cmlidXRpb24SKAomCIjYAxIg3u/Nd4L+8LT8OXJCh14o8PHIJ/GLQwsmE7KYIl1GdSYKEgoIdHJhbnNmZXISBgoECIjYAw=="
                }
            ]
        }"#;
        test_serialization_roundtrip::<ProofOps>(payload);
    }
}
