//! CometBFT consensus parameters

use serde::{Deserialize, Serialize};

use crate::{
    block, duration::Duration, evidence, prelude::*, public_key,
    serializers::allow_empty_object::allow_empty_object,
};

/// All consensus-relevant parameters that can be adjusted by the ABCI app.
///
/// [ABCI documentation](https://docs.cometbft.com/v1/spec/abci/abci.html#consensusparams)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Params {
    /// Parameters limiting the size of a block and time between consecutive blocks.
    pub block: block::Size,
    /// Parameters limiting the validity of evidence of byzantine behaviour.
    pub evidence: evidence::Params,
    /// Parameters limiting the types of public keys validators can use.
    pub validator: ValidatorParams,
    /// The ABCI application version. Will default to None if not present.
    #[serde(default, deserialize_with = "allow_empty_object")]
    //#[serde(skip)]
    pub version: Option<VersionParams>,
    /// Parameters specific to the Application Blockchain Interface.
    ///
    /// This field has been added in CometBFT 0.38 and will be ignored when
    /// encoding into earlier protocol versions.
    ///
    /// From CometBFT v1.0.0 onwards, use `FeatureParams.vote_extensions_enable_height` instead.
    #[serde(default)]
    pub abci: AbciParams,
    /// Parameters for Proposer-Based Timestamps (PBTS).
    ///
    /// This field has been added in CometBFT 1.0.0 and will be ignored when
    /// encoding into earlier protocol versions.
    pub synchrony: Option<SynchronyParams>,
    /// Parameters for enabling specific features.
    ///
    /// This field has been added in CometBFT 1.0.0 and will be ignored when
    /// encoding into earlier protocol versions.
    pub feature: Option<FeatureParams>,
}

/// ValidatorParams restrict the public key types validators can use.
///
/// NOTE: uses ABCI public keys naming, not Amino names.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ValidatorParams {
    /// List of accepted public key types.
    pub pub_key_types: Vec<public_key::Algorithm>,
}

/// VersionParams contain the version of specific components of CometBFT.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct VersionParams {
    /// The ABCI application version.
    #[serde(with = "crate::serializers::from_str", alias = "app_version")]
    pub app: u64,
}

/// ABCIParams is deprecated and its contents moved to FeatureParams
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct AbciParams {
    /// vote_extensions_enable_height has been deprecated.
    /// Instead, use FeatureParams.vote_extensions_enable_height.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vote_extensions_enable_height: Option<block::Height>,
}

/// SynchronyParams determine the validity of block timestamps.
///
/// These parameters are part of the Proposer-Based Timestamps (PBTS) algorithm.
/// For more information on the relationship of the synchrony parameters to
/// block timestamps validity, refer to the [PBTS specification][pbts].
///
/// [pbts]: https://github.com/cometbft/cometbft/blob/main/spec/consensus/proposer-based-timestamp/README.md
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct SynchronyParams {
    /// Bound for how skewed a proposer's clock may be from any validator on the
    /// network while still producing valid proposals.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<Duration>,
    /// Bound for how long a proposal message may take to reach all validators on
    /// a network and still be considered valid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_delay: Option<Duration>,
}

/// FeatureParams configure the height from which features of CometBFT are enabled.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct FeatureParams {
    /// Height during which vote extensions will be enabled.
    ///
    /// A value of 0 means vote extensions are disabled. A value > 0 denotes
    /// the height at which vote extensions will be (or have been) enabled.
    ///
    /// During the specified height, and for all subsequent heights, precommit
    /// messages that do not contain valid extension data will be considered
    /// invalid. Prior to this height, or when this height is set to 0, vote
    /// extensions will not be used or accepted by validators on the network.
    ///
    /// Once enabled, vote extensions will be created by the application in
    /// ExtendVote, validated by the application in VerifyVoteExtension, and
    /// used by the application in PrepareProposal, when proposing the next block.
    ///
    /// Cannot be set to heights lower or equal to the current blockchain height.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vote_extensions_enable_height: Option<block::Height>,
    /// Height at which Proposer-Based Timestamps (PBTS) will be enabled.
    ///
    /// A value of 0 means PBTS is disabled. A value > 0 denotes the height at
    /// which PBTS will be (or has been) enabled.
    ///
    /// From the specified height, and for all subsequent heights, the PBTS
    /// algorithm will be used to produce and validate block timestamps. Prior to
    /// this height, or when this height is set to 0, the legacy BFT Time
    /// algorithm is used to produce and validate timestamps.
    ///
    /// Cannot be set to heights lower or equal to the current blockchain height.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pbts_enable_height: Option<block::Height>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

