//! Evidence of malfeasance by validators (i.e. signing conflicting votes).

use core::slice;

use serde::{Deserialize, Serialize};

use crate::{
    block::{signed_header::SignedHeader, Height},
    duration::Duration,
    error::Error,
    prelude::*,
    serializers, validator,
    vote::Power,
    Time, Vote,
};

/// Evidence of malfeasance by validators (i.e. signing conflicting votes or light client attack).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Evidence {
    /// Duplicate vote evidence
    DuplicateVote(Box<DuplicateVoteEvidence>),

    /// LightClient attack evidence
    LightClientAttack(Box<LightClientAttackEvidence>),
}

impl From<LightClientAttackEvidence> for Evidence {
    fn from(ev: LightClientAttackEvidence) -> Self {
        Self::LightClientAttack(Box::new(ev))
    }
}

impl From<DuplicateVoteEvidence> for Evidence {
    fn from(ev: DuplicateVoteEvidence) -> Self {
        Self::DuplicateVote(Box::new(ev))
    }
}

/// Duplicate vote evidence
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DuplicateVoteEvidence {
    pub vote_a: Vote,
    pub vote_b: Vote,
    pub total_voting_power: Power,
    pub validator_power: Power,
    pub timestamp: Time,
}

impl DuplicateVoteEvidence {
    /// constructor
    pub fn new(vote_a: Vote, vote_b: Vote) -> Result<Self, Error> {
        if vote_a.height != vote_b.height {
            return Err(Error::invalid_evidence());
        }

        // Todo: make more assumptions about what is considered a valid evidence for duplicate vote
        Ok(Self {
            vote_a,
            vote_b,
            total_voting_power: Default::default(),
            validator_power: Default::default(),
            timestamp: Time::unix_epoch(),
        })
    }

    /// Get votes
    pub fn votes(&self) -> (&Vote, &Vote) {
        (&self.vote_a, &self.vote_b)
    }
}

/// Conflicting block detected in light client attack
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictingBlock {
    pub signed_header: SignedHeader,
    pub validator_set: validator::Set,
}

/// Light client attack evidence
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightClientAttackEvidence {
    pub conflicting_block: ConflictingBlock,
    pub common_height: Height,
    pub byzantine_validators: Vec<validator::Info>,
    pub total_voting_power: Power,
    pub timestamp: Time,
}

/// A list of `Evidence`.
///
/// <https://github.com/tendermint/spec/blob/d46cd7f573a2c6a2399fcab2cde981330aa63f37/spec/core/data_structures.md#evidencedata>
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct List(Vec<Evidence>);

impl List {
    /// Create a new evidence data collection
    pub fn new<I>(into_evidence: I) -> List
    where
        I: Into<Vec<Evidence>>,
    {
        List(into_evidence.into())
    }

    /// Convert this evidence data into a vector
    pub fn into_vec(self) -> Vec<Evidence> {
        self.0
    }

    /// Iterate over the evidence data
    pub fn iter(&self) -> slice::Iter<'_, Evidence> {
        self.0.iter()
    }
}

impl AsRef<[Evidence]> for List {
    fn as_ref(&self) -> &[Evidence] {
        &self.0
    }
}

