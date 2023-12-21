/// A service for broadcasting transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteSignerError {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
/// PubKeyRequest requests the consensus public key from the remote signer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKeyRequest {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
}
/// PubKeyResponse is a response message containing the public key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKeyResponse {
    #[prost(message, optional, tag = "1")]
    pub pub_key: ::core::option::Option<super::super::crypto::v1::PublicKey>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// SignVoteRequest is a request to sign a vote
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignVoteRequest {
    #[prost(message, optional, tag = "1")]
    pub vote: ::core::option::Option<super::super::types::v1beta1::Vote>,
    #[prost(string, tag = "2")]
    pub chain_id: ::prost::alloc::string::String,
}
/// SignedVoteResponse is a response containing a signed vote or an error
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedVoteResponse {
    #[prost(message, optional, tag = "1")]
    pub vote: ::core::option::Option<super::super::types::v1beta1::Vote>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// SignProposalRequest is a request to sign a proposal
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignProposalRequest {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<super::super::types::v1beta1::Proposal>,
    #[prost(string, tag = "2")]
    pub chain_id: ::prost::alloc::string::String,
}
/// SignedProposalResponse is response containing a signed proposal or an error
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedProposalResponse {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<super::super::types::v1beta1::Proposal>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<RemoteSignerError>,
}
/// PingRequest is a request to confirm that the connection is alive.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {}
/// PingResponse is a response to confirm that the connection is alive.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {}
/// Message is an abstract message to/from the remote signer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// Sum of all possible messages.
    #[prost(oneof = "message::Sum", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub sum: ::core::option::Option<message::Sum>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    /// Sum of all possible messages.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        #[prost(message, tag = "1")]
        PubKeyRequest(super::PubKeyRequest),
        #[prost(message, tag = "2")]
        PubKeyResponse(super::PubKeyResponse),
        #[prost(message, tag = "3")]
        SignVoteRequest(super::SignVoteRequest),
        #[prost(message, tag = "4")]
        SignedVoteResponse(super::SignedVoteResponse),
        #[prost(message, tag = "5")]
        SignProposalRequest(super::SignProposalRequest),
        #[prost(message, tag = "6")]
        SignedProposalResponse(super::SignedProposalResponse),
        #[prost(message, tag = "7")]
        PingRequest(super::PingRequest),
        #[prost(message, tag = "8")]
        PingResponse(super::PingResponse),
    }
}
/// Errors is a list of error codes that can be returned by the remote signer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Errors {
    /// Unknown error
    Unknown = 0,
    /// Unexpected response
    UnexpectedResponse = 1,
    /// Connection lost
    NoConnection = 2,
    /// Connection timeout
    ConnectionTimeout = 3,
    /// Read timeout
    ReadTimeout = 4,
    /// Write timeout
    WriteTimeout = 5,
}
impl Errors {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Errors::Unknown => "ERRORS_UNKNOWN",
            Errors::UnexpectedResponse => "ERRORS_UNEXPECTED_RESPONSE",
            Errors::NoConnection => "ERRORS_NO_CONNECTION",
            Errors::ConnectionTimeout => "ERRORS_CONNECTION_TIMEOUT",
            Errors::ReadTimeout => "ERRORS_READ_TIMEOUT",
            Errors::WriteTimeout => "ERRORS_WRITE_TIMEOUT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ERRORS_UNKNOWN" => Some(Self::Unknown),
            "ERRORS_UNEXPECTED_RESPONSE" => Some(Self::UnexpectedResponse),
            "ERRORS_NO_CONNECTION" => Some(Self::NoConnection),
            "ERRORS_CONNECTION_TIMEOUT" => Some(Self::ConnectionTimeout),
            "ERRORS_READ_TIMEOUT" => Some(Self::ReadTimeout),
            "ERRORS_WRITE_TIMEOUT" => Some(Self::WriteTimeout),
            _ => None,
        }
    }
}
