/// Tendermint protobuf version

/// Information on a Tendermint snapshot to generate prost structures from.
pub struct TendermintVersion {
    /// Repository URL.
    pub repo: &'static str,
    /// Identifier to use in module names.
    pub ident: &'static str,
    /// A commitish reference in the tendermint git repository, for example:
    ///
    /// - Tag: `v0.34.0-rc4`
    /// - Branch: `main`
    /// - Commit ID (full length): `d7d0ffea13c60c98b812d243ba5a2c375f341c15`
    pub commitish: &'static str,
    /// Project name
    pub project: &'static str,
}

/// All Tendermint versions to generate code for
pub const TENDERMINT_VERSIONS: &[TendermintVersion] = &[
    TendermintVersion {
        repo: "https://github.com/cometbft/cometbft",
        ident: "v0_34",
        commitish: "v0.34.29",
        project:"tendermint",
    },
    TendermintVersion {
        repo: "https://github.com/cometbft/cometbft",
        ident: "v0_37",
        commitish: "v0.37.2",
        project:"tendermint",
    },
    TendermintVersion {
        repo: "https://github.com/cometbft/cometbft",
        ident: "v0_38",
        commitish: "v0.38.0",
        project:"tendermint",
    },
    TendermintVersion {
        repo: "https://github.com/cometbft/cometbft",
        ident: "v1_0",
        commitish: "feature/proto-upgrade",
        project:"cometbft",
    },
];

/// Predefined custom attributes for message annotations
const PRIMITIVE_ENUM: &str = r#"#[derive(::num_derive::FromPrimitive, ::num_derive::ToPrimitive)]"#;
const SERIALIZED: &str = r#"#[derive(::serde::Deserialize, ::serde::Serialize)]"#;
const TYPE_TAG: &str = r#"#[serde(tag = "type", content = "value")]"#;

/// Predefined custom attributes for field annotations
const QUOTED: &str = r#"#[serde(with = "crate::serializers::from_str")]"#;
const QUOTED_WITH_DEFAULT: &str = r#"#[serde(with = "crate::serializers::from_str", default)]"#;
const QUOTED_ALLOW_NULL: &str = r#"#[serde(with = "crate::serializers::from_str_allow_null")]"#;
const DEFAULT: &str = r#"#[serde(default)]"#;
const HEXSTRING: &str = r#"#[serde(with = "crate::serializers::bytes::hexstring")]"#;
const BASE64STRING: &str = r#"#[serde(with = "crate::serializers::bytes::base64string")]"#;
const VEC_BASE64STRING: &str = r#"#[serde(with = "crate::serializers::bytes::vec_base64string")]"#;
const OPTIONAL: &str = r#"#[serde(with = "crate::serializers::optional")]"#;
const BYTES_SKIP_IF_EMPTY: &str = r#"#[serde(skip_serializing_if = "bytes::Bytes::is_empty")]"#;
const SKIP_SERIALIZING: &str = "#[serde(skip_serializing)]";
const RENAME_ALL_PASCALCASE: &str = r#"#[serde(rename_all = "PascalCase")]"#;
const NULLABLEVECARRAY: &str = r#"#[serde(with = "crate::serializers::txs")]"#;
const NULLABLE: &str = r#"#[serde(with = "crate::serializers::nullable")]"#;
const ALIAS_POWER_QUOTED: &str =
    r#"#[serde(alias = "power", with = "crate::serializers::from_str")]"#;
const PART_SET_HEADER_TOTAL: &str =
    r#"#[serde(with = "crate::serializers::part_set_header_total")]"#;
const RENAME_EDPUBKEY: &str = r#"#[serde(rename = "tendermint/PubKeyEd25519", with = "crate::serializers::bytes::base64string")]"#;
const RENAME_SECPPUBKEY: &str = r#"#[serde(rename = "tendermint/PubKeySecp256k1", with = "crate::serializers::bytes::base64string")]"#;
const RENAME_SRPUBKEY: &str = r#"#[serde(rename = "tendermint/PubKeySr25519", with = "crate::serializers::bytes::base64string")]"#;
const RENAME_DUPLICATEVOTE: &str = r#"#[serde(rename = "tendermint/DuplicateVoteEvidence")]"#;
const RENAME_LIGHTCLIENTATTACK: &str =
    r#"#[serde(rename = "tendermint/LightClientAttackEvidence")]"#;
