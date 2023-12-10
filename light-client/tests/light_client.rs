use std::{collections::HashMap, time::Duration};

use cometbft_light_client::{
    components::{
        io::{AtHeight, Io},
        scheduler,
    },
    errors::Error,
    light_client::LightClient,
    state::State,
    store::{memory::MemoryStore, LightStore},
    tests::*,
    verifier::{
        options::Options,
        types::{LightBlock, Status},
        ProdVerifier,
    },
};
use cometbft_testgen::{light_block::default_peer_id, Tester};

// Link to JSON test files repo:
// https://github.com/informalsystems/conformance-tests
const TEST_FILES_PATH: &str = "./tests/support/";

struct BisectionTestResult {
    untrusted_light_block: LightBlock,
    new_states: Result<Vec<LightBlock>, Error>,
}

fn run_test(tc: LightClientTest<LightBlock>) -> BisectionTestResult {
    let primary = default_peer_id();
    let untrusted_height = tc.height_to_verify;
    let trust_threshold = tc.trust_options.trust_level;
    let trusting_period = tc.trust_options.period;
    let now = tc.now;

    // In Go, default is 10 sec.
    // Once we switch to the proposer based timestamps, it will probably be a consensus parameter
    let clock_drift = Duration::from_secs(10);

    let clock = MockClock { now };

    let options = Options {
        trust_threshold,
        trusting_period: trusting_period.into(),
        clock_drift,
    };

    let provider = tc.primary;
    let io = MockIo::new(provider.lite_blocks);

    let trusted_height = tc.trust_options.height;
    let trusted_state = io
        .fetch_light_block(AtHeight::At(trusted_height))
        .expect("could not 'request' light block");

    let mut light_store = MemoryStore::new();
    light_store.insert(trusted_state, Status::Trusted);

    let mut state = State {
        light_store: Box::new(light_store),
        verification_trace: HashMap::new(),
    };

    let verifier = ProdVerifier::default();

    let mut light_client = LightClient::new(
        primary,
        options,
        clock,
        scheduler::basic_bisecting_schedule,
        verifier,
        io.clone(),
    );

    let result = verify_bisection(untrusted_height, &mut light_client, &mut state);

    let untrusted_light_block = io
        .fetch_light_block(AtHeight::At(untrusted_height))
        .expect("header at untrusted height not found");

    BisectionTestResult {
        untrusted_light_block,
        new_states: result,
    }
}

fn forward_test(tc: LightClientTest<LightBlock>) {
    let expect_error = match &tc.expected_output {
        Some(eo) => eo.eq("error"),
        None => false,
    };

    let test_result = run_test(tc);
    let expected_state = test_result.untrusted_light_block;

    match test_result.new_states {
        Ok(new_states) => {
            assert_eq!(new_states[0].height(), expected_state.height());
            assert_eq!(new_states[0], expected_state);
            assert!(!expect_error);
        },
        Err(e) => {
            if !expect_error {
                dbg!(e);
            }
            assert!(expect_error);
        },
    }
}

#[test]
fn run_tests() {
    let mut tester = Tester::new("light client verification", TEST_FILES_PATH);
    tester.add_test("forward verification with bisection", forward_test);
    tester.run_foreach_in_dir("bisection/single_peer");
    tester.finalize();
}
