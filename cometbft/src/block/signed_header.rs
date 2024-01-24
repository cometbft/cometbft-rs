//! SignedHeader contains commit and and block header.
//! It is what the rpc endpoint /commit returns and hence can be used by a
//! light client.

use cometbft_proto::types::v1::SignedHeader as RawSignedHeader;
use serde::{Deserialize, Serialize};

use crate::{block, Error};

/// Signed block headers
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawSignedHeader", into = "RawSignedHeader")] // used by RPC /commit endpoint
#[non_exhaustive]
pub struct SignedHeader {
    /// Block header
    pub header: block::Header,
    /// Commit containing signatures for the header
    pub commit: block::Commit,
}

mod v1 {
    use super::SignedHeader;
    use crate::Error;
    use cometbft_proto::types::v1 as pb;

    impl TryFrom<pb::SignedHeader> for SignedHeader {
        type Error = Error;

        fn try_from(value: pb::SignedHeader) -> Result<Self, Self::Error> {
            let header = value
                .header
                .ok_or_else(Error::invalid_signed_header)?
                .try_into()?;
            let commit = value
                .commit
                .ok_or_else(Error::invalid_signed_header)?
                .try_into()?;
            Self::new(header, commit) // Additional checks
        }
    }

    impl From<SignedHeader> for pb::SignedHeader {
        fn from(value: SignedHeader) -> Self {
            pb::SignedHeader {
                header: Some(value.header.into()),
                commit: Some(value.commit.into()),
            }
        }
    }
}

mod v1beta1 {
    use super::SignedHeader;
    use crate::Error;
    use cometbft_proto::types::v1beta1 as pb;

    impl TryFrom<pb::SignedHeader> for SignedHeader {
        type Error = Error;

        fn try_from(value: pb::SignedHeader) -> Result<Self, Self::Error> {
            let header = value
                .header
                .ok_or_else(Error::invalid_signed_header)?
                .try_into()?;
            let commit = value
                .commit
                .ok_or_else(Error::invalid_signed_header)?
                .try_into()?;
            Self::new(header, commit) // Additional checks
        }
    }

    impl From<SignedHeader> for pb::SignedHeader {
        fn from(value: SignedHeader) -> Self {
            pb::SignedHeader {
                header: Some(value.header.into()),
                commit: Some(value.commit.into()),
            }
        }
    }
}

impl SignedHeader {
    /// Constructor.
    pub fn new(header: block::Header, commit: block::Commit) -> Result<Self, Error> {
        if header.height != commit.height {
            return Err(Error::invalid_signed_header());
        }
        Ok(Self { header, commit })
    }

    /// Get header
    pub fn header(&self) -> &block::Header {
        &self.header
    }

    /// Get commit
    pub fn commit(&self) -> &block::Commit {
        &self.commit
    }
}