const RENAME_TOTAL_VOTING_POWER_QUOTED: &str =
    r#"#[serde(rename = "TotalVotingPower", with = "crate::serializers::from_str")]"#;
const RENAME_VALIDATOR_POWER_QUOTED: &str =
    r#"#[serde(rename = "ValidatorPower", with = "crate::serializers::from_str")]"#;
const RENAME_TIMESTAMP: &str = r#"#[serde(rename = "Timestamp")]"#;
const RENAME_PARTS: &str = r#"#[serde(rename = "parts", alias = "part_set_header")]"#;

/// Custom type attributes applied on top of protobuf structs
/// The first item in the tuple defines the message where the annotation should apply and
/// the second item is the string that should be added as annotation.
/// The first item is a path as defined in the prost_build::Config::btree_map here:
/// https://docs.rs/prost-build/0.6.1/prost_build/struct.Config.html#method.btree_map
pub static CUSTOM_TYPE_ATTRIBUTES: &[(&str, &str)] = &[
    (".tendermint.libs.bits.BitArray", SERIALIZED),
    (".tendermint.types.BlockIDFlag", PRIMITIVE_ENUM),
    (".tendermint.types.Block", SERIALIZED),
    (".tendermint.types.Data", SERIALIZED),
    (".tendermint.types.EvidenceParams", SERIALIZED),
    (".tendermint.types.Evidence.sum", SERIALIZED),
    (".tendermint.types.Evidence.sum", TYPE_TAG),
    (".tendermint.types.EvidenceList", SERIALIZED),
    (".tendermint.types.DuplicateVoteEvidence", SERIALIZED),
    (".tendermint.types.Vote", SERIALIZED),
    (".tendermint.types.BlockID", SERIALIZED),
    (".tendermint.types.PartSetHeader", SERIALIZED),
    (".tendermint.types.LightClientAttackEvidence", SERIALIZED),
    (
        ".tendermint.types.LightClientAttackEvidence",
        RENAME_ALL_PASCALCASE,
    ),
    (".tendermint.types.LightBlock", SERIALIZED),
    (".tendermint.types.SignedHeader", SERIALIZED),
    (".tendermint.types.Header", SERIALIZED),
    (".tendermint.version.Consensus", SERIALIZED),
    (".tendermint.types.Commit", SERIALIZED),
    (".tendermint.types.CommitSig", SERIALIZED),
    (".tendermint.types.ValidatorSet", SERIALIZED),
    (".tendermint.crypto.PublicKey.sum", SERIALIZED),
    (".tendermint.crypto.PublicKey.sum", TYPE_TAG),
    (".tendermint.abci.ResponseInfo", SERIALIZED),
    (".tendermint.types.CanonicalBlockID", SERIALIZED),
    (".tendermint.types.CanonicalPartSetHeader", SERIALIZED),
    (".tendermint.types.Validator", SERIALIZED),
    (".tendermint.types.CanonicalVote", SERIALIZED),
    (".tendermint.types.BlockMeta", SERIALIZED),
    (".tendermint.types.TxProof", SERIALIZED),
    (".tendermint.crypto.Proof", SERIALIZED),
    (".cometbft.libs.bits.BitArray", SERIALIZED),//TODO Revisit
    (".cometbft.types.BlockIDFlag", PRIMITIVE_ENUM),
    (".cometbft.types.Block", SERIALIZED),
    (".cometbft.types.Data", SERIALIZED),
    (".cometbft.types.EvidenceParams", SERIALIZED),
    (".cometbft.types.Evidence.sum", SERIALIZED),
    (".cometbft.types.Evidence.sum", TYPE_TAG),
    (".cometbft.types.EvidenceList", SERIALIZED),
    (".cometbft.types.DuplicateVoteEvidence", SERIALIZED),
    (".cometbft.types.Vote", SERIALIZED),
    (".cometbft.types.BlockID", SERIALIZED),
    (".cometbft.types.PartSetHeader", SERIALIZED),
    (".cometbft.types.LightClientAttackEvidence", SERIALIZED),
    (
        ".cometbft.types.LightClientAttackEvidence",
        RENAME_ALL_PASCALCASE,
    ),
    (".cometbft.types.LightBlock", SERIALIZED),
    (".cometbft.types.SignedHeader", SERIALIZED),
    (".cometbft.types.Header", SERIALIZED),
    (".cometbft.version.Consensus", SERIALIZED),
    (".cometbft.types.Commit", SERIALIZED),
    (".cometbft.types.CommitSig", SERIALIZED),
    (".cometbft.types.ValidatorSet", SERIALIZED),
    (".cometbft.crypto.PublicKey.sum", SERIALIZED),
    (".cometbft.crypto.PublicKey.sum", TYPE_TAG),
    (".cometbft.abci.ResponseInfo", SERIALIZED),
    (".cometbft.types.CanonicalBlockID", SERIALIZED),
    (".cometbft.types.CanonicalPartSetHeader", SERIALIZED),
    (".cometbft.types.Validator", SERIALIZED),
    (".cometbft.types.CanonicalVote", SERIALIZED),
    (".cometbft.types.BlockMeta", SERIALIZED),
    (".cometbft.types.TxProof", SERIALIZED),
    (".cometbft.crypto.Proof", SERIALIZED),
];

