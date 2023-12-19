/// ValidatorSet defines a set of validators.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSet {
    #[prost(message, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    #[prost(message, optional, tag = "2")]
    pub proposer: ::core::option::Option<Validator>,
    #[prost(int64, tag = "3")]
    #[serde(with = "crate::serializers::from_str", default)]
    #[serde(skip_serializing)]
    pub total_voting_power: i64,
}
/// Validator represents a node participating in the consensus protocol.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub pub_key: ::core::option::Option<super::super::crypto::v1::PublicKey>,
    #[prost(int64, tag = "3")]
    #[serde(alias = "power", with = "crate::serializers::from_str")]
    pub voting_power: i64,
    #[prost(int64, tag = "4")]
    #[serde(with = "crate::serializers::from_str_allow_null")]
    #[serde(default)]
    pub proposer_priority: i64,
}
/// SimpleValidator is a Validator, which is serialized and hashed in consensus.
/// Address is removed because it's redundant with the pubkey.
/// Proposer priority is removed because it changes every round.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleValidator {
    #[prost(message, optional, tag = "1")]
    pub pub_key: ::core::option::Option<super::super::crypto::v1::PublicKey>,
    #[prost(int64, tag = "2")]
    pub voting_power: i64,
}
/// BlockIdFlag indicates which BlockID the signature is for
#[derive(::num_derive::FromPrimitive, ::num_derive::ToPrimitive)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockIdFlag {
    /// Indicates an error condition
    Unknown = 0,
    /// The vote was not received
    Absent = 1,
    /// Voted for the block that received the majority
    Commit = 2,
    /// Voted for nil
    Nil = 3,
}
impl BlockIdFlag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockIdFlag::Unknown => "BLOCK_ID_FLAG_UNKNOWN",
            BlockIdFlag::Absent => "BLOCK_ID_FLAG_ABSENT",
            BlockIdFlag::Commit => "BLOCK_ID_FLAG_COMMIT",
            BlockIdFlag::Nil => "BLOCK_ID_FLAG_NIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLOCK_ID_FLAG_UNKNOWN" => Some(Self::Unknown),
            "BLOCK_ID_FLAG_ABSENT" => Some(Self::Absent),
            "BLOCK_ID_FLAG_COMMIT" => Some(Self::Commit),
            "BLOCK_ID_FLAG_NIL" => Some(Self::Nil),
            _ => None,
        }
    }
}
/// Header of the parts set for a block.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartSetHeader {
    #[prost(uint32, tag = "1")]
    #[serde(with = "crate::serializers::part_set_header_total")]
    pub total: u32,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// Part of the block.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Part {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof: ::core::option::Option<super::super::crypto::v1::Proof>,
}
/// BlockID defines the unique ID of a block as its hash and its `PartSetHeader`.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockId {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    #[serde(rename = "parts", alias = "part_set_header")]
    pub part_set_header: ::core::option::Option<PartSetHeader>,
}
/// Header defines the structure of a block header.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    /// basic block info
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<super::super::version::v1::Consensus>,
    #[prost(string, tag = "2")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(with = "crate::serializers::from_str")]
    pub height: i64,
    #[prost(message, optional, tag = "4")]
    #[serde(with = "crate::serializers::optional")]
    pub time: ::core::option::Option<crate::google::protobuf::Timestamp>,
    /// prev block info
    #[prost(message, optional, tag = "5")]
    pub last_block_id: ::core::option::Option<BlockId>,
    /// hashes of block data
    ///
    /// commit from validators from the last block
    #[prost(bytes = "vec", tag = "6")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub last_commit_hash: ::prost::alloc::vec::Vec<u8>,
    /// transactions
    #[prost(bytes = "vec", tag = "7")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub data_hash: ::prost::alloc::vec::Vec<u8>,
    /// hashes from the app output from the prev block
    ///
    /// validators for the current block
    #[prost(bytes = "vec", tag = "8")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// validators for the next block
    #[prost(bytes = "vec", tag = "9")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus params for current block
    #[prost(bytes = "vec", tag = "10")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub consensus_hash: ::prost::alloc::vec::Vec<u8>,
    /// state after txs from the previous block
    #[prost(bytes = "vec", tag = "11")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
    /// root hash of all results from the txs from the previous block
    #[prost(bytes = "vec", tag = "12")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub last_results_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus info
    ///
    /// evidence included in the block
    #[prost(bytes = "vec", tag = "13")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub evidence_hash: ::prost::alloc::vec::Vec<u8>,
    /// original proposer of the block
    #[prost(bytes = "vec", tag = "14")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