/// EvidenceParams determine how we handle evidence of malfeasance.
///
/// [CometBFT documentation](https://docs.cometbft.com/v1/spec/core/data_structures.html#evidenceparams)
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Params {
    /// Maximum age of evidence, in blocks.
    ///
    /// The recommended formula for calculating it is max_age_duration / {average
    /// block time}.
    #[serde(with = "serializers::from_str")]
    pub max_age_num_blocks: u64,

    /// Max age of evidence, in time.
    ///
    /// The recommended value of is should correspond to the application's
    /// "unbonding period" or other similar mechanism for handling
    /// [Nothing-At-Stake][nas] attacks.
    ///
    /// [nas]: https://github.com/ethereum/wiki/wiki/Proof-of-Stake-FAQ#what-is-the-nothing-at-stake-problem-and-how-can-it-be-fixed
    pub max_age_duration: Duration,

    /// Maximum size in bytes of evidence allowed to be included in a block.
    ///
    /// It should fall comfortably under the maximum size of a block.
    #[serde(with = "serializers::from_str", default)]
    pub max_bytes: i64,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

cometbft_old_pb_modules! {
    use pb::types as raw;

    use super::{List, LightClientAttackEvidence, DuplicateVoteEvidence, ConflictingBlock, Evidence, Params};
    use crate::{error::Error, prelude::*};

    impl Protobuf<raw::Evidence> for Evidence {}

    impl TryFrom<raw::Evidence> for Evidence {
        type Error = Error;

        fn try_from(value: raw::Evidence) -> Result<Self, Self::Error> {
            match value.sum.ok_or_else(Error::invalid_evidence)? {
                raw::evidence::Sum::DuplicateVoteEvidence(ev) => {
                    Ok(Evidence::DuplicateVote(Box::new(ev.try_into()?)))
                },
                raw::evidence::Sum::LightClientAttackEvidence(ev) => {
                    Ok(Evidence::LightClientAttack(Box::new(ev.try_into()?)))
                },
            }
        }
    }

    impl From<Evidence> for raw::Evidence {
        fn from(value: Evidence) -> Self {
            match value {
                Evidence::DuplicateVote(ev) => raw::Evidence {
                    sum: Some(raw::evidence::Sum::DuplicateVoteEvidence((*ev).into())),
                },
                Evidence::LightClientAttack(ev) => raw::Evidence {
                    sum: Some(raw::evidence::Sum::LightClientAttackEvidence((*ev).into())),
                },
            }
        }
    }

    impl Protobuf<raw::DuplicateVoteEvidence> for DuplicateVoteEvidence {}

    impl TryFrom<raw::DuplicateVoteEvidence> for DuplicateVoteEvidence {
        type Error = Error;

        fn try_from(value: raw::DuplicateVoteEvidence) -> Result<Self, Self::Error> {
            Ok(Self {
                vote_a: value
                    .vote_a
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
                vote_b: value
                    .vote_b
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
                total_voting_power: value.total_voting_power.try_into()?,
                validator_power: value.validator_power.try_into()?,
                timestamp: value
                    .timestamp
                    .ok_or_else(Error::missing_timestamp)?
                    .try_into()?,
            })
        }
    }

    impl From<DuplicateVoteEvidence> for raw::DuplicateVoteEvidence {
        fn from(value: DuplicateVoteEvidence) -> Self {
            raw::DuplicateVoteEvidence {
                vote_a: Some(value.vote_a.into()),
                vote_b: Some(value.vote_b.into()),
                total_voting_power: value.total_voting_power.into(),
                validator_power: value.total_voting_power.into(),
                timestamp: Some(value.timestamp.into()),
            }
        }
    }

    impl Protobuf<raw::LightBlock> for ConflictingBlock {}

    impl TryFrom<raw::LightBlock> for ConflictingBlock {
        type Error = Error;

        fn try_from(value: raw::LightBlock) -> Result<Self, Self::Error> {
            Ok(ConflictingBlock {
                signed_header: value
                    .signed_header
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
                validator_set: value
                    .validator_set
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
            })
        }
    }

    impl From<ConflictingBlock> for raw::LightBlock {
        fn from(value: ConflictingBlock) -> Self {
            raw::LightBlock {
                signed_header: Some(value.signed_header.into()),
                validator_set: Some(value.validator_set.into()),
            }
        }
    }

    impl Protobuf<raw::LightClientAttackEvidence> for LightClientAttackEvidence {}

    impl TryFrom<raw::LightClientAttackEvidence> for LightClientAttackEvidence {
        type Error = Error;

        fn try_from(ev: raw::LightClientAttackEvidence) -> Result<Self, Self::Error> {
            Ok(LightClientAttackEvidence {
                conflicting_block: ev
                    .conflicting_block
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
                common_height: ev.common_height.try_into()?,
                byzantine_validators: ev
                    .byzantine_validators
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?,
                total_voting_power: ev.total_voting_power.try_into()?,
                timestamp: ev
                    .timestamp
                    .ok_or_else(Error::missing_timestamp)?
                    .try_into()?,
            })
        }
    }

    impl From<LightClientAttackEvidence> for raw::LightClientAttackEvidence {
        fn from(ev: LightClientAttackEvidence) -> Self {
            raw::LightClientAttackEvidence {
                conflicting_block: Some(ev.conflicting_block.into()),
                common_height: ev.common_height.into(),
                byzantine_validators: ev
                    .byzantine_validators
                    .into_iter()
                    .map(Into::into)
                    .collect(),
                total_voting_power: ev.total_voting_power.into(),
                timestamp: Some(ev.timestamp.into()),
            }
        }
    }

    impl Protobuf<raw::EvidenceList> for List {}

    impl TryFrom<raw::EvidenceList> for List {
        type Error = Error;
        fn try_from(value: raw::EvidenceList) -> Result<Self, Self::Error> {
            let evidence = value
                .evidence
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Self(evidence))
        }
    }

    impl From<List> for raw::EvidenceList {
        fn from(value: List) -> Self {
            raw::EvidenceList {
                evidence: value.0.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl Protobuf<raw::EvidenceParams> for Params {}

    impl TryFrom<raw::EvidenceParams> for Params {
        type Error = Error;

        fn try_from(value: raw::EvidenceParams) -> Result<Self, Self::Error> {
            Ok(Self {
                max_age_num_blocks: value
                    .max_age_num_blocks
                    .try_into()
                    .map_err(Error::negative_max_age_num)?,
                max_age_duration: value
                    .max_age_duration
                    .ok_or_else(Error::missing_max_age_duration)?
                    .try_into()?,
                max_bytes: value.max_bytes,
            })
        }
    }

    impl From<Params> for raw::EvidenceParams {
        fn from(value: Params) -> Self {
            Self {
                // Todo: Implement proper domain types so this becomes infallible
                max_age_num_blocks: value.max_age_num_blocks.try_into().unwrap(),
                max_age_duration: Some(value.max_age_duration.into()),
                max_bytes: value.max_bytes,
            }
        }
    }
}

mod v1 {
    use cometbft_proto::types::v1 as raw;

    use super::{
        ConflictingBlock, DuplicateVoteEvidence, Evidence, LightClientAttackEvidence, List, Params,
    };
    use crate::{error::Error, prelude::*};

    impl TryFrom<raw::Evidence> for Evidence {
        type Error = Error;

        fn try_from(value: raw::Evidence) -> Result<Self, Self::Error> {
            match value.sum.ok_or_else(Error::invalid_evidence)? {
                raw::evidence::Sum::DuplicateVoteEvidence(ev) => {
                    Ok(Evidence::DuplicateVote(Box::new(ev.try_into()?)))
                },
                raw::evidence::Sum::LightClientAttackEvidence(ev) => {
                    Ok(Evidence::LightClientAttack(Box::new(ev.try_into()?)))
                },
            }
        }
    }

    impl From<Evidence> for raw::Evidence {
        fn from(value: Evidence) -> Self {
            match value {
                Evidence::DuplicateVote(ev) => raw::Evidence {
                    sum: Some(raw::evidence::Sum::DuplicateVoteEvidence((*ev).into())),
                },
                Evidence::LightClientAttack(ev) => raw::Evidence {
                    sum: Some(raw::evidence::Sum::LightClientAttackEvidence((*ev).into())),
                },
            }
        }
    }

    impl TryFrom<raw::DuplicateVoteEvidence> for DuplicateVoteEvidence {
        type Error = Error;

        fn try_from(value: raw::DuplicateVoteEvidence) -> Result<Self, Self::Error> {
            Ok(Self {
                vote_a: value
                    .vote_a
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
                vote_b: value
                    .vote_b
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
                total_voting_power: value.total_voting_power.try_into()?,
                validator_power: value.validator_power.try_into()?,
                timestamp: value
                    .timestamp
                    .ok_or_else(Error::missing_timestamp)?
                    .try_into()?,
            })
        }
    }

    impl From<DuplicateVoteEvidence> for raw::DuplicateVoteEvidence {
        fn from(value: DuplicateVoteEvidence) -> Self {
            raw::DuplicateVoteEvidence {
                vote_a: Some(value.vote_a.into()),
                vote_b: Some(value.vote_b.into()),
                total_voting_power: value.total_voting_power.into(),
                validator_power: value.total_voting_power.into(),
                timestamp: Some(value.timestamp.into()),
            }
        }
    }

    impl TryFrom<raw::LightBlock> for ConflictingBlock {
        type Error = Error;

        fn try_from(value: raw::LightBlock) -> Result<Self, Self::Error> {
            Ok(ConflictingBlock {
                signed_header: value
                    .signed_header
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
                validator_set: value
                    .validator_set
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
            })
        }
    }

    impl From<ConflictingBlock> for raw::LightBlock {
        fn from(value: ConflictingBlock) -> Self {
            raw::LightBlock {
                signed_header: Some(value.signed_header.into()),
                validator_set: Some(value.validator_set.into()),
            }
        }
    }

    impl TryFrom<raw::LightClientAttackEvidence> for LightClientAttackEvidence {
        type Error = Error;

        fn try_from(ev: raw::LightClientAttackEvidence) -> Result<Self, Self::Error> {
            Ok(LightClientAttackEvidence {
                conflicting_block: ev
                    .conflicting_block
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
                common_height: ev.common_height.try_into()?,
                byzantine_validators: ev
                    .byzantine_validators
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?,
                total_voting_power: ev.total_voting_power.try_into()?,
                timestamp: ev
                    .timestamp
                    .ok_or_else(Error::missing_timestamp)?
                    .try_into()?,
            })
        }
    }

    impl From<LightClientAttackEvidence> for raw::LightClientAttackEvidence {
        fn from(ev: LightClientAttackEvidence) -> Self {
            raw::LightClientAttackEvidence {
                conflicting_block: Some(ev.conflicting_block.into()),
                common_height: ev.common_height.into(),
                byzantine_validators: ev
                    .byzantine_validators
                    .into_iter()
                    .map(Into::into)
                    .collect(),
                total_voting_power: ev.total_voting_power.into(),
                timestamp: Some(ev.timestamp.into()),
            }
        }
    }

    impl TryFrom<raw::EvidenceList> for List {
        type Error = Error;
        fn try_from(value: raw::EvidenceList) -> Result<Self, Self::Error> {
            let evidence = value
                .evidence
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Self(evidence))
        }
    }

    impl From<List> for raw::EvidenceList {
        fn from(value: List) -> Self {
            raw::EvidenceList {
                evidence: value.0.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl TryFrom<raw::EvidenceParams> for Params {
        type Error = Error;

        fn try_from(value: raw::EvidenceParams) -> Result<Self, Self::Error> {
            Ok(Self {
                max_age_num_blocks: value
                    .max_age_num_blocks
                    .try_into()
                    .map_err(Error::negative_max_age_num)?,
                max_age_duration: value
                    .max_age_duration
                    .ok_or_else(Error::missing_max_age_duration)?
                    .try_into()?,
                max_bytes: value.max_bytes,
            })
        }
    }

    impl From<Params> for raw::EvidenceParams {
        fn from(value: Params) -> Self {
            Self {
                // Todo: Implement proper domain types so this becomes infallible
                max_age_num_blocks: value.max_age_num_blocks.try_into().unwrap(),
                max_age_duration: Some(value.max_age_duration.into()),
                max_bytes: value.max_bytes,
            }
        }
    }
}

mod v1beta1 {
    use cometbft_proto::types::v1beta1 as raw;

    use super::{
        ConflictingBlock, DuplicateVoteEvidence, Evidence, LightClientAttackEvidence, List, Params,
    };
    use crate::{error::Error, prelude::*};

    impl TryFrom<raw::Evidence> for Evidence {
        type Error = Error;

        fn try_from(value: raw::Evidence) -> Result<Self, Self::Error> {
            match value.sum.ok_or_else(Error::invalid_evidence)? {
                raw::evidence::Sum::DuplicateVoteEvidence(ev) => {
                    Ok(Evidence::DuplicateVote(Box::new(ev.try_into()?)))
                },
                raw::evidence::Sum::LightClientAttackEvidence(ev) => {
                    Ok(Evidence::LightClientAttack(Box::new(ev.try_into()?)))
                },
            }
        }
    }

    impl From<Evidence> for raw::Evidence {
        fn from(value: Evidence) -> Self {
            match value {
                Evidence::DuplicateVote(ev) => raw::Evidence {
                    sum: Some(raw::evidence::Sum::DuplicateVoteEvidence((*ev).into())),
                },
                Evidence::LightClientAttack(ev) => raw::Evidence {
                    sum: Some(raw::evidence::Sum::LightClientAttackEvidence((*ev).into())),
                },
            }
        }
    }

    impl TryFrom<raw::DuplicateVoteEvidence> for DuplicateVoteEvidence {
        type Error = Error;

        fn try_from(value: raw::DuplicateVoteEvidence) -> Result<Self, Self::Error> {
            Ok(Self {
                vote_a: value
                    .vote_a
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
                vote_b: value
                    .vote_b
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
                total_voting_power: value.total_voting_power.try_into()?,
                validator_power: value.validator_power.try_into()?,
                timestamp: value
                    .timestamp
                    .ok_or_else(Error::missing_timestamp)?
                    .try_into()?,
            })
        }
    }

    impl From<DuplicateVoteEvidence> for raw::DuplicateVoteEvidence {
        fn from(value: DuplicateVoteEvidence) -> Self {
            raw::DuplicateVoteEvidence {
                vote_a: Some(value.vote_a.into()),
                vote_b: Some(value.vote_b.into()),
                total_voting_power: value.total_voting_power.into(),
                validator_power: value.total_voting_power.into(),
                timestamp: Some(value.timestamp.into()),
            }
        }
    }

    impl TryFrom<raw::LightBlock> for ConflictingBlock {
        type Error = Error;

        fn try_from(value: raw::LightBlock) -> Result<Self, Self::Error> {
            Ok(ConflictingBlock {
                signed_header: value
                    .signed_header
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
                validator_set: value
                    .validator_set
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
            })
        }
    }

    impl From<ConflictingBlock> for raw::LightBlock {
        fn from(value: ConflictingBlock) -> Self {
            raw::LightBlock {
                signed_header: Some(value.signed_header.into()),
                validator_set: Some(value.validator_set.into()),
            }
        }
    }

    impl TryFrom<raw::LightClientAttackEvidence> for LightClientAttackEvidence {
        type Error = Error;

        fn try_from(ev: raw::LightClientAttackEvidence) -> Result<Self, Self::Error> {
            Ok(LightClientAttackEvidence {
                conflicting_block: ev
                    .conflicting_block
                    .ok_or_else(Error::missing_evidence)?
                    .try_into()?,
                common_height: ev.common_height.try_into()?,
                byzantine_validators: ev
                    .byzantine_validators
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?,
                total_voting_power: ev.total_voting_power.try_into()?,
                timestamp: ev
                    .timestamp
                    .ok_or_else(Error::missing_timestamp)?
                    .try_into()?,
            })
        }
    }

    impl From<LightClientAttackEvidence> for raw::LightClientAttackEvidence {
        fn from(ev: LightClientAttackEvidence) -> Self {
            raw::LightClientAttackEvidence {
                conflicting_block: Some(ev.conflicting_block.into()),
                common_height: ev.common_height.into(),
                byzantine_validators: ev
                    .byzantine_validators
                    .into_iter()
                    .map(Into::into)
                    .collect(),
                total_voting_power: ev.total_voting_power.into(),
                timestamp: Some(ev.timestamp.into()),
            }
        }
    }

    impl TryFrom<raw::EvidenceList> for List {
        type Error = Error;
        fn try_from(value: raw::EvidenceList) -> Result<Self, Self::Error> {
            let evidence = value
                .evidence
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Self(evidence))
        }
    }

    impl From<List> for raw::EvidenceList {
        fn from(value: List) -> Self {
            raw::EvidenceList {
                evidence: value.0.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl TryFrom<raw::EvidenceParams> for Params {
        type Error = Error;

        fn try_from(value: raw::EvidenceParams) -> Result<Self, Self::Error> {
            Ok(Self {
                max_age_num_blocks: value
                    .max_age_num_blocks
                    .try_into()
                    .map_err(Error::negative_max_age_num)?,
                max_age_duration: value
                    .max_age_duration
                    .ok_or_else(Error::missing_max_age_duration)?
                    .try_into()?,
                max_bytes: value.max_bytes,
            })
        }
    }

    impl From<Params> for raw::EvidenceParams {
        fn from(value: Params) -> Self {
            Self {
                // Todo: Implement proper domain types so this becomes infallible
                max_age_num_blocks: value.max_age_num_blocks.try_into().unwrap(),
                max_age_duration: Some(value.max_age_duration.into()),
                max_bytes: value.max_bytes,
            }
        }
    }
}
