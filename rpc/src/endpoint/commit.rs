//! `/commit` endpoint JSON-RPC wrapper

use cometbft::{block, block::signed_header::SignedHeader};
use serde::{Deserialize, Serialize};

use crate::{dialect::Dialect, request::RequestMessage};

/// Get commit information about a specific block
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    pub height: Option<block::Height>,
}

impl Request {
    /// Create a new request for commit info about a particular block
    pub fn new(height: block::Height) -> Self {
        Self {
            height: Some(height),
        }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> crate::Method {
        crate::Method::Commit
    }
}

impl<S: Dialect> crate::Request<S> for Request {
    type Response = Response;
}

impl<S: Dialect> crate::SimpleRequest<S> for Request {
    type Output = Response;
}

/// Commit responses
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// Signed header
    pub signed_header: SignedHeader,

    /// Is the signed header canonical?
    pub canonical: bool,
}

impl crate::Response for Response {}