// Todo: How are these key types created?
fn key_type(s: &str) -> public_key::Algorithm {
    if s == "Ed25519" || s == "ed25519" {
        return public_key::Algorithm::Ed25519;
    }
    if s == "Secp256k1" || s == "secp256k1" {
        return public_key::Algorithm::Secp256k1;
    }
    public_key::Algorithm::Ed25519 // Todo: Shall we error out for invalid key types?
}

mod v0_34 {
    use cometbft_proto::v0_34::{
        abci::ConsensusParams as RawAbciConsensusParams,
        types::{
            ConsensusParams as RawParams, ValidatorParams as RawValidatorParams,
            VersionParams as RawVersionParams,
        },
    };
    use cometbft_proto::Protobuf;

    use super::{key_type, Params, ValidatorParams, VersionParams};
    use crate::{error::Error, prelude::*, public_key};

    impl Protobuf<RawParams> for Params {}

    impl TryFrom<RawParams> for Params {
        type Error = Error;

        fn try_from(value: RawParams) -> Result<Self, Self::Error> {
            Ok(Self {
                block: value
                    .block
                    .ok_or_else(|| Error::invalid_block("missing block".to_string()))?
                    .try_into()?,
                evidence: value
                    .evidence
                    .ok_or_else(Error::invalid_evidence)?
                    .try_into()?,
                validator: value
                    .validator
                    .ok_or_else(Error::invalid_validator_params)?
                    .try_into()?,
                version: value.version.map(TryFrom::try_from).transpose()?,
                abci: Default::default(),
                synchrony: None,
                feature: None,
            })
        }
    }

    impl From<Params> for RawParams {
        fn from(value: Params) -> Self {
            RawParams {
                block: Some(value.block.into()),
                evidence: Some(value.evidence.into()),
                validator: Some(value.validator.into()),
                version: value.version.map(From::from),
            }
        }
    }

    impl Protobuf<RawAbciConsensusParams> for Params {}

    impl TryFrom<RawAbciConsensusParams> for Params {
        type Error = Error;

        fn try_from(value: RawAbciConsensusParams) -> Result<Self, Self::Error> {
            Ok(Self {
                block: value
                    .block
                    .ok_or_else(|| Error::invalid_block("missing block".to_string()))?
                    .try_into()?,
                evidence: value
                    .evidence
                    .ok_or_else(Error::invalid_evidence)?
                    .try_into()?,
                validator: value
                    .validator
                    .ok_or_else(Error::invalid_validator_params)?
                    .try_into()?,
                version: value.version.map(TryFrom::try_from).transpose()?,
                abci: Default::default(),
                synchrony: None,
                feature: None,
            })
        }
    }

    impl From<Params> for RawAbciConsensusParams {
        fn from(value: Params) -> Self {
            RawAbciConsensusParams {
                block: Some(value.block.into()),
                evidence: Some(value.evidence.into()),
                validator: Some(value.validator.into()),
                version: value.version.map(From::from),
            }
        }
    }

    impl Protobuf<RawValidatorParams> for ValidatorParams {}

    impl TryFrom<RawValidatorParams> for ValidatorParams {
        type Error = Error;

        fn try_from(value: RawValidatorParams) -> Result<Self, Self::Error> {
            Ok(Self {
                pub_key_types: value.pub_key_types.iter().map(|f| key_type(f)).collect(),
            })
        }
    }

    impl From<ValidatorParams> for RawValidatorParams {
        fn from(value: ValidatorParams) -> Self {
            RawValidatorParams {
                pub_key_types: value
                    .pub_key_types
                    .into_iter()
                    .map(|k| match k {
                        public_key::Algorithm::Ed25519 => "ed25519".to_string(),
                        public_key::Algorithm::Secp256k1 => "secp256k1".to_string(),
                    })
                    .collect(),
            }
        }
    }