/// Custom field attributes applied on top of protobuf fields in (a) struct(s)
/// The first item in the tuple defines the field where the annotation should apply and
/// the second item is the string that should be added as annotation.
/// The first item is a path as defined in the prost_build::Config::btree_map here:
/// https://docs.rs/prost-build/0.6.1/prost_build/struct.Config.html#method.btree_map
pub static CUSTOM_FIELD_ATTRIBUTES: &[(&str, &str)] = &[
    (
        ".tendermint.types.EvidenceParams.max_bytes",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".tendermint.types.EvidenceParams.max_age_num_blocks",
        QUOTED_WITH_DEFAULT,
    ),
    (".tendermint.version.Consensus.block", QUOTED),
    (".tendermint.version.Consensus.app", QUOTED_WITH_DEFAULT),
    (".tendermint.abci.ResponseInfo.data", DEFAULT),
    (".tendermint.abci.ResponseInfo.version", DEFAULT),
    (
        ".tendermint.abci.ResponseInfo.app_version",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".tendermint.abci.ResponseInfo.last_block_height",
        QUOTED_WITH_DEFAULT,
    ),
    (".tendermint.abci.ResponseInfo.last_block_app_hash", DEFAULT),
    (
        ".tendermint.abci.ResponseInfo.last_block_app_hash",
        BYTES_SKIP_IF_EMPTY,
    ),
    (".tendermint.types.BlockID.hash", HEXSTRING),
    (".tendermint.types.BlockID.part_set_header", RENAME_PARTS),
    (
        ".tendermint.types.PartSetHeader.total",
        PART_SET_HEADER_TOTAL,
    ),
    (".tendermint.types.PartSetHeader.hash", HEXSTRING),
    (".tendermint.types.Header.height", QUOTED),
    (".tendermint.types.Header.time", OPTIONAL),
    (".tendermint.types.Header.last_commit_hash", HEXSTRING),
    (".tendermint.types.Header.data_hash", HEXSTRING),
    (".tendermint.types.Header.validators_hash", HEXSTRING),
    (".tendermint.types.Header.next_validators_hash", HEXSTRING),
    (".tendermint.types.Header.consensus_hash", HEXSTRING),
    (".tendermint.types.Header.app_hash", HEXSTRING),
    (".tendermint.types.Header.last_results_hash", HEXSTRING),
    (".tendermint.types.Header.evidence_hash", HEXSTRING),
    (".tendermint.types.Header.proposer_address", HEXSTRING),
    (".tendermint.types.Data.txs", NULLABLEVECARRAY),
    (".tendermint.types.EvidenceList.evidence", NULLABLE),
    (".tendermint.types.Commit.height", QUOTED),
    (".tendermint.types.Commit.signatures", NULLABLE),
    (".tendermint.types.CommitSig.validator_address", HEXSTRING),
    (".tendermint.types.CommitSig.timestamp", OPTIONAL),
    (".tendermint.types.CommitSig.signature", BASE64STRING),
    (
        ".tendermint.types.DuplicateVoteEvidence.total_voting_power",
        RENAME_TOTAL_VOTING_POWER_QUOTED,
    ),
    (
        ".tendermint.types.DuplicateVoteEvidence.validator_power",
        RENAME_VALIDATOR_POWER_QUOTED,
    ),
    (
        ".tendermint.types.DuplicateVoteEvidence.timestamp",
        RENAME_TIMESTAMP,
    ),
    (
        ".tendermint.types.LightClientAttackEvidence.common_height",
        QUOTED,
    ),
    (
        ".tendermint.types.LightClientAttackEvidence.total_voting_power",
        QUOTED,
    ),
    (".tendermint.types.Vote.height", QUOTED),
    (".tendermint.types.Vote.validator_address", HEXSTRING),
    (".tendermint.types.Vote.signature", BASE64STRING),
    (".tendermint.types.Vote.timestamp", OPTIONAL),
    (".tendermint.types.Validator.address", HEXSTRING),
    (
        ".tendermint.types.Validator.voting_power",
        ALIAS_POWER_QUOTED,
    ), // https://github.com/tendermint/tendermint/issues/5549
    (
        ".tendermint.types.Validator.proposer_priority",
        QUOTED_ALLOW_NULL,
    ), // null occurs in some LightBlock data
    (".tendermint.types.Validator.proposer_priority", DEFAULT), // Default is for /genesis deserialization
    (
        ".tendermint.types.ValidatorSet.total_voting_power",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".tendermint.types.ValidatorSet.total_voting_power",
        SKIP_SERIALIZING,
    ),
    (".tendermint.types.BlockMeta.block_size", QUOTED),
    (".tendermint.types.BlockMeta.num_txs", QUOTED),
    (".tendermint.crypto.PublicKey.sum.ed25519", RENAME_EDPUBKEY),
    (
        ".tendermint.crypto.PublicKey.sum.secp256k1",
        RENAME_SECPPUBKEY,
    ),
    (".tendermint.crypto.PublicKey.sum.sr25519", RENAME_SRPUBKEY),
    (
        ".tendermint.types.Evidence.sum.duplicate_vote_evidence",
        RENAME_DUPLICATEVOTE,
    ),
    (
        ".tendermint.types.Evidence.sum.light_client_attack_evidence",
        RENAME_LIGHTCLIENTATTACK,
    ),
    (".tendermint.types.TxProof.data", BASE64STRING),
    (".tendermint.types.TxProof.root_hash", HEXSTRING),
    (".tendermint.crypto.Proof.index", QUOTED),
    (".tendermint.crypto.Proof.total", QUOTED),
    (".tendermint.crypto.Proof.aunts", VEC_BASE64STRING),
    (".tendermint.crypto.Proof.leaf_hash", BASE64STRING),
    (
        ".cometbft.types.EvidenceParams.max_bytes",
        QUOTED_WITH_DEFAULT,
    ), //TODO Revisit
    (
        ".cometbft.types.EvidenceParams.max_age_num_blocks",
        QUOTED_WITH_DEFAULT,
    ),
    (".cometbft.version.Consensus.block", QUOTED),
    (".cometbft.version.Consensus.app", QUOTED_WITH_DEFAULT),
    (".cometbft.abci.ResponseInfo.data", DEFAULT),
    (".cometbft.abci.ResponseInfo.version", DEFAULT),
    (
        ".cometbft.abci.ResponseInfo.app_version",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".cometbft.abci.ResponseInfo.last_block_height",
        QUOTED_WITH_DEFAULT,
    ),
    (".cometbft.abci.ResponseInfo.last_block_app_hash", DEFAULT),
    (
        ".cometbft.abci.ResponseInfo.last_block_app_hash",
        BYTES_SKIP_IF_EMPTY,
    ),
    (".cometbft.types.BlockID.hash", HEXSTRING),
    (".cometbft.types.BlockID.part_set_header", RENAME_PARTS),
    (
        ".cometbft.types.PartSetHeader.total",
        PART_SET_HEADER_TOTAL,
    ),
    (".cometbft.types.PartSetHeader.hash", HEXSTRING),
    (".cometbft.types.Header.height", QUOTED),
    (".cometbft.types.Header.time", OPTIONAL),
    (".cometbft.types.Header.last_commit_hash", HEXSTRING),
    (".cometbft.types.Header.data_hash", HEXSTRING),
    (".cometbft.types.Header.validators_hash", HEXSTRING),
    (".cometbft.types.Header.next_validators_hash", HEXSTRING),
    (".cometbft.types.Header.consensus_hash", HEXSTRING),
    (".cometbft.types.Header.app_hash", HEXSTRING),
    (".cometbft.types.Header.last_results_hash", HEXSTRING),
    (".cometbft.types.Header.evidence_hash", HEXSTRING),
    (".cometbft.types.Header.proposer_address", HEXSTRING),
    (".cometbft.types.Data.txs", NULLABLEVECARRAY),
    (".cometbft.types.EvidenceList.evidence", NULLABLE),
    (".cometbft.types.Commit.height", QUOTED),
    (".cometbft.types.Commit.signatures", NULLABLE),
    (".cometbft.types.CommitSig.validator_address", HEXSTRING),
    (".cometbft.types.CommitSig.timestamp", OPTIONAL),
    (".cometbft.types.CommitSig.signature", BASE64STRING),
    (
        ".cometbft.types.DuplicateVoteEvidence.total_voting_power",
        RENAME_TOTAL_VOTING_POWER_QUOTED,
    ),
    (
        ".cometbft.types.DuplicateVoteEvidence.validator_power",
        RENAME_VALIDATOR_POWER_QUOTED,
    ),
    (
        ".cometbft.types.DuplicateVoteEvidence.timestamp",
        RENAME_TIMESTAMP,
    ),
    (
        ".cometbft.types.LightClientAttackEvidence.common_height",
        QUOTED,
    ),
    (
        ".cometbft.types.LightClientAttackEvidence.total_voting_power",
        QUOTED,
    ),
    (".cometbft.types.Vote.height", QUOTED),
    (".cometbft.types.Vote.validator_address", HEXSTRING),
    (".cometbft.types.Vote.signature", BASE64STRING),
    (".cometbft.types.Vote.timestamp", OPTIONAL),
    (".cometbft.types.Validator.address", HEXSTRING),
    (
        ".cometbft.types.Validator.voting_power",
        ALIAS_POWER_QUOTED,
    ), // https://github.com/tendermint/tendermint/issues/5549
    (
        ".cometbft.types.Validator.proposer_priority",
        QUOTED_ALLOW_NULL,
    ), // null occurs in some LightBlock data
    (".cometbft.types.Validator.proposer_priority", DEFAULT), // Default is for /genesis deserialization
    (
        ".cometbft.types.ValidatorSet.total_voting_power",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".cometbft.types.ValidatorSet.total_voting_power",
        SKIP_SERIALIZING,
    ),
    (".cometbft.types.BlockMeta.block_size", QUOTED),
    (".cometbft.types.BlockMeta.num_txs", QUOTED),
    (".cometbft.crypto.PublicKey.sum.ed25519", RENAME_EDPUBKEY),
    (
        ".cometbft.crypto.PublicKey.sum.secp256k1",
        RENAME_SECPPUBKEY,
    ),
    (".cometbft.crypto.PublicKey.sum.sr25519", RENAME_SRPUBKEY),
    (
        ".cometbft.types.Evidence.sum.duplicate_vote_evidence",
        RENAME_DUPLICATEVOTE,
    ),
    (
        ".cometbft.types.Evidence.sum.light_client_attack_evidence",
        RENAME_LIGHTCLIENTATTACK,
    ),
    (".cometbft.types.TxProof.data", BASE64STRING),
    (".cometbft.types.TxProof.root_hash", HEXSTRING),
    (".cometbft.crypto.Proof.index", QUOTED),
    (".cometbft.crypto.Proof.total", QUOTED),
    (".cometbft.crypto.Proof.aunts", VEC_BASE64STRING),
    (".cometbft.crypto.Proof.leaf_hash", BASE64STRING),
];
