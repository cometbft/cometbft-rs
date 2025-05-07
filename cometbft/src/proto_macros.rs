//! Macros to facilitate protobuf conversions

macro_rules! cometbft_old_pb_modules {
    ($name:ident, { $($contents:item)* }) => {
        mod v0_34 {
            use cometbft_proto::v0_34::$name as pb;
            #[allow(unused_imports)]
            use cometbft_proto::Protobuf;

            $($contents)*
        }
        mod v0_37 {
            use cometbft_proto::v0_37::$name as pb;
            #[allow(unused_imports)]
            use cometbft_proto::Protobuf;

            $($contents)*
        }
        mod v0_38 {
            use cometbft_proto::v0_38::$name as pb;
            #[allow(unused_imports)]
            use cometbft_proto::Protobuf;

            $($contents)*
        }
    };
}

macro_rules! cometbft_pb_modules {
    ($name:ident, { $($contents:item)* }) => {
        mod v1 {
            use cometbft_proto::v1::$name::v1 as pb;
            #[allow(unused_imports)]
            use cometbft_proto::Protobuf;

            $($contents)*
        }
    };
}