    impl Protobuf<RawVersionParams> for VersionParams {}

    impl TryFrom<RawVersionParams> for VersionParams {
        type Error = Error;

        fn try_from(value: RawVersionParams) -> Result<Self, Self::Error> {
            Ok(Self {
                app: value.app_version,
            })
        }
    }

    impl From<VersionParams> for RawVersionParams {
        fn from(value: VersionParams) -> Self {
            RawVersionParams {
                app_version: value.app,
            }
        }
    }
}

mod v0_37 {
    use cometbft_proto::v0_37::types::{
        ConsensusParams as RawParams, ValidatorParams as RawValidatorParams,
        VersionParams as RawVersionParams,
    };
    use cometbft_proto::Protobuf;

    use super::{key_type, Params, ValidatorParams, VersionParams};
    use crate::{error::Error, prelude::*, public_key};

    impl Protobuf<RawParams> for Params {}

    impl TryFrom<RawParams> for Params {
        type Error = Error;

        fn try_from(value: RawParams) -> Result<Self, Self::Error> {
            Ok(Self {
                block: value
                    .block
                    .ok_or_else(|| Error::invalid_block("missing block".to_string()))?
                    .try_into()?,
                evidence: value
                    .evidence
                    .ok_or_else(Error::invalid_evidence)?
                    .try_into()?,
                validator: value
                    .validator
                    .ok_or_else(Error::invalid_validator_params)?
                    .try_into()?,
                version: value.version.map(TryFrom::try_from).transpose()?,
                abci: Default::default(),
                synchrony: None,
                feature: None,
            })
        }
    }

    impl From<Params> for RawParams {
        fn from(value: Params) -> Self {
            RawParams {
                block: Some(value.block.into()),
                evidence: Some(value.evidence.into()),
                validator: Some(value.validator.into()),
                version: value.version.map(From::from),
            }
        }
    }

    impl Protobuf<RawValidatorParams> for ValidatorParams {}

    impl TryFrom<RawValidatorParams> for ValidatorParams {
        type Error = Error;

        fn try_from(value: RawValidatorParams) -> Result<Self, Self::Error> {
            Ok(Self {
                pub_key_types: value.pub_key_types.iter().map(|f| key_type(f)).collect(),
            })
        }
    }

    impl From<ValidatorParams> for RawValidatorParams {
        fn from(value: ValidatorParams) -> Self {
            RawValidatorParams {
                pub_key_types: value
                    .pub_key_types
                    .into_iter()
                    .map(|k| match k {
                        public_key::Algorithm::Ed25519 => "ed25519".to_string(),
                        public_key::Algorithm::Secp256k1 => "secp256k1".to_string(),
                    })
                    .collect(),
            }
        }
    }

    impl Protobuf<RawVersionParams> for VersionParams {}

    impl TryFrom<RawVersionParams> for VersionParams {
        type Error = Error;

        fn try_from(value: RawVersionParams) -> Result<Self, Self::Error> {
            Ok(Self { app: value.app })
        }
    }

    impl From<VersionParams> for RawVersionParams {
        fn from(value: VersionParams) -> Self {
            RawVersionParams { app: value.app }
        }
    }
}

mod v0_38 {
    use cometbft_proto::v0_38::types::{
        AbciParams as RawAbciParams, ConsensusParams as RawParams,
        ValidatorParams as RawValidatorParams, VersionParams as RawVersionParams,
    };
    use cometbft_proto::Protobuf;

    use super::{key_type, AbciParams, Params, ValidatorParams, VersionParams};
    use crate::{error::Error, prelude::*, public_key};

    impl Protobuf<RawParams> for Params {}

    impl TryFrom<RawParams> for Params {
        type Error = Error;

