//! Light Client integration tests.
//!
//! If you have a kvstore app running on 127.0.0.1:26657,
//! these can be run using:
//!
//!     cargo test
//!
//! Or else, if you have docker installed, you can tell the tests to run an endpoint,
//! by running:
//!
//!     cargo make
//!
//! (Make sure you install cargo-make using `cargo install cargo-make` first.)

use std::{convert::TryFrom, time::Duration};

use cometbft_light_client::{
    builder::LightClientBuilder,
    components::io::{AtHeight, Io, ProdIo},
    errors::Error,
    instance::Instance,
    store::{memory::MemoryStore, LightStore},
    verifier::{
        options::Options as LightClientOptions,
        types::{Height, PeerId, Status, TrustThreshold},
    },
};
use cometbft_rpc as rpc;

fn make_instance(peer_id: PeerId, options: LightClientOptions, address: rpc::Url) -> Instance {
    let rpc_client = rpc::HttpClient::new(address).unwrap();
    let io = ProdIo::new(peer_id, rpc_client.clone(), Some(Duration::from_secs(2)));
    let latest_block = io.fetch_light_block(AtHeight::Highest).unwrap();

    let mut light_store = Box::new(MemoryStore::new());
    light_store.insert(latest_block, Status::Trusted);

    LightClientBuilder::prod(
        peer_id,
        rpc_client,
        light_store,
        options,
        Some(Duration::from_secs(2)),
    )
    .trust_from_store()
    .unwrap()
    .build()
}

fn make_primary() -> Instance {
    let primary: PeerId = "BADFADAD0BEFEEDC0C0ADEADBEEFC0FFEEFACADE".parse().unwrap();

    // Because our CI infrastructure can only spawn a single Tendermint node at the moment,
    // we run this test against this very node as both the primary and witness.
    // In a production environment, one should make sure that the primary and witness are
    // different nodes, and check that the configured peer IDs match the ones returned
    // by the nodes.
    let node_address: rpc::Url = "http://127.0.0.1:26657".parse().unwrap();

    let options = LightClientOptions {
        trust_threshold: TrustThreshold::new(1, 3).unwrap(),
        trusting_period: Duration::from_secs(60 * 60), // 60 minutes
        clock_drift: Duration::from_secs(5 * 60),      // 5 minutes
    };

    make_instance(primary, options, node_address)
}

#[test]
fn forward() {
    let mut primary = make_primary();

    let max_iterations: usize = 10;

    for i in 1..=max_iterations {
        println!("[info ] - iteration {i}/{max_iterations}");

        match primary.light_client.verify_to_highest(&mut primary.state) {
            Ok(light_block) => {
                println!("[info ] synced to block {}", light_block.height());
            },
            Err(err) => {
                println!("[error] sync failed: {err}");
                panic!("failed to sync to highest: {err}");
            },
        }

        std::thread::sleep(Duration::from_millis(800));
    }
}

#[test]
fn backward() -> Result<(), Error> {
    let mut primary = make_primary();

    let max_iterations: usize = 10;

    // Sleep a little bit to ensure we have a few blocks already
    std::thread::sleep(Duration::from_secs(2));

    for i in 1..=max_iterations {
        println!("[info ] - iteration {i}/{max_iterations}");

        // First we sync to the highest block to have a high enough trusted state
        let trusted_state = primary.light_client.verify_to_highest(&mut primary.state)?;
        println!("[info ] synced to highest block {}", trusted_state.height());

        // Then we pick a height below the trusted state
        let target_height = Height::try_from(trusted_state.height().value() / 2).unwrap();

        // We now try to verify a block at this height
        let light_block = primary
            .light_client
            .verify_to_target(target_height, &mut primary.state)?;

        println!("[info ] verified lower block {}", light_block.height());

        std::thread::sleep(Duration::from_millis(800));
    }

    Ok(())
}
