//! cometbft-proto auto-generated sub-modules for CometBFT. DO NOT EDIT

pub mod abci {
    pub mod v1 {
        include!("../prost/v1/cometbft.abci.v1.rs");
    }
    pub mod v1beta1 {
        include!("../prost/v1/cometbft.abci.v1beta1.rs");
    }
    pub mod v1beta2 {
        include!("../prost/v1/cometbft.abci.v1beta2.rs");
    }
    pub mod v1beta3 {
        include!("../prost/v1/cometbft.abci.v1beta3.rs");
    }
}

pub mod blocksync {
    pub mod v1 {
        include!("../prost/v1/cometbft.blocksync.v1.rs");
    }
    pub mod v1beta1 {
        include!("../prost/v1/cometbft.blocksync.v1beta1.rs");
    }
}

pub mod consensus {
    pub mod v1 {
        include!("../prost/v1/cometbft.consensus.v1.rs");
    }
    pub mod v1beta1 {
        include!("../prost/v1/cometbft.consensus.v1beta1.rs");
    }
}

pub mod crypto {
    pub mod v1 {
        include!("../prost/v1/cometbft.crypto.v1.rs");
    }
}

pub mod libs {
    pub mod bits {
        pub mod v1 {
            include!("../prost/v1/cometbft.libs.bits.v1.rs");
        }
    }
}

pub mod mempool {
    pub mod v1 {
        include!("../prost/v1/cometbft.mempool.v1.rs");
    }
}

pub mod p2p {
    pub mod v1 {
        include!("../prost/v1/cometbft.p2p.v1.rs");
    }
}

pub mod privval {
    pub mod v1 {
        include!("../prost/v1/cometbft.privval.v1.rs");
    }
    pub mod v1beta1 {
        include!("../prost/v1/cometbft.privval.v1beta1.rs");
    }
}

pub mod rpc {
    pub mod grpc {
        pub mod v1beta1 {
            include!("../prost/v1/cometbft.rpc.grpc.v1beta1.rs");
        }
        pub mod v1beta2 {
            include!("../prost/v1/cometbft.rpc.grpc.v1beta2.rs");
        }
        pub mod v1beta3 {
            include!("../prost/v1/cometbft.rpc.grpc.v1beta3.rs");
        }
    }
}

pub mod services {
    pub mod block {
        pub mod v1 {
            include!("../prost/v1/cometbft.services.block.v1.rs");
        }
    }
    pub mod block_results {
        pub mod v1 {
            include!("../prost/v1/cometbft.services.block_results.v1.rs");
        }
    }
    pub mod pruning {
        pub mod v1 {
            include!("../prost/v1/cometbft.services.pruning.v1.rs");
        }
    }
    pub mod version {
        pub mod v1 {
            include!("../prost/v1/cometbft.services.version.v1.rs");
        }
    }
}

pub mod state {
    pub mod v1 {
        include!("../prost/v1/cometbft.state.v1.rs");
    }
    pub mod v1beta1 {
        include!("../prost/v1/cometbft.state.v1beta1.rs");
    }
    pub mod v1beta2 {
        include!("../prost/v1/cometbft.state.v1beta2.rs");
    }
    pub mod v1beta3 {
        include!("../prost/v1/cometbft.state.v1beta3.rs");
    }
}

pub mod statesync {
    pub mod v1 {
        include!("../prost/v1/cometbft.statesync.v1.rs");
    }
}

pub mod store {
    pub mod v1 {
        include!("../prost/v1/cometbft.store.v1.rs");
    }
}

pub mod types {
    pub mod v1 {
        include!("../prost/v1/cometbft.types.v1.rs");
    }
    pub mod v1beta1 {
        include!("../prost/v1/cometbft.types.v1beta1.rs");
    }
    pub mod v1beta2 {
        include!("../prost/v1/cometbft.types.v1beta2.rs");
    }
}

pub mod version {
    pub mod v1 {
        include!("../prost/v1/cometbft.version.v1.rs");
    }
}

pub mod meta {
    pub const REPOSITORY: &str = "https://github.com/cometbft/cometbft";
    pub const COMMITISH: &str = "v1.0.0-alpha.1";
}