        fn try_from(value: RawParams) -> Result<Self, Self::Error> {
            Ok(Self {
                block: value
                    .block
                    .ok_or_else(|| Error::invalid_block("missing block".to_string()))?
                    .try_into()?,
                evidence: value
                    .evidence
                    .ok_or_else(Error::invalid_evidence)?
                    .try_into()?,
                validator: value
                    .validator
                    .ok_or_else(Error::invalid_validator_params)?
                    .try_into()?,
                version: value.version.map(TryFrom::try_from).transpose()?,
                abci: value
                    .abci
                    .map(TryFrom::try_from)
                    .transpose()?
                    .unwrap_or_default(),
                synchrony: None,
                feature: None,
            })
        }
    }

    impl From<Params> for RawParams {
        fn from(value: Params) -> Self {
            RawParams {
                block: Some(value.block.into()),
                evidence: Some(value.evidence.into()),
                validator: Some(value.validator.into()),
                version: value.version.map(From::from),
                abci: Some(value.abci.into()),
            }
        }
    }

    impl Protobuf<RawValidatorParams> for ValidatorParams {}

    impl TryFrom<RawValidatorParams> for ValidatorParams {
        type Error = Error;

        fn try_from(value: RawValidatorParams) -> Result<Self, Self::Error> {
            Ok(Self {
                pub_key_types: value.pub_key_types.iter().map(|f| key_type(f)).collect(),
            })
        }
    }

    impl From<ValidatorParams> for RawValidatorParams {
        fn from(value: ValidatorParams) -> Self {
            RawValidatorParams {
                pub_key_types: value
                    .pub_key_types
                    .into_iter()
                    .map(|k| match k {
                        public_key::Algorithm::Ed25519 => "ed25519".to_string(),
                        public_key::Algorithm::Secp256k1 => "secp256k1".to_string(),
                    })
                    .collect(),
            }
        }
    }

    impl Protobuf<RawVersionParams> for VersionParams {}

    impl TryFrom<RawVersionParams> for VersionParams {
        type Error = Error;

        fn try_from(value: RawVersionParams) -> Result<Self, Self::Error> {
            Ok(Self { app: value.app })
        }
    }

    impl From<VersionParams> for RawVersionParams {
        fn from(value: VersionParams) -> Self {
            RawVersionParams { app: value.app }
        }
    }

    impl Protobuf<RawAbciParams> for AbciParams {}

    impl TryFrom<RawAbciParams> for AbciParams {
        type Error = Error;

        fn try_from(message: RawAbciParams) -> Result<Self, Self::Error> {
            let vote_extensions_enable_height = match message.vote_extensions_enable_height {
                0 => None,
                h => Some(h.try_into()?),
            };
            Ok(Self {
                vote_extensions_enable_height,
            })
        }
    }

    impl From<AbciParams> for RawAbciParams {
        fn from(value: AbciParams) -> Self {
            Self {
                vote_extensions_enable_height: value
                    .vote_extensions_enable_height
                    .map_or(0, Into::into),
            }
        }
    }
}

mod v1beta1 {
    use cometbft_proto::abci::v1beta1::ConsensusParams as RawAbciConsensusParams;
    use cometbft_proto::types::v1beta1::{
        ConsensusParams as RawParams, ValidatorParams as RawValidatorParams,
        VersionParams as RawVersionParams,
    };

    use super::{key_type, Params, ValidatorParams, VersionParams};
    use crate::{error::Error, prelude::*, public_key};

    impl TryFrom<RawParams> for Params {
        type Error = Error;

        fn try_from(value: RawParams) -> Result<Self, Self::Error> {
            Ok(Self {
                block: value
                    .block
                    .ok_or_else(|| Error::invalid_block("missing block".to_string()))?
                    .try_into()?,
                evidence: value
                    .evidence
                    .ok_or_else(Error::invalid_evidence)?
                    .try_into()?,
                validator: value
                    .validator
                    .ok_or_else(Error::invalid_validator_params)?
                    .try_into()?,
                version: value.version.map(TryFrom::try_from).transpose()?,
                abci: Default::default(),
                synchrony: None,
                feature: None,
            })
        }
    }

    impl From<Params> for RawParams {
        fn from(value: Params) -> Self {
            RawParams {
                block: Some(value.block.into()),
                evidence: Some(value.evidence.into()),
                validator: Some(value.validator.into()),
                version: value.version.map(From::from),
            }
        }
    }

    impl TryFrom<RawAbciConsensusParams> for Params {
        type Error = Error;

