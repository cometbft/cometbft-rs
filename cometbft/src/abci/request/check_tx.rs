use bytes::Bytes;

use crate::prelude::*;

#[doc = include_str!("../doc/request-checktx.md")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CheckTx {
    /// The transaction bytes.
    pub tx: Bytes,
    /// The kind of check to perform.
    ///
    /// Note: this field is called `type` in the protobuf, but we call it `kind`
    /// to avoid the Rust keyword.
    pub kind: CheckTxKind,
}

/// The possible kinds of [`CheckTx`] checks.
///
/// Note: the
/// [ABCI documentation](https://docs.cometbft.com/v1/spec/abci/abci.html#checktx)
/// calls this `CheckTxType`, but we follow the Rust convention and name it `CheckTxKind`
/// to avoid confusion with Rust types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
#[derive(Default)]
pub enum CheckTxKind {
    /// A full check is required (the default).
    #[default]
    New = 0,
    /// Indicates that the mempool is initiating a recheck of the transaction.
    Recheck = 1,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

cometbft_old_pb_modules! {
    use super::{CheckTx, CheckTxKind};

    impl From<CheckTx> for pb::abci::RequestCheckTx {
        fn from(check_tx: CheckTx) -> Self {
            Self {
                tx: check_tx.tx,
                r#type: check_tx.kind as i32,
            }
        }
    }

    impl TryFrom<pb::abci::RequestCheckTx> for CheckTx {
        type Error = crate::Error;

        fn try_from(check_tx: pb::abci::RequestCheckTx) -> Result<Self, Self::Error> {
            let kind = match check_tx.r#type {
                0 => CheckTxKind::New,
                1 => CheckTxKind::Recheck,
                _ => return Err(crate::Error::unsupported_check_tx_type()),
            };
            Ok(Self {
                tx: check_tx.tx,
                kind,
            })
        }
    }

    impl Protobuf<pb::abci::RequestCheckTx> for CheckTx {}
}

mod v1 {
    use super::{CheckTx, CheckTxKind};
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<CheckTx> for pb::CheckTxRequest {
        fn from(check_tx: CheckTx) -> Self {
            Self {
                tx: check_tx.tx,
                r#type: check_tx.kind as i32,
            }
        }
    }

    impl TryFrom<pb::CheckTxRequest> for CheckTx {
        type Error = crate::Error;

        fn try_from(check_tx: pb::CheckTxRequest) -> Result<Self, Self::Error> {
            let kind = match check_tx.r#type {
                0 => CheckTxKind::New,
                1 => CheckTxKind::Recheck,
                _ => return Err(crate::Error::unsupported_check_tx_type()),
            };
            Ok(Self {
                tx: check_tx.tx,
                kind,
            })
        }
    }

    impl Protobuf<pb::CheckTxRequest> for CheckTx {}
}

mod v1beta1 {
    use super::{CheckTx, CheckTxKind};
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<CheckTx> for pb::RequestCheckTx {
        fn from(check_tx: CheckTx) -> Self {
            Self {
                tx: check_tx.tx,
                r#type: check_tx.kind as i32,
            }
        }
    }

    impl TryFrom<pb::RequestCheckTx> for CheckTx {
        type Error = crate::Error;

        fn try_from(check_tx: pb::RequestCheckTx) -> Result<Self, Self::Error> {
            let kind = match check_tx.r#type {
                0 => CheckTxKind::New,
                1 => CheckTxKind::Recheck,
                _ => return Err(crate::Error::unsupported_check_tx_type()),
            };
            Ok(Self {
                tx: check_tx.tx,
                kind,
            })
        }
    }

    impl Protobuf<pb::RequestCheckTx> for CheckTx {}
}
