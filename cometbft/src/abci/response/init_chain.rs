use crate::AppHash;

use crate::{consensus, prelude::*, validator};

#[doc = include_str!("../doc/response-initchain.md")]
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct InitChain {
    /// Initial consensus-critical parameters (optional).
    pub consensus_params: Option<consensus::Params>,
    /// Initial validator set (optional).
    ///
    /// If this list is empty, the initial validator set will be the one given in
    /// [`request::InitChain::validators`](super::super::request::InitChain::validators).
    ///
    /// If this list is nonempty, it will be the initial validator set, instead
    /// of the one given in
    /// [`request::InitChain::validators`](super::super::request::InitChain::validators).
    pub validators: Vec<validator::Update>,
    /// Initial application hash.
    pub app_hash: AppHash,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1 {
    use super::InitChain;
    use cometbft_proto::abci::v1 as pb;
    use cometbft_proto::Protobuf;

    impl From<InitChain> for pb::InitChainResponse {
        fn from(init_chain: InitChain) -> Self {
            Self {
                consensus_params: init_chain.consensus_params.map(Into::into),
                validators: init_chain.validators.into_iter().map(Into::into).collect(),
                app_hash: init_chain.app_hash.into(),
            }
        }
    }

    impl TryFrom<pb::InitChainResponse> for InitChain {
        type Error = crate::Error;

        fn try_from(init_chain: pb::InitChainResponse) -> Result<Self, Self::Error> {
            Ok(Self {
                consensus_params: init_chain
                    .consensus_params
                    .map(TryInto::try_into)
                    .transpose()?,
                validators: init_chain
                    .validators
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                app_hash: init_chain.app_hash.try_into()?,
            })
        }
    }

    impl Protobuf<pb::InitChainResponse> for InitChain {}
}

mod v1beta1 {
    use super::InitChain;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<InitChain> for pb::ResponseInitChain {
        fn from(init_chain: InitChain) -> Self {
            Self {
                consensus_params: init_chain.consensus_params.map(Into::into),
                validators: init_chain.validators.into_iter().map(Into::into).collect(),
                app_hash: init_chain.app_hash.into(),
            }
        }
    }

    impl TryFrom<pb::ResponseInitChain> for InitChain {
        type Error = crate::Error;

        fn try_from(init_chain: pb::ResponseInitChain) -> Result<Self, Self::Error> {
            Ok(Self {
                consensus_params: init_chain
                    .consensus_params
                    .map(TryInto::try_into)
                    .transpose()?,
                validators: init_chain
                    .validators
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                app_hash: init_chain.app_hash.try_into()?,
            })
        }
    }

    impl Protobuf<pb::ResponseInitChain> for InitChain {}
}

mod v1beta2 {
    use super::InitChain;
    use cometbft_proto::abci::v1beta2 as pb;
    use cometbft_proto::Protobuf;

    impl From<InitChain> for pb::ResponseInitChain {
        fn from(init_chain: InitChain) -> Self {
            Self {
                consensus_params: init_chain.consensus_params.map(Into::into),
                validators: init_chain.validators.into_iter().map(Into::into).collect(),
                app_hash: init_chain.app_hash.into(),
            }
        }
    }

    impl TryFrom<pb::ResponseInitChain> for InitChain {
        type Error = crate::Error;

        fn try_from(init_chain: pb::ResponseInitChain) -> Result<Self, Self::Error> {
            Ok(Self {
                consensus_params: init_chain
                    .consensus_params
                    .map(TryInto::try_into)
                    .transpose()?,
                validators: init_chain
                    .validators
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                app_hash: init_chain.app_hash.try_into()?,
            })
        }
    }

    impl Protobuf<pb::ResponseInitChain> for InitChain {}
}

mod v1beta3 {
    use super::InitChain;
    use cometbft_proto::abci::v1beta3 as pb;
    use cometbft_proto::Protobuf;

    impl From<InitChain> for pb::ResponseInitChain {
        fn from(init_chain: InitChain) -> Self {
            Self {
                consensus_params: init_chain.consensus_params.map(Into::into),
                validators: init_chain.validators.into_iter().map(Into::into).collect(),
                app_hash: init_chain.app_hash.into(),
            }
        }
    }

    impl TryFrom<pb::ResponseInitChain> for InitChain {
        type Error = crate::Error;

        fn try_from(init_chain: pb::ResponseInitChain) -> Result<Self, Self::Error> {
            Ok(Self {
                consensus_params: init_chain
                    .consensus_params
                    .map(TryInto::try_into)
                    .transpose()?,
                validators: init_chain
                    .validators
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                app_hash: init_chain.app_hash.try_into()?,
            })
        }
    }

    impl Protobuf<pb::ResponseInitChain> for InitChain {}
}