        fn try_from(value: RawAbciConsensusParams) -> Result<Self, Self::Error> {
            Ok(Self {
                block: value
                    .block
                    .ok_or_else(|| Error::invalid_block("missing block".to_string()))?
                    .try_into()?,
                evidence: value
                    .evidence
                    .ok_or_else(Error::invalid_evidence)?
                    .try_into()?,
                validator: value
                    .validator
                    .ok_or_else(Error::invalid_validator_params)?
                    .try_into()?,
                version: value.version.map(TryFrom::try_from).transpose()?,
                abci: Default::default(),
                synchrony: None,
                feature: None,
            })
        }
    }

    impl From<Params> for RawAbciConsensusParams {
        fn from(value: Params) -> Self {
            RawAbciConsensusParams {
                block: Some(value.block.into()),
                evidence: Some(value.evidence.into()),
                validator: Some(value.validator.into()),
                version: value.version.map(From::from),
            }
        }
    }

    impl TryFrom<RawValidatorParams> for ValidatorParams {
        type Error = Error;

        fn try_from(value: RawValidatorParams) -> Result<Self, Self::Error> {
            Ok(Self {
                pub_key_types: value.pub_key_types.iter().map(|f| key_type(f)).collect(),
            })
        }
    }

    impl From<ValidatorParams> for RawValidatorParams {
        fn from(value: ValidatorParams) -> Self {
            RawValidatorParams {
                pub_key_types: value
                    .pub_key_types
                    .into_iter()
                    .map(|k| match k {
                        public_key::Algorithm::Ed25519 => "ed25519".to_string(),
                        public_key::Algorithm::Secp256k1 => "secp256k1".to_string(),
                    })
                    .collect(),
            }
        }
    }

    impl TryFrom<RawVersionParams> for VersionParams {
        type Error = Error;

        fn try_from(value: RawVersionParams) -> Result<Self, Self::Error> {
            Ok(Self { app: value.app })
        }
    }

    impl From<VersionParams> for RawVersionParams {
        fn from(value: VersionParams) -> Self {
            RawVersionParams { app: value.app }
        }
    }
}

mod v1beta2 {
    use cometbft_proto::types::v1beta2::ConsensusParams as RawParams;

    use super::Params;
    use crate::{error::Error, prelude::*};

    impl TryFrom<RawParams> for Params {
        type Error = Error;

        fn try_from(value: RawParams) -> Result<Self, Self::Error> {
            Ok(Self {
                block: value
                    .block
                    .ok_or_else(|| Error::invalid_block("missing block".to_string()))?
                    .try_into()?,
                evidence: value
                    .evidence
                    .ok_or_else(Error::invalid_evidence)?
                    .try_into()?,
                validator: value
                    .validator
                    .ok_or_else(Error::invalid_validator_params)?
                    .try_into()?,
                version: value.version.map(TryFrom::try_from).transpose()?,
                abci: Default::default(),
                synchrony: None,
                feature: None,
            })
        }
    }

    impl From<Params> for RawParams {
        fn from(value: Params) -> Self {
            RawParams {
                block: Some(value.block.into()),
                evidence: Some(value.evidence.into()),
                validator: Some(value.validator.into()),
                version: value.version.map(From::from),
            }
        }
    }
}

mod v1 {
    use cometbft_proto::types::v1::{
        AbciParams as RawAbciParams, ConsensusParams as RawParams,
        FeatureParams as RawFeatureParams, SynchronyParams as RawSynchronyParams,
        ValidatorParams as RawValidatorParams, VersionParams as RawVersionParams,
    };

    use super::{
        key_type, AbciParams, FeatureParams, Params, SynchronyParams, ValidatorParams,
        VersionParams,
    };
    use crate::{error::Error, prelude::*, public_key};

    impl TryFrom<RawParams> for Params {
        type Error = Error;

