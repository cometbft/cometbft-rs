/// Repository to fetch CometBFT from.
const COMETBFT_REPO: &str = "https://github.com/cometbft/cometbft";

/// Information on a CometBFT snapshot to generate prost structures from.
pub struct CometBFTVersion {
    /// Repository URL.
    pub repo: &'static str,
    /// Identifier to use in module names.
    pub ident: &'static str,
    /// Prefix: either tendermint or cometbft.
    pub prefix: &'static str,
    /// A commitish reference in the tendermint git repository, for example:
    ///
    /// - Tag: `v0.34.0-rc4`
    /// - Branch: `main`
    /// - Commit ID (full length): `d7d0ffea13c60c98b812d243ba5a2c375f341c15`
    pub commitish: &'static str,
}

/// All CometBFT versions to generate code for
pub const COMETBFT_VERSIONS: &[CometBFTVersion] = &[
    CometBFTVersion {
        repo: COMETBFT_REPO,
        ident: "v0_34",
        prefix: "tendermint",
        commitish: "v0.34.35",
    },
    CometBFTVersion {
        repo: COMETBFT_REPO,
        ident: "v0_37",
        prefix: "tendermint",
        commitish: "v0.37.15",
    },
    CometBFTVersion {
        repo: COMETBFT_REPO,
        ident: "v0_38",
        prefix: "tendermint",
        commitish: "v0.38.17",
    },
    CometBFTVersion {
        repo: COMETBFT_REPO,
        ident: "v1",
        prefix: "cometbft",
        commitish: "v1.0.0",
    },
];

/// Predefined custom attributes for message annotations
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
const ALIAS_TOTAL_VOTING_POWER_QUOTED: &str =
    r#"#[serde(alias = "TotalVotingPower", with = "crate::serializers::from_str")]"#;
const ALIAS_VALIDATOR_POWER_QUOTED: &str =
    r#"#[serde(alias = "ValidatorPower", with = "crate::serializers::from_str")]"#;
const RENAME_TIMESTAMP: &str = r#"#[serde(rename = "Timestamp")]"#;
const ALIAS_TIMESTAMP: &str = r#"#[serde(alias = "Timestamp")]"#;
const RENAME_PARTS: &str = r#"#[serde(rename = "parts", alias = "part_set_header")]"#;

/// Custom type attributes applied on top of protobuf structs
/// The first item in the tuple defines the message where the annotation should apply and
/// the second item is the string that should be added as annotation.
/// The first item is a path as defined in the prost_build::Config::btree_map here:
/// https://docs.rs/prost-build/0.6.1/prost_build/struct.Config.html#method.btree_map
pub static CUSTOM_TYPE_ATTRIBUTES_COMMON: &[(&str, &str)] = &[
    (".tendermint.abci", SERIALIZED),
    (".tendermint.crypto.Proof", SERIALIZED),
    (".tendermint.crypto.ProofOp", SERIALIZED),
    (".tendermint.crypto.ProofOps", SERIALIZED),
    (".tendermint.crypto.PublicKey.sum", SERIALIZED),
    (".tendermint.crypto.PublicKey.sum", TYPE_TAG),
    (".tendermint.libs.bits.BitArray", SERIALIZED),
    (".tendermint.p2p", SERIALIZED),
    (".tendermint.types.ABCIParams", SERIALIZED),
    (".tendermint.types.Block", SERIALIZED),
    (".tendermint.types.BlockID", SERIALIZED),
    (".tendermint.types.BlockMeta", SERIALIZED),
    (".tendermint.types.CanonicalBlockID", SERIALIZED),
    (".tendermint.types.CanonicalPartSetHeader", SERIALIZED),
    (".tendermint.types.CanonicalVote", SERIALIZED),
    (".tendermint.types.Commit", SERIALIZED),
    (".tendermint.types.CommitSig", SERIALIZED),
    (".tendermint.types.ConsensusParams", SERIALIZED),
    (".tendermint.types.Data", SERIALIZED),
    (".tendermint.types.DuplicateVoteEvidence", SERIALIZED),
    (".tendermint.types.Evidence.sum", SERIALIZED),
    (".tendermint.types.Evidence.sum", TYPE_TAG),
    (".tendermint.types.EvidenceList", SERIALIZED),
    (".tendermint.types.EvidenceParams", SERIALIZED),
    (".tendermint.types.Header", SERIALIZED),
    (".tendermint.types.LightBlock", SERIALIZED),
    (".tendermint.types.LightClientAttackEvidence", SERIALIZED),
    (".tendermint.types.PartSetHeader", SERIALIZED),
    (".tendermint.types.SignedHeader", SERIALIZED),
    (".tendermint.types.TxProof", SERIALIZED),
    (".tendermint.types.Validator", SERIALIZED),
    (".tendermint.types.ValidatorSet", SERIALIZED),
    (".tendermint.types.VersionParams", SERIALIZED),
    (".tendermint.types.ValidatorParams", SERIALIZED),
    (".tendermint.types.BlockParams", SERIALIZED),
    (".tendermint.types.Vote", SERIALIZED),
    (".tendermint.version.Consensus", SERIALIZED),
];

