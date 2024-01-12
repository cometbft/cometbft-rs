use crate::prelude::*;

#[doc = include_str!("../doc/request-endblock.md")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EndBlock {
    /// The height of the block just executed.
    pub height: i64,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

mod v1beta1 {
    use super::EndBlock;
    use cometbft_proto::abci::v1beta1 as pb;
    use cometbft_proto::Protobuf;

    impl From<EndBlock> for pb::RequestEndBlock {
        fn from(end_block: EndBlock) -> Self {
            Self {
                height: end_block.height,
            }
        }
    }

    impl TryFrom<pb::RequestEndBlock> for EndBlock {
        type Error = crate::Error;

        fn try_from(end_block: pb::RequestEndBlock) -> Result<Self, Self::Error> {
            Ok(Self {
                height: end_block.height,
            })
        }
    }

    impl Protobuf<pb::RequestEndBlock> for EndBlock {}
}