        fn try_from(value: RawParams) -> Result<Self, Self::Error> {
            Ok(Self {
                block: value
                    .block
                    .ok_or_else(|| Error::invalid_block("missing block".to_string()))?
                    .try_into()?,
                evidence: value
                    .evidence
                    .ok_or_else(Error::invalid_evidence)?
                    .try_into()?,
                validator: value
                    .validator
                    .ok_or_else(Error::invalid_validator_params)?
                    .try_into()?,
                version: value.version.map(TryFrom::try_from).transpose()?,
                abci: Default::default(), // AbciParams is deprecated in v1
                synchrony: value.synchrony.map(TryFrom::try_from).transpose()?,
                feature: value.feature.map(TryFrom::try_from).transpose()?,
            })
        }
    }

    impl From<Params> for RawParams {
        fn from(value: Params) -> Self {
            #[allow(deprecated)]
            RawParams {
                block: Some(value.block.into()),
                evidence: Some(value.evidence.into()),
                validator: Some(value.validator.into()),
                version: value.version.map(From::from),
                abci: None, // AbciParams is deprecated in v1
                synchrony: value.synchrony.map(From::from),
                feature: value.feature.map(From::from),
            }
        }
    }

    impl TryFrom<RawValidatorParams> for ValidatorParams {
        type Error = Error;

        fn try_from(value: RawValidatorParams) -> Result<Self, Self::Error> {
            Ok(Self {
                pub_key_types: value.pub_key_types.iter().map(|f| key_type(f)).collect(),
            })
        }
    }

    impl From<ValidatorParams> for RawValidatorParams {
        fn from(value: ValidatorParams) -> Self {
            RawValidatorParams {
                pub_key_types: value
                    .pub_key_types
                    .into_iter()
                    .map(|k| match k {
                        public_key::Algorithm::Ed25519 => "ed25519".to_string(),
                        public_key::Algorithm::Secp256k1 => "secp256k1".to_string(),
                    })
                    .collect(),
            }
        }
    }

    impl TryFrom<RawVersionParams> for VersionParams {
        type Error = Error;

        fn try_from(value: RawVersionParams) -> Result<Self, Self::Error> {
            Ok(Self { app: value.app })
        }
    }

    impl From<VersionParams> for RawVersionParams {
        fn from(value: VersionParams) -> Self {
            RawVersionParams { app: value.app }
        }
    }

    impl TryFrom<RawAbciParams> for AbciParams {
        type Error = Error;

        fn try_from(message: RawAbciParams) -> Result<Self, Self::Error> {
            let vote_extensions_enable_height = match message.vote_extensions_enable_height {
                0 => None,
                h => Some(h.try_into()?),
            };
            Ok(Self {
                vote_extensions_enable_height,
            })
        }
    }

    impl From<AbciParams> for RawAbciParams {
        fn from(value: AbciParams) -> Self {
            Self {
                vote_extensions_enable_height: value
                    .vote_extensions_enable_height
                    .map_or(0, Into::into),
            }
        }
    }

    impl TryFrom<RawSynchronyParams> for SynchronyParams {
        type Error = Error;

        fn try_from(value: RawSynchronyParams) -> Result<Self, Self::Error> {
            Ok(Self {
                precision: value.precision.map(TryFrom::try_from).transpose()?,
                message_delay: value.message_delay.map(TryFrom::try_from).transpose()?,
            })
        }
    }

    impl From<SynchronyParams> for RawSynchronyParams {
        fn from(value: SynchronyParams) -> Self {
            RawSynchronyParams {
                precision: value.precision.map(Into::into),
                message_delay: value.message_delay.map(Into::into),
            }
        }
    }

    impl TryFrom<RawFeatureParams> for FeatureParams {
        type Error = Error;

        fn try_from(value: RawFeatureParams) -> Result<Self, Self::Error> {
            Ok(Self {
                vote_extensions_enable_height: value
                    .vote_extensions_enable_height
                    .map(TryFrom::try_from)
                    .transpose()?,
                pbts_enable_height: value
                    .pbts_enable_height
                    .map(TryFrom::try_from)
                    .transpose()?,
            })
        }
    }

    impl From<FeatureParams> for RawFeatureParams {
        fn from(value: FeatureParams) -> Self {
            RawFeatureParams {
                vote_extensions_enable_height: value.vote_extensions_enable_height.map(Into::into),
                pbts_enable_height: value.pbts_enable_height.map(Into::into),
            }
        }
    }
}