pub static CUSTOM_TYPE_ATTRIBUTES_V_034: &[(&str, &str)] = &[(
    ".tendermint.types.LightClientAttackEvidence",
    RENAME_ALL_PASCALCASE,
)];

pub static CUSTOM_TYPE_ATTRIBUTES_V_037: &[(&str, &str)] = &[(
    ".tendermint.types.LightClientAttackEvidence",
    RENAME_ALL_PASCALCASE,
)];

pub static CUSTOM_TYPE_ATTRIBUTES_V_038: &[(&str, &str)] = &[];

pub static CUSTOM_TYPE_ATTRIBUTES_V_1: &[(&str, &str)] = &[
    (".cometbft.abci", SERIALIZED),
    (".cometbft.p2p", SERIALIZED),
    (".cometbft.crypto.v1.Proof", SERIALIZED),
    (".cometbft.crypto.v1.ProofOp", SERIALIZED),
    (".cometbft.crypto.v1.ProofOps", SERIALIZED),
    (".cometbft.crypto.v1.PublicKey.sum", SERIALIZED),
    (".cometbft.crypto.v1.PublicKey.sum", TYPE_TAG),
    (".cometbft.libs.bits.v1.BitArray", SERIALIZED),
    (".cometbft.types.v1.ABCIParams", SERIALIZED),
    (".cometbft.types.v1.Block", SERIALIZED),
    (".cometbft.types.v1.BlockID", SERIALIZED),
    (".cometbft.types.v1.BlockMeta", SERIALIZED),
    (".cometbft.types.v1.CanonicalBlockID", SERIALIZED),
    (".cometbft.types.v1.CanonicalPartSetHeader", SERIALIZED),
    (".cometbft.types.v1.CanonicalVote", SERIALIZED),
    (".cometbft.types.v1.Commit", SERIALIZED),
    (".cometbft.types.v1.CommitSig", SERIALIZED),
    (".cometbft.types.v1.ConsensusParams", SERIALIZED),
    (".cometbft.types.v1.Data", SERIALIZED),
    (".cometbft.types.v1.DuplicateVoteEvidence", SERIALIZED),
    (".cometbft.types.v1.Evidence.sum", SERIALIZED),
    (".cometbft.types.v1.Evidence.sum", TYPE_TAG),
    (".cometbft.types.v1.EvidenceList", SERIALIZED),
    (".cometbft.types.v1.EvidenceParams", SERIALIZED),
    (".cometbft.types.v1.Header", SERIALIZED),
    (".cometbft.types.v1.LightBlock", SERIALIZED),
    (
        ".cometbft.types.v1.LightClientAttackEvidence",
        RENAME_ALL_PASCALCASE,
    ),
    (".cometbft.types.v1.LightClientAttackEvidence", SERIALIZED),
    (".cometbft.types.v1.PartSetHeader", SERIALIZED),
    (".cometbft.types.v1.SignedHeader", SERIALIZED),
    (".cometbft.types.v1.TxProof", SERIALIZED),
    (".cometbft.types.v1.Validator", SERIALIZED),
    (".cometbft.types.v1.ValidatorSet", SERIALIZED),
    (".cometbft.types.v1.VersionParams", SERIALIZED),
    (".cometbft.types.v1.ValidatorParams", SERIALIZED),
    (".cometbft.types.v1.BlockParams", SERIALIZED),
    (".cometbft.types.v1.SynchronyParams", SERIALIZED),
    (".cometbft.types.v1.FeatureParams", SERIALIZED),
    (".cometbft.types.v1.Vote", SERIALIZED),
    (".cometbft.version.v1.Consensus", SERIALIZED),
    (".cometbft.types.v1beta1.Block", SERIALIZED),
    (".cometbft.types.v1beta1.BlockID", SERIALIZED),
    (".cometbft.types.v1beta1.BlockMeta", SERIALIZED),
    (".cometbft.types.v1beta1.CanonicalBlockID", SERIALIZED),
    (".cometbft.types.v1beta1.CanonicalPartSetHeader", SERIALIZED),
    (".cometbft.types.v1beta1.CanonicalVote", SERIALIZED),
    (".cometbft.types.v1beta1.Commit", SERIALIZED),
    (".cometbft.types.v1beta1.CommitSig", SERIALIZED),
    (".cometbft.types.v1beta1.ConsensusParams", SERIALIZED),
    (".cometbft.types.v1beta1.Data", SERIALIZED),
    (".cometbft.types.v1beta1.DuplicateVoteEvidence", SERIALIZED),
    (".cometbft.types.v1beta1.Evidence.sum", SERIALIZED),
    (".cometbft.types.v1beta1.Evidence.sum", TYPE_TAG),
    (".cometbft.types.v1beta1.EvidenceList", SERIALIZED),
    (".cometbft.types.v1beta1.EvidenceParams", SERIALIZED),
    (".cometbft.types.v1beta1.Header", SERIALIZED),
    (".cometbft.types.v1beta1.LightBlock", SERIALIZED),
    (
        ".cometbft.types.v1beta1.LightClientAttackEvidence",
        RENAME_ALL_PASCALCASE,
    ),
    (
        ".cometbft.types.v1beta1.LightClientAttackEvidence",
        SERIALIZED,
    ),
    (".cometbft.types.v1beta1.PartSetHeader", SERIALIZED),
    (".cometbft.types.v1beta1.SignedHeader", SERIALIZED),
    (".cometbft.types.v1beta1.TxProof", SERIALIZED),
    (".cometbft.types.v1beta1.Validator", SERIALIZED),
    (".cometbft.types.v1beta1.ValidatorSet", SERIALIZED),
    (".cometbft.types.v1beta1.VersionParams", SERIALIZED),
    (".cometbft.types.v1beta1.ValidatorParams", SERIALIZED),
    (".cometbft.types.v1beta1.BlockParams", SERIALIZED),
    (".cometbft.types.v1beta1.Vote", SERIALIZED),
    (".cometbft.types.v1beta2.ConsensusParams", SERIALIZED),
    (".cometbft.types.v1beta2.BlockParams", SERIALIZED),
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
        ".tendermint.types.Evidence.sum.duplicate_vote_evidence",
        RENAME_DUPLICATEVOTE,
    ),
    (
        ".tendermint.types.Evidence.sum.light_client_attack_evidence",
        RENAME_LIGHTCLIENTATTACK,
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
    (".tendermint.types.Vote.extension", NULLABLE),
    (".tendermint.types.Vote.extension_signature", NULLABLE),
    (".tendermint.types.Validator.address", HEXSTRING),
    (
        ".tendermint.types.Validator.voting_power",
        ALIAS_POWER_QUOTED,
    ), // https://github.com/tendermint/tendermint/issues/5549
    (
        ".tendermint.types.Validator.proposer_priority",
        QUOTED_ALLOW_NULL,
    ), // null occurs in some LightBlock data
    (".tendermint.types.Validator.proposer_priority", DEFAULT), /* Default is for /genesis
                                                                 * deserialization */
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
    (".tendermint.types.TxProof.data", BASE64STRING),
    (".tendermint.types.TxProof.root_hash", HEXSTRING),
    (".tendermint.crypto.Proof.index", QUOTED),
    (".tendermint.crypto.Proof.total", QUOTED),
    (".tendermint.crypto.Proof.aunts", VEC_BASE64STRING),
    (".tendermint.crypto.Proof.leaf_hash", BASE64STRING),
];