/// Data contains the set of transactions included in the block
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// Txs that will be applied by state @ block.Height+1.
    /// NOTE: not all txs here are valid.  We're just agreeing on the order first.
    /// This means that block.AppHash does not include these txs.
    #[prost(bytes = "vec", repeated, tag = "1")]
    #[serde(with = "crate::serializers::txs")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Vote represents a prevote or precommit vote from validators for
/// consensus.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    pub r#type: i32,
    #[prost(int64, tag = "2")]
    #[serde(with = "crate::serializers::from_str")]
    pub height: i64,
    #[prost(int32, tag = "3")]
    pub round: i32,
    /// zero if vote is nil.
    #[prost(message, optional, tag = "4")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag = "5")]
    #[serde(with = "crate::serializers::optional")]
    pub timestamp: ::core::option::Option<crate::google::protobuf::Timestamp>,
    #[prost(bytes = "vec", tag = "6")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag = "7")]
    pub validator_index: i32,
    /// Vote signature by the validator if they participated in consensus for the
    /// associated block.
    #[prost(bytes = "vec", tag = "8")]
    #[serde(with = "crate::serializers::bytes::base64string")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// Vote extension provided by the application. Only valid for precommit
    /// messages.
    #[prost(bytes = "vec", tag = "9")]
    pub extension: ::prost::alloc::vec::Vec<u8>,
    /// Vote extension signature by the validator if they participated in
    /// consensus for the associated block.
    /// Only valid for precommit messages.
    #[prost(bytes = "vec", tag = "10")]
    pub extension_signature: ::prost::alloc::vec::Vec<u8>,
}
/// Commit contains the evidence that a block was committed by a set of validators.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commit {
    #[prost(int64, tag = "1")]
    #[serde(with = "crate::serializers::from_str")]
    pub height: i64,
    #[prost(int32, tag = "2")]
    pub round: i32,
    #[prost(message, optional, tag = "3")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, repeated, tag = "4")]
    #[serde(with = "crate::serializers::nullable")]
    pub signatures: ::prost::alloc::vec::Vec<CommitSig>,
}
/// CommitSig is a part of the Vote included in a Commit.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitSig {
    #[prost(enumeration = "BlockIdFlag", tag = "1")]
    pub block_id_flag: i32,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    #[serde(with = "crate::serializers::optional")]
    pub timestamp: ::core::option::Option<crate::google::protobuf::Timestamp>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(with = "crate::serializers::bytes::base64string")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// ExtendedCommit is a Commit with ExtendedCommitSig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedCommit {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(int32, tag = "2")]
    pub round: i32,
    #[prost(message, optional, tag = "3")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, repeated, tag = "4")]
    pub extended_signatures: ::prost::alloc::vec::Vec<ExtendedCommitSig>,
}
/// ExtendedCommitSig retains all the same fields as CommitSig but adds vote
/// extension-related fields. We use two signatures to ensure backwards compatibility.
/// That is the digest of the original signature is still the same in prior versions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedCommitSig {
    #[prost(enumeration = "BlockIdFlag", tag = "1")]
    pub block_id_flag: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<crate::google::protobuf::Timestamp>,
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// Vote extension data
    #[prost(bytes = "vec", tag = "5")]
    pub extension: ::prost::alloc::vec::Vec<u8>,
    /// Vote extension signature
    #[prost(bytes = "vec", tag = "6")]
    pub extension_signature: ::prost::alloc::vec::Vec<u8>,
}
/// Block proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    pub r#type: i32,
    #[prost(int64, tag = "2")]
    pub height: i64,
    #[prost(int32, tag = "3")]
    pub round: i32,
    #[prost(int32, tag = "4")]
    pub pol_round: i32,
    #[prost(message, optional, tag = "5")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag = "6")]
    pub timestamp: ::core::option::Option<crate::google::protobuf::Timestamp>,
    #[prost(bytes = "vec", tag = "7")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// SignedHeader contains a Header(H) and Commit(H+1) with signatures of validators who signed it.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedHeader {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub commit: ::core::option::Option<Commit>,
}
/// LightBlock is a combination of SignedHeader and ValidatorSet. It is used by light clients.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LightBlock {
    #[prost(message, optional, tag = "1")]
    pub signed_header: ::core::option::Option<SignedHeader>,
    #[prost(message, optional, tag = "2")]
    pub validator_set: ::core::option::Option<ValidatorSet>,
}
/// BlockMeta contains meta information about a block.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMeta {
    #[prost(message, optional, tag = "1")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(int64, tag = "2")]
    #[serde(with = "crate::serializers::from_str")]
    pub block_size: i64,
    #[prost(message, optional, tag = "3")]
    pub header: ::core::option::Option<Header>,
    #[prost(int64, tag = "4")]
    #[serde(with = "crate::serializers::from_str")]
    pub num_txs: i64,
}
/// TxProof represents a Merkle proof of the presence of a transaction in the Merkle tree.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxProof {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(with = "crate::serializers::bytes::hexstring")]
    pub root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(with = "crate::serializers::bytes::base64string")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof: ::core::option::Option<super::super::crypto::v1::Proof>,
}
/// SignedMsgType is a type of signed message in the consensus.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignedMsgType {
    /// Unknown
    Unknown = 0,
    /// Prevote
    Prevote = 1,
    /// Precommit
    Precommit = 2,
    /// Proposal
    Proposal = 32,
}
impl SignedMsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignedMsgType::Unknown => "SIGNED_MSG_TYPE_UNKNOWN",
            SignedMsgType::Prevote => "SIGNED_MSG_TYPE_PREVOTE",
            SignedMsgType::Precommit => "SIGNED_MSG_TYPE_PRECOMMIT",
            SignedMsgType::Proposal => "SIGNED_MSG_TYPE_PROPOSAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGNED_MSG_TYPE_UNKNOWN" => Some(Self::Unknown),
            "SIGNED_MSG_TYPE_PREVOTE" => Some(Self::Prevote),
            "SIGNED_MSG_TYPE_PRECOMMIT" => Some(Self::Precommit),
            "SIGNED_MSG_TYPE_PROPOSAL" => Some(Self::Proposal),
            _ => None,
        }
    }
}
/// Evidence is a generic type for wrapping evidence of misbehavior by a validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Evidence {
    /// The type of evidence.
    #[prost(oneof = "evidence::Sum", tags = "1, 2")]
    pub sum: ::core::option::Option<evidence::Sum>,
}
/// Nested message and enum types in `Evidence`.
pub mod evidence {
    /// The type of evidence.
    #[derive(::serde::Deserialize, ::serde::Serialize)]
    #[serde(tag = "type", content = "value")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        #[prost(message, tag = "1")]
        #[serde(rename = "tendermint/DuplicateVoteEvidence")]
        DuplicateVoteEvidence(super::DuplicateVoteEvidence),
        #[prost(message, tag = "2")]
        #[serde(rename = "tendermint/LightClientAttackEvidence")]
        LightClientAttackEvidence(super::LightClientAttackEvidence),
    }
}
/// DuplicateVoteEvidence contains evidence of a validator signed two conflicting votes.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DuplicateVoteEvidence {
    #[prost(message, optional, tag = "1")]
    pub vote_a: ::core::option::Option<Vote>,
    #[prost(message, optional, tag = "2")]
    pub vote_b: ::core::option::Option<Vote>,
    #[prost(int64, tag = "3")]
    #[serde(rename = "TotalVotingPower", with = "crate::serializers::from_str")]
    pub total_voting_power: i64,
    #[prost(int64, tag = "4")]
    #[serde(rename = "ValidatorPower", with = "crate::serializers::from_str")]
    pub validator_power: i64,
    #[prost(message, optional, tag = "5")]
    #[serde(rename = "Timestamp")]
    pub timestamp: ::core::option::Option<crate::google::protobuf::Timestamp>,
}
/// LightClientAttackEvidence contains evidence of a set of validators attempting to mislead a light client.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LightClientAttackEvidence {
    #[prost(message, optional, tag = "1")]
    pub conflicting_block: ::core::option::Option<LightBlock>,
    #[prost(int64, tag = "2")]
    #[serde(with = "crate::serializers::from_str")]
    pub common_height: i64,
    #[prost(message, repeated, tag = "3")]
    pub byzantine_validators: ::prost::alloc::vec::Vec<Validator>,
    #[prost(int64, tag = "4")]
    #[serde(with = "crate::serializers::from_str")]
    pub total_voting_power: i64,
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<crate::google::protobuf::Timestamp>,
}
/// EvidenceList is a list of evidence.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvidenceList {
    #[prost(message, repeated, tag = "1")]
    #[serde(with = "crate::serializers::nullable")]
    pub evidence: ::prost::alloc::vec::Vec<Evidence>,
}
/// Block defines the structure of a block in the CometBFT blockchain.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<Data>,
    #[prost(message, optional, tag = "3")]
    pub evidence: ::core::option::Option<EvidenceList>,
    #[prost(message, optional, tag = "4")]
    pub last_commit: ::core::option::Option<Commit>,
}
/// EventDataRoundState is emmitted with each new round step.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDataRoundState {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(int32, tag = "2")]
    pub round: i32,
    #[prost(string, tag = "3")]
    pub step: ::prost::alloc::string::String,
}
/// ConsensusParams contains consensus critical parameters that determine the
/// validity of blocks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusParams {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<BlockParams>,
    #[prost(message, optional, tag = "2")]
    pub evidence: ::core::option::Option<EvidenceParams>,
    #[prost(message, optional, tag = "3")]
    pub validator: ::core::option::Option<ValidatorParams>,
    #[prost(message, optional, tag = "4")]
    pub version: ::core::option::Option<VersionParams>,
    #[prost(message, optional, tag = "5")]
    pub abci: ::core::option::Option<AbciParams>,
}
/// BlockParams contains limits on the block size.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockParams {
    /// Max block size, in bytes.
    /// Note: must be greater than 0
    #[prost(int64, tag = "1")]
    pub max_bytes: i64,
    /// Max gas per block.
    /// Note: must be greater or equal to -1
    #[prost(int64, tag = "2")]
    pub max_gas: i64,
}
/// EvidenceParams determine how we handle evidence of malfeasance.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvidenceParams {
    /// Max age of evidence, in blocks.
    ///
    /// The basic formula for calculating this is: MaxAgeDuration / {average block
    /// time}.
    #[prost(int64, tag = "1")]
    #[serde(with = "crate::serializers::from_str", default)]
    pub max_age_num_blocks: i64,
    /// Max age of evidence, in time.
    ///
    /// It should correspond with an app's "unbonding period" or other similar
    /// mechanism for handling [Nothing-At-Stake
    /// attacks](<https://github.com/ethereum/wiki/wiki/Proof-of-Stake-FAQ#what-is-the-nothing-at-stake-problem-and-how-can-it-be-fixed>).
    #[prost(message, optional, tag = "2")]
    pub max_age_duration: ::core::option::Option<crate::google::protobuf::Duration>,
    /// This sets the maximum size of total evidence in bytes that can be committed in a single block.
    /// and should fall comfortably under the max block bytes.
    /// Default is 1048576 or 1MB
    #[prost(int64, tag = "3")]
    #[serde(with = "crate::serializers::from_str", default)]
    pub max_bytes: i64,
}
/// ValidatorParams restrict the public key types validators can use.
/// NOTE: uses ABCI pubkey naming, not Amino names.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorParams {
    #[prost(string, repeated, tag = "1")]
    pub pub_key_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// VersionParams contains the ABCI application version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionParams {
    /// Was named app_version in Tendermint 0.34
    #[prost(uint64, tag = "1")]
    pub app: u64,
}
/// HashedParams is a subset of ConsensusParams.
///
/// It is hashed into the Header.ConsensusHash.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashedParams {
    #[prost(int64, tag = "1")]
    pub block_max_bytes: i64,
    #[prost(int64, tag = "2")]
    pub block_max_gas: i64,
}
/// ABCIParams configure functionality specific to the Application Blockchain Interface.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbciParams {
    /// vote_extensions_enable_height configures the first height during which
    /// vote extensions will be enabled. During this specified height, and for all
    /// subsequent heights, precommit messages that do not contain valid extension data
    /// will be considered invalid. Prior to this height, vote extensions will not
    /// be used or accepted by validators on the network.
    ///
    /// Once enabled, vote extensions will be created by the application in ExtendVote,
    /// passed to the application for validation in VerifyVoteExtension and given
    /// to the application to use when proposing a block during PrepareProposal.
    #[prost(int64, tag = "1")]
    pub vote_extensions_enable_height: i64,
}
/// CanonicalBlockID is a canonical representation of a BlockID, which gets
/// serialized and signed.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalBlockId {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub part_set_header: ::core::option::Option<CanonicalPartSetHeader>,
}
/// CanonicalPartSetHeader is a canonical representation of a PartSetHeader,
/// which gets serialized and signed.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalPartSetHeader {
    #[prost(uint32, tag = "1")]
    pub total: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// CanonicalProposal is a canonical representation of a Proposal, which gets
