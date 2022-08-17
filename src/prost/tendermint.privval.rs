#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteSignerError {
    #[prost(int32, tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub description: std::string::String,
}
/// PubKeyRequest requests the consensus public key from the remote signer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKeyRequest {
    #[prost(string, tag="1")]
    pub chain_id: std::string::String,
}
/// PubKeyResponse is a response message containing the public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKeyResponse {
    #[prost(message, optional, tag="1")]
    pub pub_key: ::std::option::Option<super::crypto::PublicKey>,
    #[prost(message, optional, tag="2")]
    pub error: ::std::option::Option<RemoteSignerError>,
}
/// SignVoteRequest is a request to sign a vote
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignVoteRequest {
    #[prost(message, optional, tag="1")]
    pub vote: ::std::option::Option<super::types::Vote>,
    #[prost(string, tag="2")]
    pub chain_id: std::string::String,
}
/// SignedVoteResponse is a response containing a signed vote or an error
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedVoteResponse {
    #[prost(message, optional, tag="1")]
    pub vote: ::std::option::Option<super::types::Vote>,
    #[prost(message, optional, tag="2")]
    pub error: ::std::option::Option<RemoteSignerError>,
}
/// SignProposalRequest is a request to sign a proposal
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignProposalRequest {
    #[prost(message, optional, tag="1")]
    pub proposal: ::std::option::Option<super::types::Proposal>,
    #[prost(string, tag="2")]
    pub chain_id: std::string::String,
}
/// SignedProposalResponse is response containing a signed proposal or an error
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedProposalResponse {
    #[prost(message, optional, tag="1")]
    pub proposal: ::std::option::Option<super::types::Proposal>,
    #[prost(message, optional, tag="2")]
    pub error: ::std::option::Option<RemoteSignerError>,
}
/// PingRequest is a request to confirm that the connection is alive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
}
/// PingResponse is a response to confirm that the connection is alive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(oneof="message::Sum", tags="1, 2, 3, 4, 5, 6, 7, 8")]
    pub sum: ::std::option::Option<message::Sum>,
}
pub mod message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        #[prost(message, tag="1")]
        PubKeyRequest(super::PubKeyRequest),
        #[prost(message, tag="2")]
        PubKeyResponse(super::PubKeyResponse),
        #[prost(message, tag="3")]
        SignVoteRequest(super::SignVoteRequest),
        #[prost(message, tag="4")]
        SignedVoteResponse(super::SignedVoteResponse),
        #[prost(message, tag="5")]
        SignProposalRequest(super::SignProposalRequest),
        #[prost(message, tag="6")]
        SignedProposalResponse(super::SignedProposalResponse),
        #[prost(message, tag="7")]
        PingRequest(super::PingRequest),
        #[prost(message, tag="8")]
        PingResponse(super::PingResponse),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Errors {
    Unknown = 0,
    UnexpectedResponse = 1,
    NoConnection = 2,
    ConnectionTimeout = 3,
    ReadTimeout = 4,
    WriteTimeout = 5,
}