pub static CUSTOM_FIELD_ATTRIBUTES_V_034: &[(&str, &str)] = &[
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
];

pub static CUSTOM_FIELD_ATTRIBUTES_V_037: &[(&str, &str)] = &[
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
];

pub static CUSTOM_FIELD_ATTRIBUTES_V_038: &[(&str, &str)] = &[
    (
        ".tendermint.types.DuplicateVoteEvidence.total_voting_power",
        ALIAS_TOTAL_VOTING_POWER_QUOTED,
    ),
    (
        ".tendermint.types.DuplicateVoteEvidence.validator_power",
        ALIAS_VALIDATOR_POWER_QUOTED,
    ),
    (
        ".tendermint.types.DuplicateVoteEvidence.timestamp",
        ALIAS_TIMESTAMP,
    ),
];

pub static CUSTOM_FIELD_ATTRIBUTES_V_1: &[(&str, &str)] = &[
    (
        ".cometbft.types.v1beta1.EvidenceParams.max_bytes",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".cometbft.types.v1beta1.EvidenceParams.max_age_num_blocks",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".cometbft.types.v1.EvidenceParams.max_bytes",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".cometbft.types.v1.EvidenceParams.max_age_num_blocks",
        QUOTED_WITH_DEFAULT,
    ),
    (".cometbft.version.v1.Consensus.block", QUOTED),
    (".cometbft.version.v1.Consensus.app", QUOTED_WITH_DEFAULT),
    (".cometbft.abci.v1.InfoResponse.data", DEFAULT),
    (".cometbft.abci.v1.InfoResponse.version", DEFAULT),
    (
        ".cometbft.abci.v1.InfoResponse.app_version",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".cometbft.abci.v1.InfoResponse.last_block_height",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".cometbft.abci.v1.InfoResponse.last_block_app_hash",
        DEFAULT,
    ),
    (
        ".cometbft.abci.v1.InfoResponse.last_block_app_hash",
        BYTES_SKIP_IF_EMPTY,
    ),
    (".cometbft.abci.v1beta1.ResponseInfo.data", DEFAULT),
    (".cometbft.abci.v1beta1.ResponseInfo.version", DEFAULT),
    (
        ".cometbft.abci.v1beta1.ResponseInfo.app_version",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".cometbft.abci.v1beta1.ResponseInfo.last_block_height",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".cometbft.abci.v1beta1.ResponseInfo.last_block_app_hash",
        DEFAULT,
    ),
    (
        ".cometbft.abci.v1beta1.ResponseInfo.last_block_app_hash",
        BYTES_SKIP_IF_EMPTY,
    ),
    (".cometbft.types.v1beta1.BlockID.hash", HEXSTRING),
    (".cometbft.types.v1.BlockID.hash", HEXSTRING),
    (
        ".cometbft.types.v1beta1.BlockID.part_set_header",
        RENAME_PARTS,
    ),
    (".cometbft.types.v1.BlockID.part_set_header", RENAME_PARTS),
    (
        ".cometbft.types.v1beta1.PartSetHeader.total",
        PART_SET_HEADER_TOTAL,
    ),
    (
        ".cometbft.types.v1.PartSetHeader.total",
        PART_SET_HEADER_TOTAL,
    ),
    (".cometbft.types.v1beta1.PartSetHeader.hash", HEXSTRING),
    (".cometbft.types.v1.PartSetHeader.hash", HEXSTRING),
    (".cometbft.types.v1beta1.Header.height", QUOTED),
    (".cometbft.types.v1.Header.height", QUOTED),
    (".cometbft.types.v1beta1.Header.time", OPTIONAL),
    (".cometbft.types.v1.Header.time", OPTIONAL),
    (".cometbft.types.v1beta1.Header.last_commit_hash", HEXSTRING),
    (".cometbft.types.v1.Header.last_commit_hash", HEXSTRING),
    (".cometbft.types.v1beta1.Header.data_hash", HEXSTRING),
    (".cometbft.types.v1.Header.data_hash", HEXSTRING),
    (".cometbft.types.v1beta1.Header.validators_hash", HEXSTRING),
    (".cometbft.types.v1.Header.validators_hash", HEXSTRING),
    (
        ".cometbft.types.v1beta1.Header.next_validators_hash",
        HEXSTRING,
    ),
    (".cometbft.types.v1.Header.next_validators_hash", HEXSTRING),
    (".cometbft.types.v1beta1.Header.consensus_hash", HEXSTRING),
    (".cometbft.types.v1.Header.consensus_hash", HEXSTRING),
    (".cometbft.types.v1beta1.Header.app_hash", HEXSTRING),
    (".cometbft.types.v1.Header.app_hash", HEXSTRING),
    (
        ".cometbft.types.v1beta1.Header.last_results_hash",
        HEXSTRING,
    ),
    (".cometbft.types.v1.Header.last_results_hash", HEXSTRING),
    (".cometbft.types.v1beta1.Header.evidence_hash", HEXSTRING),
    (".cometbft.types.v1.Header.evidence_hash", HEXSTRING),
    (".cometbft.types.v1beta1.Header.proposer_address", HEXSTRING),
    (".cometbft.types.v1.Header.proposer_address", HEXSTRING),
    (".cometbft.types.v1beta1.Data.txs", NULLABLEVECARRAY),
    (".cometbft.types.v1.Data.txs", NULLABLEVECARRAY),
    (".cometbft.types.v1beta1.EvidenceList.evidence", NULLABLE),
    (".cometbft.types.v1.EvidenceList.evidence", NULLABLE),
    (".cometbft.types.v1beta1.Commit.height", QUOTED),
    (".cometbft.types.v1.Commit.height", QUOTED),
    (".cometbft.types.v1beta1.Commit.signatures", NULLABLE),
    (".cometbft.types.v1.Commit.signatures", NULLABLE),
    (
        ".cometbft.types.v1beta1.CommitSig.validator_address",
        HEXSTRING,
    ),
    (".cometbft.types.v1.CommitSig.validator_address", HEXSTRING),
    (".cometbft.types.v1beta1.CommitSig.timestamp", OPTIONAL),
    (".cometbft.types.v1.CommitSig.timestamp", OPTIONAL),
    (".cometbft.types.v1beta1.CommitSig.signature", BASE64STRING),
    (".cometbft.types.v1.CommitSig.signature", BASE64STRING),
    (
        ".cometbft.types.v1beta1.DuplicateVoteEvidence.total_voting_power",
        ALIAS_TOTAL_VOTING_POWER_QUOTED,
    ),
    (
        ".cometbft.types.v1.DuplicateVoteEvidence.total_voting_power",
        ALIAS_TOTAL_VOTING_POWER_QUOTED,
    ),
    (
        ".cometbft.types.v1beta1.DuplicateVoteEvidence.validator_power",
        ALIAS_VALIDATOR_POWER_QUOTED,
    ),
    (
        ".cometbft.types.v1.DuplicateVoteEvidence.validator_power",
        ALIAS_VALIDATOR_POWER_QUOTED,
    ),
    (
        ".cometbft.types.v1beta1.DuplicateVoteEvidence.timestamp",
        ALIAS_TIMESTAMP,
    ),
    (
        ".cometbft.types.v1.DuplicateVoteEvidence.timestamp",
        ALIAS_TIMESTAMP,
    ),
    (
        ".cometbft.types.v1beta1.LightClientAttackEvidence.common_height",
        QUOTED,
    ),
    (
        ".cometbft.types.v1.LightClientAttackEvidence.common_height",
        QUOTED,
    ),
    (
        ".cometbft.types.v1beta1.LightClientAttackEvidence.total_voting_power",
        QUOTED,
    ),
    (
        ".cometbft.types.v1.LightClientAttackEvidence.total_voting_power",
        QUOTED,
    ),
    (".cometbft.types.v1beta1.Vote.height", QUOTED),
    (".cometbft.types.v1.Vote.height", QUOTED),
    (".cometbft.types.v1beta1.Vote.validator_address", HEXSTRING),
    (".cometbft.types.v1.Vote.validator_address", HEXSTRING),
    (".cometbft.types.v1beta1.Vote.signature", BASE64STRING),
    (".cometbft.types.v1.Vote.signature", BASE64STRING),
    (".cometbft.types.v1beta1.Vote.timestamp", OPTIONAL),
    (".cometbft.types.v1.Vote.timestamp", OPTIONAL),
    (".cometbft.types.v1.Vote.extension", DEFAULT),
    (".cometbft.types.v1.Vote.extension_signature", DEFAULT),
    (".cometbft.types.v1beta1.Validator.address", HEXSTRING),
    (".cometbft.types.v1.Validator.address", HEXSTRING),
    (
        ".cometbft.types.v1beta1.Validator.voting_power",
        ALIAS_POWER_QUOTED,
    ), // https://github.com/tendermint/tendermint/issues/5549
    (
        ".cometbft.types.v1.Validator.voting_power",
        ALIAS_POWER_QUOTED,
    ), // https://github.com/tendermint/tendermint/issues/5549
    (
        ".cometbft.types.v1beta1.Validator.proposer_priority",
        QUOTED_ALLOW_NULL,
    ), // null occurs in some LightBlock data
    (
        ".cometbft.types.v1.Validator.proposer_priority",
        QUOTED_ALLOW_NULL,
    ), // null occurs in some LightBlock data
    (
        ".cometbft.types.v1beta1.Validator.proposer_priority",
        DEFAULT,
    ), // Default is for /genesis deserialization
    (".cometbft.types.v1.Validator.proposer_priority", DEFAULT), // Default is for /genesis deserialization
    (".cometbft.types.v1.Validator.pub_key_bytes", BASE64STRING),
    (
        ".cometbft.types.v1beta1.ValidatorSet.total_voting_power",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".cometbft.types.v1.ValidatorSet.total_voting_power",
        QUOTED_WITH_DEFAULT,
    ),
    (
        ".cometbft.types.v1beta1.ValidatorSet.total_voting_power",
        SKIP_SERIALIZING,
    ),
    (
        ".cometbft.types.v1.ValidatorSet.total_voting_power",
        SKIP_SERIALIZING,
    ),
    (".cometbft.types.v1beta1.BlockMeta.block_size", QUOTED),
    (".cometbft.types.v1.BlockMeta.block_size", QUOTED),
    (".cometbft.types.v1beta1.BlockMeta.num_txs", QUOTED),
    (".cometbft.types.v1.BlockMeta.num_txs", QUOTED),
    (".cometbft.crypto.v1.PublicKey.sum.ed25519", RENAME_EDPUBKEY),
    (
        ".cometbft.crypto.v1.PublicKey.sum.secp256k1",
        RENAME_SECPPUBKEY,
    ),
    (".cometbft.crypto.v1.PublicKey.sum.sr25519", RENAME_SRPUBKEY),
    (
        ".cometbft.types.v1beta1.Evidence.sum.duplicate_vote_evidence",
        RENAME_DUPLICATEVOTE,
    ),
    (
        ".cometbft.types.v1.Evidence.sum.duplicate_vote_evidence",
        RENAME_DUPLICATEVOTE,
    ),
    (
        ".cometbft.types.v1beta1.Evidence.sum.light_client_attack_evidence",
        RENAME_LIGHTCLIENTATTACK,
    ),
    (
        ".cometbft.types.v1.Evidence.sum.light_client_attack_evidence",
        RENAME_LIGHTCLIENTATTACK,
    ),
    (".cometbft.types.v1beta1.TxProof.data", BASE64STRING),
    (".cometbft.types.v1.TxProof.data", BASE64STRING),
    (".cometbft.types.v1beta1.TxProof.root_hash", HEXSTRING),
    (".cometbft.types.v1.TxProof.root_hash", HEXSTRING),
    (".cometbft.crypto.v1.Proof.index", QUOTED),
    (".cometbft.crypto.v1.Proof.total", QUOTED),
    (".cometbft.crypto.v1.Proof.aunts", VEC_BASE64STRING),
    (".cometbft.crypto.v1.Proof.leaf_hash", BASE64STRING),
];

pub fn get_custom_field_attributes(version: &CometBFTVersion) -> &[(&str, &str)] {
    match version.ident {
        "v0_34" => CUSTOM_FIELD_ATTRIBUTES_V_034,
        "v0_37" => CUSTOM_FIELD_ATTRIBUTES_V_037,
        "v0_38" => CUSTOM_FIELD_ATTRIBUTES_V_038,
        "v1" => CUSTOM_FIELD_ATTRIBUTES_V_1,
        _ => unreachable!(),
    }
}

pub fn get_custom_type_attributes(version: &CometBFTVersion) -> &[(&str, &str)] {
    match version.ident {
        "v0_34" => CUSTOM_TYPE_ATTRIBUTES_V_034,
        "v0_37" => CUSTOM_TYPE_ATTRIBUTES_V_037,
        "v0_38" => CUSTOM_TYPE_ATTRIBUTES_V_038,
        "v1" => CUSTOM_TYPE_ATTRIBUTES_V_1,
        _ => unreachable!(),
    }
}