/// serialized and signed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalProposal {
    /// type alias for byte
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    pub r#type: i32,
    /// canonicalization requires fixed size encoding here
    #[prost(sfixed64, tag = "2")]
    pub height: i64,
    /// canonicalization requires fixed size encoding here
    #[prost(sfixed64, tag = "3")]
    pub round: i64,
    #[prost(int64, tag = "4")]
    pub pol_round: i64,
    #[prost(message, optional, tag = "5")]
    pub block_id: ::core::option::Option<CanonicalBlockId>,
    #[prost(message, optional, tag = "6")]
    pub timestamp: ::core::option::Option<crate::google::protobuf::Timestamp>,
    #[prost(string, tag = "7")]
    pub chain_id: ::prost::alloc::string::String,
}
/// CanonicalVote is a canonical representation of a Vote, which gets
/// serialized and signed.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalVote {
    /// type alias for byte
    #[prost(enumeration = "SignedMsgType", tag = "1")]
    pub r#type: i32,
    /// canonicalization requires fixed size encoding here
    #[prost(sfixed64, tag = "2")]
    pub height: i64,
    /// canonicalization requires fixed size encoding here
    #[prost(sfixed64, tag = "3")]
    pub round: i64,
    #[prost(message, optional, tag = "4")]
    pub block_id: ::core::option::Option<CanonicalBlockId>,
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<crate::google::protobuf::Timestamp>,
    #[prost(string, tag = "6")]
    pub chain_id: ::prost::alloc::string::String,
}
/// CanonicalVoteExtension provides us a way to serialize a vote extension from
/// a particular validator such that we can sign over those serialized bytes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanonicalVoteExtension {
    #[prost(bytes = "vec", tag = "1")]
    pub extension: ::prost::alloc::vec::Vec<u8>,
    #[prost(sfixed64, tag = "2")]
    pub height: i64,
    #[prost(sfixed64, tag = "3")]
    pub round: i64,
    #[prost(string, tag = "4")]
    pub chain_id: ::prost::alloc::string::String,
}
