use super::*;

#[test]
fn outgoing_fixtures() {
    for json_file in find_fixtures("v0_37", "outgoing") {
        let file_name = json_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".json")
            .unwrap();
        let content = fs::read_to_string(&json_file).unwrap();
        match file_name {
            "abci_info" => assert!(serde_json::from_str::<
                RequestWrapper<endpoint::abci_info::Request>,
            >(&content)
            .is_ok()),
            "abci_query_with_existing_key" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::abci_query::Request>>(&content)
                        .unwrap();
                assert!(wrapped.params().path.is_none());
                assert_eq!(wrapped.params().data, hex::decode("747830").unwrap());
                assert!(wrapped.params().height.is_none());
                assert!(!wrapped.params().prove);
            },
            "abci_query_with_non_existent_key" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::abci_query::Request>>(&content)
                        .unwrap();
                assert!(wrapped.params().path.is_none());
                assert_eq!(
                    wrapped.params().data,
                    hex::decode("6e6f6e5f6578697374656e745f6b6579").unwrap()
                );
                assert!(wrapped.params().height.is_none());
                assert!(!wrapped.params().prove);
            },
            "block_at_height_0" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::block::Request>>(&content)
                        .unwrap();
                assert_eq!(wrapped.params().height.unwrap().value(), 0);
            },
            "block_at_height_1" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::block::Request>>(&content)
                        .unwrap();
                assert_eq!(wrapped.params().height.unwrap().value(), 1);
            },
            "block_at_height_10" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::block::Request>>(&content)
                        .unwrap();
                assert_eq!(wrapped.params().height.unwrap().value(), 10);
            },
            "block_by_hash" => {
                let wrapped = serde_json::from_str::<
                    RequestWrapper<endpoint::block_by_hash::Request>,
                >(&content)
                .unwrap();
                assert_eq!(
                    wrapped.params().hash.unwrap().to_string(),
                    "FCF9C2537FC3534CA71001FE1F14C4F769090948C1A521682F612E7CF73AE639"
                );
            },
            "block_results_at_height_10" => {
                let wrapped = serde_json::from_str::<
                    RequestWrapper<endpoint::block_results::Request>,
                >(&content)
                .unwrap();
                assert_eq!(wrapped.params().height.unwrap().value(), 10);
            },
            "block_search" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::block_search::Request>>(
                        &content,
                    )
                    .unwrap();
                assert_eq!(wrapped.params().query, "block.height > 1");
                assert_eq!(wrapped.params().page, 1);
                assert_eq!(wrapped.params().per_page, 100);
                assert_eq!(wrapped.params().order_by, Order::Ascending);
            },
            "blockchain_from_1_to_10" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::blockchain::Request>>(&content)
                        .unwrap();
                assert_eq!(wrapped.params().min_height.value(), 1);
                assert_eq!(wrapped.params().max_height.value(), 10);
            },
            "broadcast_tx_async" => {
                let wrapped = serde_json::from_str::<
                    RequestWrapper<endpoint::broadcast::tx_async::Request>,
                >(&content)
                .unwrap();
                assert_eq!(
                    wrapped.params().tx,
                    base64::decode("YXN5bmMta2V5PXZhbHVl").unwrap()
                );
            },
            "broadcast_tx_commit" => {
                let wrapped = serde_json::from_str::<
                    RequestWrapper<endpoint::broadcast::tx_commit::Request>,
                >(&content)
                .unwrap();
                assert_eq!(
                    wrapped.params().tx,
                    base64::decode("Y29tbWl0LWtleT12YWx1ZQ==").unwrap()
                );
            },
            "broadcast_tx_sync" => {
                let wrapped = serde_json::from_str::<
                    RequestWrapper<endpoint::broadcast::tx_sync::Request>,
                >(&content)
                .unwrap();
                assert_eq!(
                    wrapped.params().tx,
                    base64::decode("c3luYy1rZXk9dmFsdWU=").unwrap()
                );
            },
            "commit_at_height_10" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::commit::Request>>(&content)
                        .unwrap();
                assert_eq!(wrapped.params().height.unwrap().value(), 10);
            },
            "consensus_params" => {
                let wrapped = serde_json::from_str::<
                    RequestWrapper<endpoint::consensus_params::Request>,
                >(&content)
                .unwrap();
                let height = wrapped.params().height.unwrap();
                assert_eq!(u64::from(height), 10u64);
            },
            "consensus_state" => assert!(serde_json::from_str::<
                RequestWrapper<endpoint::consensus_state::Request>,
            >(&content)
            .is_ok()),
            "genesis" => assert!(serde_json::from_str::<
                RequestWrapper<endpoint::genesis::Request::<serde_json::Value>>,
            >(&content)
            .is_ok()),
            "net_info" => assert!(serde_json::from_str::<
                RequestWrapper<endpoint::net_info::Request>,
            >(&content)
            .is_ok()),
            "status" => assert!(
                serde_json::from_str::<RequestWrapper<endpoint::status::Request>>(&content).is_ok()
            ),
            "subscribe_malformed" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::subscribe::Request>>(&content)
                        .unwrap();
                assert_eq!(wrapped.params().query, "malformed query");
            },
            "subscribe_newblock" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::subscribe::Request>>(&content)
                        .unwrap();
                assert_eq!(wrapped.params().query, "tm.event = 'NewBlock'");
            },
            "subscribe_txs" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::subscribe::Request>>(&content)
                        .unwrap();
                assert_eq!(wrapped.params().query, "tm.event = 'Tx'");
            },
            "subscribe_txs_broadcast_tx_0" => {
                let wrapped = serde_json::from_str::<
                    RequestWrapper<endpoint::broadcast::tx_async::Request>,
                >(&content)
                .unwrap();
                assert_eq!(wrapped.params().tx, base64::decode("dHgwPXZhbHVl").unwrap());
            },
            "subscribe_txs_broadcast_tx_1" => {
                let wrapped = serde_json::from_str::<
                    RequestWrapper<endpoint::broadcast::tx_async::Request>,
                >(&content)
                .unwrap();
                assert_eq!(wrapped.params().tx, base64::decode("dHgxPXZhbHVl").unwrap());
            },
            "subscribe_txs_broadcast_tx_2" => {
                let wrapped = serde_json::from_str::<
                    RequestWrapper<endpoint::broadcast::tx_async::Request>,
                >(&content)
                .unwrap();
                assert_eq!(wrapped.params().tx, base64::decode("dHgyPXZhbHVl").unwrap());
            },
            "subscribe_txs_broadcast_tx_3" => {
                let wrapped = serde_json::from_str::<
                    RequestWrapper<endpoint::broadcast::tx_async::Request>,
                >(&content)
                .unwrap();
                assert_eq!(wrapped.params().tx, base64::decode("dHgzPXZhbHVl").unwrap());
            },
            "subscribe_txs_broadcast_tx_4" => {
                let wrapped = serde_json::from_str::<
                    RequestWrapper<endpoint::broadcast::tx_async::Request>,
                >(&content)
                .unwrap();
                assert_eq!(wrapped.params().tx, base64::decode("dHg0PXZhbHVl").unwrap());
            },
            "subscribe_txs_broadcast_tx_5" => {
                let wrapped = serde_json::from_str::<
                    RequestWrapper<endpoint::broadcast::tx_async::Request>,
                >(&content)
                .unwrap();
                assert_eq!(wrapped.params().tx, base64::decode("dHg1PXZhbHVl").unwrap());
            },
            "tx" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::tx::Request>>(&content)
                        .unwrap();
                assert_eq!(
                    wrapped.params().hash,
                    Hash::from_bytes(
                        Algorithm::Sha256,
                        &[
                            214, 63, 156, 35, 121, 30, 97, 4, 16, 181, 118, 216, 194, 123, 181,
                            174, 172, 147, 204, 26, 88, 82, 36, 40, 167, 179, 42, 18, 118, 8, 88,
                            96
                        ]
                    )
                    .unwrap()
                );
                assert!(!wrapped.params().prove);
            },
            "tx_search_no_prove" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::tx_search::Request>>(&content)
                        .unwrap();
                assert_eq!(wrapped.params().query, "tx.height > 1");
                assert!(!wrapped.params().prove);
                assert_eq!(wrapped.params().page, 1);
                assert_eq!(wrapped.params().per_page, 10);
                assert_eq!(wrapped.params().order_by, Order::Ascending);
            },
            "tx_search_with_prove" => {
                let wrapped =
                    serde_json::from_str::<RequestWrapper<endpoint::tx_search::Request>>(&content)
                        .unwrap();
                assert_eq!(wrapped.params().query, "tx.height > 1");
                assert!(wrapped.params().prove);
                assert_eq!(wrapped.params().page, 1);
                assert_eq!(wrapped.params().per_page, 10);
                assert_eq!(wrapped.params().order_by, Order::Ascending);
            },
            _ => {
                panic!("cannot parse file name: {file_name}");
            },
        }
    }
}

#[test]
fn incoming_fixtures() {
    use cometbft_rpc::event::{v0_37::DeEvent, EventData};

    let empty_merkle_root_hash = Some(
        cometbft::Hash::from_hex_upper(
            cometbft::hash::Algorithm::Sha256,
            "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
        )
        .unwrap(),
    );
    let informal_epoch =
        cometbft::Time::parse_from_rfc3339("2020-01-01T00:00:00.000000000Z").unwrap();

    for json_file in find_fixtures("v0_37", "incoming") {
        let file_name = json_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".json")
            .unwrap();
        let content = fs::read_to_string(&json_file).unwrap();
        match file_name {
            "abci_info" => {
                let result = endpoint::abci_info::Response::from_string(content).unwrap();
                assert_eq!(result.response.app_version, 1);
                assert_eq!(result.response.data, "{\"size\":9}");
                assert_eq!(
                    result.response.last_block_app_hash.as_bytes(),
                    b"EgAAAAAAAAA="
                );
                assert_eq!(result.response.version, "1.0.0");
            },
            "abci_query_with_existing_key" => {
                let result = endpoint::abci_query::Response::from_string(content).unwrap();
                assert_eq!(result.response.code.value(), 0);
                assert!(result.response.codespace.is_empty());
                assert_eq!(result.response.index, 0);
                assert!(result.response.info.is_empty());
                assert_eq!(result.response.key, base64::decode("dHgw").unwrap());
                assert_eq!(result.response.log, "exists");
                assert!(result.response.proof.is_none());
                assert_eq!(result.response.value, base64::decode("dmFsdWU=").unwrap());
            },
            "abci_query_with_non_existent_key" => {
                let result = endpoint::abci_query::Response::from_string(content).unwrap();
                assert_eq!(result.response.code.value(), 0);
                assert!(result.response.codespace.is_empty());
                assert_eq!(result.response.index, 0);
                assert!(result.response.info.is_empty());
                assert_eq!(
                    result.response.key,
                    base64::decode("bm9uX2V4aXN0ZW50X2tleQ==").unwrap()
                );
                assert_eq!(result.response.log, "does not exist");
                assert!(result.response.proof.is_none());
                assert!(result.response.value.is_empty());
            },
            "block_at_height_0" => {
                let res = endpoint::block::Response::from_string(&content);

                match res {
                    Err(Error(ErrorDetail::Response(e), _)) => {
                        let response = e.source;
                        assert_eq!(response.code(), Code::InternalError);
                        assert_eq!(response.message(), "Internal error");
                        assert_eq!(
                            response.data(),
                            Some("height must be greater than 0, but got 0")
                        );
                    },
                    _ => panic!("expected Response error"),
                }
            },
            "block_at_height_1" => {
                let result = endpoint::block::Response::from_string(content).unwrap();
                assert!(result.block.data.first().is_none());
                assert!(result.block.evidence.iter().next().is_none());
                assert!(result.block.header.app_hash.as_bytes().is_empty());
                assert_eq!(result.block.header.chain_id.as_str(), CHAIN_ID);
                assert!(!result.block.header.consensus_hash.is_empty());
                assert_eq!(result.block.header.data_hash, empty_merkle_root_hash);
                assert_eq!(result.block.header.evidence_hash, empty_merkle_root_hash);
                assert_eq!(result.block.header.height.value(), 1);
                assert!(result.block.header.last_block_id.is_none());
                assert_eq!(result.block.header.last_commit_hash, empty_merkle_root_hash);
                assert_eq!(
                    result.block.header.last_results_hash,
                    empty_merkle_root_hash
                );
                assert!(!result.block.header.next_validators_hash.is_empty());
                assert_ne!(
                    result.block.header.proposer_address.as_bytes(),
                    [0u8; cometbft::account::LENGTH]
                );
                assert!(
                    result
                        .block
                        .header
                        .time
                        .duration_since(informal_epoch)
                        .unwrap()
                        .as_secs()
                        > 0
                );
                assert!(!result.block.header.validators_hash.is_empty());
                assert_eq!(
                    result.block.header.version,
                    cometbft::block::header::Version { block: 11, app: 1 }
                );
                assert!(result.block.last_commit.is_none());
                assert!(!result.block_id.hash.is_empty());
                assert!(!result.block_id.part_set_header.hash.is_empty());
                assert_eq!(result.block_id.part_set_header.total, 1);
            },
            "block_at_height_10" => {
                let result = endpoint::block::Response::from_string(content).unwrap();
                assert!(result.block.data.first().is_none());
                assert!(result.block.evidence.iter().next().is_none());
                assert_eq!(result.block.header.app_hash.as_bytes(), &[0u8; 8]);
                assert_eq!(result.block.header.chain_id.as_str(), CHAIN_ID);
                assert!(!result.block.header.consensus_hash.is_empty());
                assert_eq!(result.block.header.data_hash, empty_merkle_root_hash);
                assert_eq!(result.block.header.evidence_hash, empty_merkle_root_hash);
                assert_eq!(result.block.header.height.value(), 10);
                assert!(result.block.header.last_block_id.is_some());
                assert!(result.block.header.last_commit_hash.is_some());
                assert!(result.block.header.last_results_hash.is_some());
                assert!(!result.block.header.next_validators_hash.is_empty());
                assert_ne!(
                    result.block.header.proposer_address.as_bytes(),
                    [0u8; cometbft::account::LENGTH]
                );
                assert!(
                    result
                        .block
                        .header
                        .time
                        .duration_since(informal_epoch)
                        .unwrap()
                        .as_secs()
                        > 0
                );
                assert!(!result.block.header.validators_hash.is_empty());
                assert_eq!(
                    result.block.header.version,
                    cometbft::block::header::Version { block: 11, app: 1 }
                );
                let last_commit = result.block.last_commit.unwrap();
                assert!(!last_commit.block_id.hash.is_empty());
                assert!(!last_commit.block_id.part_set_header.hash.is_empty());
                assert_eq!(last_commit.block_id.part_set_header.total, 1);
                assert_eq!(last_commit.height.value(), 9);
                assert_eq!(last_commit.round.value(), 0);
                assert_eq!(last_commit.signatures.len(), 1);
                assert!(last_commit.signatures[0].is_commit());
                assert!(last_commit.signatures[0].validator_address().is_some());
                // It's weird but there is no implementation to get the signature out of CommitSig.
                assert!(!result.block_id.hash.is_empty());
                assert!(!result.block_id.part_set_header.hash.is_empty());
                assert_eq!(result.block_id.part_set_header.total, 1);
            },
            "block_results_at_height_10" => {
                let result = endpoint::block_results::Response::from_string(content).unwrap();
                assert!(result.begin_block_events.is_none());
                assert!(result.consensus_param_updates.is_none());
                assert!(result.end_block_events.is_none());
                assert_eq!(result.height.value(), 10);
                assert!(result.txs_results.is_none());
                assert!(result.validator_updates.is_empty());
            },
            "block_by_hash" => {
                let result = endpoint::block_by_hash::Response::from_string(content).unwrap();
                assert_eq!(
                    result.block_id.hash.to_string(),
                    "FCF9C2537FC3534CA71001FE1F14C4F769090948C1A521682F612E7CF73AE639"
                );
            },
            "block_search" => {
                let result = endpoint::block_search::Response::from_string(content).unwrap();
                assert_eq!(result.total_count as usize, result.blocks.len());
                for response in result.blocks {
                    assert!(response.block.header.height.value() > 1);
                }
            },
            "block_search_evidence" => {
                let result = endpoint::block_search::Response::from_string(content).unwrap();
                assert_eq!(result.total_count as usize, result.blocks.len());

                // Test a few selected attributes of the results.
                for block in result.blocks {
                    let evidence = block.block.evidence.iter().next().unwrap();

                    use cometbft::vote;

                    fn check_vote(vote: &Vote) {
                        assert_eq!(vote.vote_type, vote::Type::Precommit);
                        assert_eq!(vote.height.value(), 8009);
                        assert_eq!(vote.round.value(), 0);
                        assert_eq!(
                            vote.validator_address,
                            "9319035301DA526CC78DCF174A47A74F81401291".parse().unwrap(),
                        );
                        assert_eq!(vote.validator_index.value(), 8);
                    }

                    if let Evidence::DuplicateVote(dup) = evidence {
                        assert_eq!(dup.total_voting_power.value(), 121);
                        assert_eq!(dup.validator_power.value(), 1);
                        assert_eq!(
                            dup.timestamp,
                            "2022-09-12T19:49:49.984608464Z".parse().unwrap()
                        );

                        check_vote(&dup.vote_a);
                        check_vote(&dup.vote_b);
                    } else {
                        panic!("not a duplicate vote: {evidence:?}");
                    }
                }
            },
            "broadcast_tx_async" => {
                let result = endpoint::broadcast::tx_async::Response::from_string(content).unwrap();
                assert_eq!(result.code, abci::Code::Ok);
                assert!(result.data.is_empty());
                assert_ne!(
                    result.hash,
                    Hash::from_bytes(Algorithm::Sha256, &[0; 32]).unwrap()
                );
                assert!(result.log.is_empty());
            },
            "broadcast_tx_commit" => {
                let result =
                    endpoint::broadcast::tx_commit::Response::from_string(content).unwrap();
                assert_eq!(result.check_tx.code, abci::Code::Ok);
                assert!(result.check_tx.codespace.is_empty());
                assert!(result.check_tx.data.is_empty());
                assert!(result.check_tx.events.is_empty());
                assert_eq!(result.check_tx.gas_used, 0);
                // Todo: https://github.com/informalsystems/tendermint-rs/issues/761
                // assert_eq!(result.check_tx.gas_wanted.value(), 1);
                assert!(result.check_tx.info.to_string().is_empty());
                assert!(result.check_tx.log.is_empty());
                assert_eq!(result.tx_result.code, abci::Code::Ok);
                assert!(result.tx_result.codespace.is_empty());
                assert!(result.tx_result.data.is_empty());
                assert_eq!(result.tx_result.events.len(), 2);
                assert_eq!(result.tx_result.events[0].attributes.len(), 4);
                assert_eq!(result.tx_result.events[0].attributes[0].key, "creator");
                assert_eq!(
                    result.tx_result.events[0].attributes[0].value,
                    "Cosmoshi Netowoko"
                );
                assert_eq!(result.tx_result.events[0].attributes[1].key, "key");
                assert_eq!(result.tx_result.events[0].attributes[1].value, "commit-key");
                assert_eq!(result.tx_result.events[0].attributes[2].key, "index_key");
                assert_eq!(
                    result.tx_result.events[0].attributes[2].value,
                    "index is working"
                );
                assert_eq!(result.tx_result.events[0].attributes[3].key, "noindex_key");
                assert_eq!(
                    result.tx_result.events[0].attributes[3].value,
                    "index is working"
                );
                assert_eq!(result.tx_result.events[0].kind, "app");
                assert_eq!(result.tx_result.gas_used, 0);
                assert_eq!(result.tx_result.gas_wanted, 0);
                assert!(result.tx_result.info.to_string().is_empty());
                assert!(result.tx_result.log.is_empty());
                assert_ne!(
                    result.hash,
                    Hash::from_bytes(Algorithm::Sha256, &[0; 32]).unwrap()
                );
            },
            "broadcast_tx_sync" => {
                let result = endpoint::broadcast::tx_sync::Response::from_string(content).unwrap();
                assert_eq!(result.code, abci::Code::Ok);
                assert!(result.data.is_empty());
                assert_ne!(
                    result.hash,
                    Hash::from_bytes(Algorithm::Sha256, &[0; 32]).unwrap()
                );
                assert!(result.log.is_empty());
            },
            "commit_at_height_10" => {
                let result = endpoint::commit::Response::from_string(content).unwrap();
                assert!(!result.signed_header.commit.block_id.hash.is_empty());
                assert_eq!(result.signed_header.commit.height.value(), 10);
                assert_eq!(result.signed_header.commit.round.value(), 0);
                assert_eq!(result.signed_header.commit.signatures.len(), 1);
                assert!(result.signed_header.commit.signatures[0].is_commit());
                assert!(result.signed_header.commit.signatures[0]
                    .validator_address()
                    .is_some());
                assert_eq!(result.signed_header.header.app_hash.as_bytes(), [0u8; 8]);
                assert_eq!(result.signed_header.header.chain_id.as_str(), CHAIN_ID);
                assert!(!result.signed_header.header.consensus_hash.is_empty());
                assert_eq!(
                    result.signed_header.header.data_hash,
                    empty_merkle_root_hash
                );
                assert_eq!(
                    result.signed_header.header.evidence_hash,
                    empty_merkle_root_hash
                );
                assert_eq!(result.signed_header.header.height.value(), 10);
                assert!(result.signed_header.header.last_block_id.is_some());
                assert!(result.signed_header.header.last_commit_hash.is_some());
                assert!(result.signed_header.header.last_results_hash.is_some());
                assert!(!result.signed_header.header.next_validators_hash.is_empty());
                assert_ne!(
                    result.signed_header.header.proposer_address.as_bytes(),
                    [0u8; cometbft::account::LENGTH]
                );
                assert!(
                    result
                        .signed_header
                        .header
                        .time
                        .duration_since(informal_epoch)
                        .unwrap()
                        .as_secs()
                        > 0
                );
                assert!(!result.signed_header.header.validators_hash.is_empty());
                assert_eq!(
                    result.signed_header.header.version,
                    cometbft::block::header::Version { block: 11, app: 1 }
                );
            },
            "consensus_params" => {
                let result = endpoint::consensus_params::Response::from_string(content).unwrap();
                assert_eq!(u64::from(result.block_height), 10_u64);
                assert_eq!(result.consensus_params.block.max_bytes, 22020096_u64);
                assert_eq!(result.consensus_params.block.max_gas, -1_i64);
                assert_eq!(result.consensus_params.block.time_iota_ms, 1000_i64);
                assert_eq!(
                    result.consensus_params.evidence.max_age_duration,
                    Duration(core::time::Duration::from_nanos(172800000000000_u64))
                );
                assert_eq!(
                    result.consensus_params.evidence.max_age_num_blocks,
                    100000_u64
                );
                assert_eq!(result.consensus_params.evidence.max_bytes, 1048576_i64);
                assert_eq!(
                    result.consensus_params.validator.pub_key_types,
                    vec![public_key::Algorithm::Ed25519]
                );
            },
            "consensus_state" => {
                assert!(endpoint::consensus_state::Response::from_string(content).is_ok());
            },
            "genesis" => {
                let result =
                    endpoint::genesis::Response::<Option<serde_json::Value>>::from_string(content)
                        .unwrap();
                assert!(result.genesis.app_hash.as_bytes().is_empty());
                assert_eq!(result.genesis.chain_id.as_str(), CHAIN_ID);
                assert_eq!(result.genesis.consensus_params.block.max_bytes, 22020096);
                assert_eq!(result.genesis.consensus_params.block.max_gas, -1);
                assert_eq!(
                    result
                        .genesis
                        .consensus_params
                        .evidence
                        .max_age_duration
                        .0
                        .as_nanos(),
                    172800000000000
                );
                assert_eq!(
                    result.genesis.consensus_params.evidence.max_age_num_blocks,
                    100000
                );
                assert_eq!(result.genesis.consensus_params.evidence.max_bytes, 1048576);
                assert_eq!(
                    result
                        .genesis
                        .consensus_params
                        .validator
                        .pub_key_types
                        .len(),
                    1
                );
                assert_eq!(
                    result.genesis.consensus_params.validator.pub_key_types[0],
                    cometbft::public_key::Algorithm::Ed25519
                );
                assert!(result.genesis.consensus_params.version.is_none());
                assert!(
                    result
                        .genesis
                        .genesis_time
                        .duration_since(informal_epoch)
                        .unwrap()
                        .as_secs()
                        > 0
                );
                assert_eq!(result.genesis.validators.len(), 1);
                assert_ne!(
                    result.genesis.validators[0].address.as_bytes(),
                    [0; cometbft::account::LENGTH]
                );
                assert_eq!(result.genesis.validators[0].power(), 10);
                assert!(result.genesis.validators[0].pub_key.ed25519().is_some());
                assert_eq!(result.genesis.validators[0].proposer_priority.value(), 0);
                assert_eq!(result.genesis.consensus_params.block.time_iota_ms, 1000);
            },
            "net_info" => {
                let result = endpoint::net_info::Response::from_string(content).unwrap();
                assert_eq!(result.listeners.len(), 1);
                assert_eq!(result.listeners[0].to_string(), "Listener(@)");
                assert!(result.listening);
                assert_eq!(result.n_peers, 0);
                assert!(result.peers.is_empty());
            },
            "status" => {
                let result = endpoint::status::Response::from_string(content).unwrap();
                assert_eq!(
                    Address::from_listen_address(&result.node_info.listen_addr).unwrap(),
                    Address::from_str("tcp://0.0.0.0:26656").unwrap()
                );
                assert_eq!(result.node_info.moniker.to_string(), "dockernode");
                assert_eq!(result.node_info.network.to_string(), CHAIN_ID);
                assert_eq!(
                    result.node_info.other.rpc_address,
                    format!("{}", Address::from_str("tcp://0.0.0.0:26657").unwrap())
                );
                assert_eq!(
                    result.node_info.other.tx_index,
                    cometbft::node::info::TxIndexStatus::On
                );
                assert_eq!(
                    result.node_info.protocol_version,
                    cometbft::node::info::ProtocolVersionInfo {
                        p2p: 8,
                        block: 11,
                        app: 1
                    }
                );
                assert_eq!(result.node_info.version.to_string(), "0.37.0-alpha.3");
                assert!(!result.sync_info.catching_up);
                assert!(result.sync_info.earliest_app_hash.as_bytes().is_empty());
                assert!(!result.sync_info.earliest_block_hash.is_empty());
                assert_eq!(result.sync_info.earliest_block_height.value(), 1);
                assert!(
                    result
                        .sync_info
                        .earliest_block_time
                        .duration_since(informal_epoch)
                        .unwrap()
                        .as_secs()
                        > 0
                );
                assert_eq!(
                    result.sync_info.latest_app_hash.as_bytes(),
                    [6, 0, 0, 0, 0, 0, 0, 0]
                );
                assert!(!result.sync_info.latest_block_hash.is_empty());
                assert_eq!(result.sync_info.latest_block_height.value(), 53);
                assert!(
                    result
                        .sync_info
                        .latest_block_time
                        .duration_since(informal_epoch)
                        .unwrap()
                        .as_secs()
                        > 0
                );
                assert!(result.validator_info.pub_key.ed25519().is_some());
                assert_eq!(result.validator_info.power.value(), 10);
            },
            "blockchain_from_1_to_10" => {
                let result = endpoint::blockchain::Response::from_string(content).unwrap();
                assert_eq!(result.block_metas.len(), 10);
                for block_meta in result.block_metas {
                    assert!(!block_meta.block_id.hash.is_empty());
                    assert!(!block_meta.block_id.part_set_header.hash.is_empty());
                    assert_eq!(block_meta.block_id.part_set_header.total, 1);
                    assert!(block_meta.block_size > 0);
                    if block_meta.header.height.value() == 1 {
                        assert!(block_meta.header.app_hash.as_bytes().is_empty());
                        assert_eq!(block_meta.header.data_hash, empty_merkle_root_hash);
                        assert_eq!(block_meta.header.evidence_hash, empty_merkle_root_hash);
                        assert!(block_meta.header.last_block_id.is_none());
                        assert_eq!(block_meta.header.last_commit_hash, empty_merkle_root_hash);
                        assert_eq!(block_meta.header.last_results_hash, empty_merkle_root_hash);
                    } else {
                        assert!(!block_meta.header.app_hash.as_bytes().is_empty());
                        assert!(block_meta.header.data_hash.is_some());
                        assert!(block_meta.header.evidence_hash.is_some());
                        assert!(block_meta.header.last_block_id.is_some());
                        assert!(block_meta.header.last_commit_hash.is_some());
                        assert!(block_meta.header.last_results_hash.is_some());
                    }
                    assert_eq!(block_meta.header.chain_id.as_str(), CHAIN_ID);
                    assert!(!block_meta.header.consensus_hash.is_empty());
                    assert!(!block_meta.header.next_validators_hash.is_empty());
                    assert_ne!(
                        block_meta.header.proposer_address.as_bytes(),
                        [0u8; cometbft::account::LENGTH]
                    );
                    assert!(
                        block_meta
                            .header
                            .time
                            .duration_since(informal_epoch)
                            .unwrap()
                            .as_secs()
                            > 0
                    );
                    assert!(!block_meta.header.validators_hash.is_empty());
                    assert_eq!(
                        block_meta.header.version,
                        cometbft::block::header::Version { block: 11, app: 1 }
                    );
                    assert_eq!(block_meta.num_txs, 0);
                }
            },
            "subscribe_malformed" => {
                let result = endpoint::subscribe::Response::from_string(content);

                match result {
                    Err(Error(ErrorDetail::Response(e), _)) => {
                        let response = e.source;

                        assert_eq!(response.code(), Code::InternalError);
                        assert_eq!(response.message(), "Internal error");
                        assert_eq!(response.data().unwrap(),"failed to parse query: \nparse error near PegText (line 1 symbol 2 - line 1 symbol 11):\n\"malformed\"\n");
                    },
                    _ => panic!("expected Response error"),
                }
            },
            "subscribe_newblock" => {
                let result = endpoint::subscribe::Response::from_string(content);

                match result {
                    Err(Error(ErrorDetail::Serde(_), _)) => {},
                    _ => panic!("expected Serde parse error, instead got {result:?}"),
                }
            },
            "subscribe_newblock_0" => {
                let result = DeEvent::from_string(content).unwrap();
                if let EventData::LegacyNewBlock {
                    block,
                    result_begin_block,
                    result_end_block,
                } = result.data.into()
                {
                    let b = block.unwrap();
                    assert!(b.data.first().is_none());
                    assert!(b.evidence.iter().next().is_none());
                    assert!(!b.header.app_hash.as_bytes().is_empty());
                    assert_eq!(b.header.chain_id.as_str(), CHAIN_ID);
                    assert!(!b.header.consensus_hash.is_empty());
                    assert_eq!(b.header.data_hash, empty_merkle_root_hash);
                    assert_eq!(b.header.evidence_hash, empty_merkle_root_hash);
                    assert!(b.header.last_block_id.is_some());
                    assert!(b.header.last_commit_hash.is_some());
                    assert!(b.header.last_results_hash.is_some());
                    assert!(!b.header.next_validators_hash.is_empty());
                    assert_ne!(
                        b.header.proposer_address.as_bytes(),
                        [0u8; cometbft::account::LENGTH]
                    );
                    assert!(
                        b.header
                            .time
                            .duration_since(informal_epoch)
                            .unwrap()
                            .as_secs()
                            > 0
                    );
                    assert!(!b.header.validators_hash.is_empty());
                    assert_eq!(
                        b.header.version,
                        cometbft::block::header::Version { block: 11, app: 1 }
                    );
                    let last_commit = b.last_commit.unwrap();
                    assert!(!last_commit.block_id.hash.is_empty());
                    assert!(!last_commit.block_id.part_set_header.hash.is_empty());
                    assert_eq!(last_commit.block_id.part_set_header.total, 1);
                    assert_eq!(last_commit.round.value(), 0);
                    assert_eq!(last_commit.signatures.len(), 1);
                    assert!(last_commit.signatures[0].is_commit());
                    assert!(last_commit.signatures[0].validator_address().is_some());
                    assert!(result_begin_block.unwrap().events.is_empty());
                    let reb = result_end_block.unwrap();
                    assert!(reb.validator_updates.is_empty());
                    assert!(reb.consensus_param_updates.is_none());
                    assert!(reb.events.is_empty());
                } else {
                    panic!("not a LegacyNewBlock event");
                }
                assert_eq!(result.query, "tm.event = 'NewBlock'");
            },
            "subscribe_newblock_1" => {
                let result = DeEvent::from_string(content).unwrap();
                if let EventData::LegacyNewBlock {
                    block,
                    result_begin_block,
                    result_end_block,
                } = result.data.into()
                {
                    let b = block.unwrap();
                    assert!(b.data.first().is_none());
                    assert!(b.evidence.iter().next().is_none());
                    assert!(!b.header.app_hash.as_bytes().is_empty());
                    assert_eq!(b.header.chain_id.as_str(), CHAIN_ID);
                    assert!(!b.header.consensus_hash.is_empty());
                    assert_eq!(b.header.data_hash, empty_merkle_root_hash);
                    assert_eq!(b.header.evidence_hash, empty_merkle_root_hash);
                    assert!(b.header.last_block_id.is_some());
                    assert!(b.header.last_commit_hash.is_some());
                    assert!(b.header.last_results_hash.is_some());
                    assert!(!b.header.next_validators_hash.is_empty());
                    assert_ne!(
                        b.header.proposer_address.as_bytes(),
                        [0u8; cometbft::account::LENGTH]
                    );
                    assert!(
                        b.header
                            .time
                            .duration_since(informal_epoch)
                            .unwrap()
                            .as_secs()
                            > 0
                    );
                    assert!(!b.header.validators_hash.is_empty());
                    assert_eq!(
                        b.header.version,
                        cometbft::block::header::Version { block: 11, app: 1 }
                    );
                    let last_commit = b.last_commit.unwrap();
                    assert!(!last_commit.block_id.hash.is_empty());
                    assert!(!last_commit.block_id.part_set_header.hash.is_empty());
                    assert_eq!(last_commit.block_id.part_set_header.total, 1);
                    assert_eq!(last_commit.round.value(), 0);
                    assert_eq!(last_commit.signatures.len(), 1);
                    assert!(last_commit.signatures[0].is_commit());
                    assert!(last_commit.signatures[0].validator_address().is_some());
                    let rbb = result_begin_block.unwrap();
                    assert_eq!(rbb.events.len(), 2);
                    assert_eq!(rbb.events[0].kind, "transfer");
                    assert_eq!(rbb.events[0].attributes.len(), 2);
                    assert_eq!(rbb.events[0].attributes[0].key, "recipient");
                    assert_eq!(
                        rbb.events[0].attributes[0].value,
                        "cosmos17xpfvakm2amg962yls6f84z3kell8c5lserqta"
                    );
                    assert!(rbb.events[0].attributes[0].index);
                    assert_eq!(rbb.events[0].attributes[1].key, "sender");
                    assert_eq!(
                        rbb.events[0].attributes[1].value,
                        "cosmos1m3h30wlvsf8llruxtpukdvsy0km2kum8g38c8q"
                    );
                    assert!(!rbb.events[0].attributes[1].index);
                    assert_eq!(rbb.events[1].kind, "message");
                    assert_eq!(rbb.events[1].attributes.len(), 1);
                    assert_eq!(rbb.events[1].attributes[0].key, "sender");
                    assert_eq!(
                        rbb.events[1].attributes[0].value,
                        "cosmos1m3h30wlvsf8llruxtpukdvsy0km2kum8g38c8q"
                    );
                    let reb = result_end_block.unwrap();
                    assert!(reb.validator_updates.is_empty());
                    assert!(reb.consensus_param_updates.is_none());
                    assert!(reb.events.is_empty());
                } else {
                    panic!("not a LegacyNewBlock event");
                }
                assert_eq!(result.query, "tm.event = 'NewBlock'");
            },
            "subscribe_newblock_2" => {
                let result = DeEvent::from_string(content).unwrap();
                if let EventData::LegacyNewBlock {
                    block,
                    result_begin_block,
                    result_end_block,
                } = result.data.into()
                {
                    let b = block.unwrap();
                    assert!(b.data.first().is_none());
                    assert!(b.evidence.iter().next().is_none());
                    assert!(!b.header.app_hash.as_bytes().is_empty());
                    assert_eq!(b.header.chain_id.as_str(), CHAIN_ID);
                    assert!(!b.header.consensus_hash.is_empty());
                    assert_eq!(b.header.data_hash, empty_merkle_root_hash);
                    assert_eq!(b.header.evidence_hash, empty_merkle_root_hash);
                    assert!(b.header.last_block_id.is_some());
                    assert!(b.header.last_commit_hash.is_some());
                    assert!(b.header.last_results_hash.is_some());
                    assert!(!b.header.next_validators_hash.is_empty());
                    assert_ne!(
                        b.header.proposer_address.as_bytes(),
                        [0u8; cometbft::account::LENGTH]
                    );
                    assert!(
                        b.header
                            .time
                            .duration_since(informal_epoch)
                            .unwrap()
                            .as_secs()
                            > 0
                    );
                    assert!(!b.header.validators_hash.is_empty());
                    assert_eq!(
                        b.header.version,
                        cometbft::block::header::Version { block: 11, app: 1 }
                    );
                    let last_commit = b.last_commit.unwrap();
                    assert!(!last_commit.block_id.hash.is_empty());
                    assert!(!last_commit.block_id.part_set_header.hash.is_empty());
                    assert_eq!(last_commit.block_id.part_set_header.total, 1);
                    assert_eq!(last_commit.round.value(), 0);
                    assert_eq!(last_commit.signatures.len(), 1);
                    assert!(last_commit.signatures[0].is_commit());
                    assert!(last_commit.signatures[0].validator_address().is_some());
                    assert!(result_begin_block.unwrap().events.is_empty());
                    let reb = result_end_block.unwrap();
                    assert!(reb.validator_updates.is_empty());
                    assert!(reb.consensus_param_updates.is_none());
                    assert!(reb.events.is_empty());
                } else {
                    panic!("not a LegacyNewBlock event");
                }
                assert_eq!(result.query, "tm.event = 'NewBlock'");
            },
            "subscribe_newblock_3" => {
                let result = DeEvent::from_string(content).unwrap();
                if let EventData::LegacyNewBlock {
                    block,
                    result_begin_block,
                    result_end_block,
                } = result.data.into()
                {
                    let b = block.unwrap();
                    assert!(b.data.first().is_none());
                    assert!(b.evidence.iter().next().is_none());
                    assert!(!b.header.app_hash.as_bytes().is_empty());
                    assert_eq!(b.header.chain_id.as_str(), CHAIN_ID);
                    assert!(!b.header.consensus_hash.is_empty());
                    assert_eq!(b.header.data_hash, empty_merkle_root_hash);
                    assert_eq!(b.header.evidence_hash, empty_merkle_root_hash);
                    assert!(b.header.last_block_id.is_some());
                    assert!(b.header.last_commit_hash.is_some());
                    assert!(b.header.last_results_hash.is_some());
                    assert!(!b.header.next_validators_hash.is_empty());
                    assert_ne!(
                        b.header.proposer_address.as_bytes(),
                        [0u8; cometbft::account::LENGTH]
                    );
                    assert!(
                        b.header
                            .time
                            .duration_since(informal_epoch)
                            .unwrap()
                            .as_secs()
                            > 0
                    );
                    assert!(!b.header.validators_hash.is_empty());
                    assert_eq!(
                        b.header.version,
                        cometbft::block::header::Version { block: 11, app: 1 }
                    );
                    let last_commit = b.last_commit.unwrap();
                    assert!(!last_commit.block_id.hash.is_empty());
                    assert!(!last_commit.block_id.part_set_header.hash.is_empty());
                    assert_eq!(last_commit.block_id.part_set_header.total, 1);
                    assert_eq!(last_commit.round.value(), 0);
                    assert_eq!(last_commit.signatures.len(), 1);
                    assert!(last_commit.signatures[0].is_commit());
                    assert!(last_commit.signatures[0].validator_address().is_some());
                    assert!(result_begin_block.unwrap().events.is_empty());
                    let reb = result_end_block.unwrap();
                    assert!(reb.validator_updates.is_empty());
                    assert!(reb.consensus_param_updates.is_none());
                    assert!(reb.events.is_empty());
                } else {
                    panic!("not a LegacyNewBlock event");
                }
                assert_eq!(result.query, "tm.event = 'NewBlock'");
            },
            "subscribe_newblock_4" => {
                let result = DeEvent::from_string(content).unwrap();
                if let EventData::LegacyNewBlock {
                    block,
                    result_begin_block,
                    result_end_block,
                } = result.data.into()
                {
                    let b = block.unwrap();
                    assert!(b.data.first().is_none());
                    assert!(b.evidence.iter().next().is_none());
                    assert!(!b.header.app_hash.as_bytes().is_empty());
                    assert_eq!(b.header.chain_id.as_str(), CHAIN_ID);
                    assert!(!b.header.consensus_hash.is_empty());
                    assert_eq!(b.header.data_hash, empty_merkle_root_hash);
                    assert_eq!(b.header.evidence_hash, empty_merkle_root_hash);
                    assert!(b.header.last_block_id.is_some());
                    assert!(b.header.last_commit_hash.is_some());
                    assert!(b.header.last_results_hash.is_some());
                    assert!(!b.header.next_validators_hash.is_empty());
                    assert_ne!(
                        b.header.proposer_address.as_bytes(),
                        [0u8; cometbft::account::LENGTH]
                    );
                    assert!(
                        b.header
                            .time
                            .duration_since(informal_epoch)
                            .unwrap()
                            .as_secs()
                            > 0
                    );
                    assert!(!b.header.validators_hash.is_empty());
                    assert_eq!(
                        b.header.version,
                        cometbft::block::header::Version { block: 11, app: 1 }
                    );
                    let last_commit = b.last_commit.unwrap();
                    assert!(!last_commit.block_id.hash.is_empty());
                    assert!(!last_commit.block_id.part_set_header.hash.is_empty());
                    assert_eq!(last_commit.block_id.part_set_header.total, 1);
                    assert_eq!(last_commit.round.value(), 0);
                    assert_eq!(last_commit.signatures.len(), 1);
                    assert!(last_commit.signatures[0].is_commit());
                    assert!(last_commit.signatures[0].validator_address().is_some());
                    assert!(result_begin_block.unwrap().events.is_empty());
                    let reb = result_end_block.unwrap();
                    assert!(reb.validator_updates.is_empty());
                    assert!(reb.consensus_param_updates.is_none());
                    assert!(reb.events.is_empty());
                } else {
                    panic!("not a LegacyNewBlock event");
                }
                assert_eq!(result.query, "tm.event = 'NewBlock'");
            },
            "subscribe_txs" => {
                assert!(endpoint::subscribe::Response::from_string(content).is_ok());
            },
            "subscribe_txs_0" => {
                let result = DeEvent::from_string(content).unwrap();
                let height;
                if let cometbft_rpc::event::EventData::Tx { tx_result } = result.data.into() {
                    height = tx_result.height;
                    assert!(tx_result.result.log.is_none());
                    assert!(tx_result.result.gas_wanted.is_none());
                    assert!(tx_result.result.gas_used.is_none());
                    assert_eq!(tx_result.result.events.len(), 2);
                    assert_eq!(tx_result.result.events[0].kind, "app");
                    for attr in &tx_result.result.events[0].attributes {
                        match attr.key.as_str() {
                            "creator" => {
                                assert_eq!(attr.value, "Cosmoshi Netowoko")
                            },
                            "key" => assert_eq!(attr.value, "tx0"),
                            "index_key" => {
                                assert_eq!(attr.value, "index is working")
                            },
                            "noindex_key" => {
                                assert_eq!(attr.value, "index is working")
                            },
                            _ => panic!("unknown attribute found {}", attr.key),
                        }
                    }
                    assert_eq!(tx_result.tx, base64::decode("dHgwPXZhbHVl").unwrap());
                } else {
                    panic!("not a tx");
                }
                check_event_attrs(&result.events.unwrap(), "tx0", height);
                assert_eq!(result.query, "tm.event = 'Tx'");
            },
            "subscribe_txs_1" => {
                let result = DeEvent::from_string(content).unwrap();
                let height;
                if let cometbft_rpc::event::EventData::Tx { tx_result } = result.data.into() {
                    height = tx_result.height;
                    assert!(tx_result.result.log.is_none());
                    assert!(tx_result.result.gas_wanted.is_none());
                    assert!(tx_result.result.gas_used.is_none());
                    assert_eq!(tx_result.result.events.len(), 2);
                    assert_eq!(tx_result.result.events[0].kind, "app");
                    for attr in &tx_result.result.events[0].attributes {
                        match attr.key.as_str() {
                            "creator" => {
                                assert_eq!(attr.value, "Cosmoshi Netowoko")
                            },
                            "key" => assert_eq!(attr.value, "tx1"),
                            "index_key" => {
                                assert_eq!(attr.value, "index is working")
                            },
                            "noindex_key" => {
                                assert_eq!(attr.value, "index is working")
                            },
                            _ => panic!("unknown attribute found {}", attr.key),
                        }
                    }
                    assert_eq!(tx_result.tx, base64::decode("dHgxPXZhbHVl").unwrap());
                } else {
                    panic!("not a tx");
                }

                check_event_attrs(&result.events.unwrap(), "tx1", height);
                assert_eq!(result.query, "tm.event = 'Tx'");
            },
            "subscribe_txs_2" => {
                let result = DeEvent::from_string(content).unwrap();
                let height;
                if let cometbft_rpc::event::EventData::Tx { tx_result } = result.data.into() {
                    height = tx_result.height;
                    assert!(tx_result.result.log.is_none());
                    assert!(tx_result.result.gas_wanted.is_none());
                    assert!(tx_result.result.gas_used.is_none());
                    assert_eq!(tx_result.result.events.len(), 2);
                    assert_eq!(tx_result.result.events[0].kind, "app");
                    for attr in &tx_result.result.events[0].attributes {
                        match attr.key.as_str() {
                            "creator" => {
                                assert_eq!(attr.value, "Cosmoshi Netowoko")
                            },
                            "key" => assert_eq!(attr.value, "tx2"),
                            "index_key" => {
                                assert_eq!(attr.value, "index is working")
                            },
                            "noindex_key" => {
                                assert_eq!(attr.value, "index is working")
                            },
                            _ => panic!("unknown attribute found {}", attr.key),
                        }
                    }
                    assert_eq!(tx_result.tx, base64::decode("dHgyPXZhbHVl").unwrap());
                } else {
                    panic!("not a tx");
                }
                check_event_attrs(&result.events.unwrap(), "tx2", height);
                assert_eq!(result.query, "tm.event = 'Tx'");
            },
            "subscribe_txs_3" => {
                let result = DeEvent::from_string(content).unwrap();
                let height;
                if let cometbft_rpc::event::EventData::Tx { tx_result } = result.data.into() {
                    height = tx_result.height;
                    assert!(tx_result.result.log.is_none());
                    assert!(tx_result.result.gas_wanted.is_none());
                    assert!(tx_result.result.gas_used.is_none());
                    assert_eq!(tx_result.result.events.len(), 2);
                    assert_eq!(tx_result.result.events[0].kind, "app");
                    for attr in &tx_result.result.events[0].attributes {
                        match attr.key.as_str() {
                            "creator" => {
                                assert_eq!(attr.value, "Cosmoshi Netowoko")
                            },
                            "key" => assert_eq!(attr.value, "tx3"),
                            "index_key" => {
                                assert_eq!(attr.value, "index is working")
                            },
                            "noindex_key" => {
                                assert_eq!(attr.value, "index is working")
                            },
                            _ => panic!("unknown attribute found {}", attr.key),
                        }
                    }
                    assert_eq!(tx_result.tx, base64::decode("dHgzPXZhbHVl").unwrap());
                } else {
                    panic!("not a tx");
                }
                check_event_attrs(&result.events.unwrap(), "tx3", height);
                assert_eq!(result.query, "tm.event = 'Tx'");
            },
            "subscribe_txs_4" => {
                let result = DeEvent::from_string(content).unwrap();
                let height;
                if let cometbft_rpc::event::EventData::Tx { tx_result } = result.data.into() {
                    height = tx_result.height;
                    assert!(tx_result.result.log.is_none());
                    assert!(tx_result.result.gas_wanted.is_none());
                    assert!(tx_result.result.gas_used.is_none());
                    assert_eq!(tx_result.result.events.len(), 2);
                    assert_eq!(tx_result.result.events[0].kind, "app");
                    for attr in &tx_result.result.events[0].attributes {
                        match attr.key.as_str() {
                            "creator" => {
                                assert_eq!(attr.value, "Cosmoshi Netowoko")
                            },
                            "key" => assert_eq!(attr.value, "tx4"),
                            "index_key" => {
                                assert_eq!(attr.value, "index is working")
                            },
                            "noindex_key" => {
                                assert_eq!(attr.value, "index is working")
                            },
                            _ => panic!("unknown attribute found {}", attr.key),
                        }
                    }
                    assert_eq!(tx_result.tx, base64::decode("dHg0PXZhbHVl").unwrap());
                } else {
                    panic!("not a tx");
                }
                check_event_attrs(&result.events.unwrap(), "tx4", height);
                assert_eq!(result.query, "tm.event = 'Tx'");
            },
            "subscribe_txs_broadcast_tx_0" => {
                let result = endpoint::broadcast::tx_async::Response::from_string(content).unwrap();
                assert_eq!(result.code, abci::Code::Ok);
                assert!(result.data.is_empty());
                assert_ne!(
                    result.hash,
                    Hash::from_bytes(Algorithm::Sha256, &[0; 32]).unwrap()
                );
                assert!(result.log.is_empty());
            },
            "subscribe_txs_broadcast_tx_1" => {
                let result = endpoint::broadcast::tx_async::Response::from_string(content).unwrap();
                assert_eq!(result.code, abci::Code::Ok);
                assert!(result.data.is_empty());
                assert_ne!(
                    result.hash,
                    Hash::from_bytes(Algorithm::Sha256, &[0; 32]).unwrap()
                );
                assert!(result.log.is_empty());
            },
            "subscribe_txs_broadcast_tx_2" => {
                let result = endpoint::broadcast::tx_async::Response::from_string(content).unwrap();
                assert_eq!(result.code, abci::Code::Ok);
                assert!(result.data.is_empty());
                assert_ne!(
                    result.hash,
                    Hash::from_bytes(Algorithm::Sha256, &[0; 32]).unwrap()
                );
                assert!(result.log.is_empty());
            },
            "subscribe_txs_broadcast_tx_3" => {
                let result = endpoint::broadcast::tx_async::Response::from_string(content).unwrap();
                assert_eq!(result.code, abci::Code::Ok);
                assert!(result.data.is_empty());
                assert_ne!(
                    result.hash,
                    Hash::from_bytes(Algorithm::Sha256, &[0; 32]).unwrap()
                );
                assert!(result.log.is_empty());
            },
            "subscribe_txs_broadcast_tx_4" => {
                let result = endpoint::broadcast::tx_async::Response::from_string(content).unwrap();
                assert_eq!(result.code, abci::Code::Ok);
                assert!(result.data.is_empty());
                assert_ne!(
                    result.hash,
                    Hash::from_bytes(Algorithm::Sha256, &[0; 32]).unwrap()
                );
                assert!(result.log.is_empty());
            },
            "subscribe_txs_broadcast_tx_5" => {
                let result = endpoint::broadcast::tx_async::Response::from_string(content).unwrap();
                assert_eq!(result.code, abci::Code::Ok);
                assert!(result.data.is_empty());
                assert_ne!(
                    result.hash,
                    Hash::from_bytes(Algorithm::Sha256, &[0; 32]).unwrap()
                );
                assert!(result.log.is_empty());
            },
            "tx" => {
                let result = endpoint::tx::Response::from_string(content).unwrap();
                assert_eq!(
                    result.hash,
                    Hash::from_bytes(
                        Algorithm::Sha256,
                        &[
                            214, 63, 156, 35, 121, 30, 97, 4, 16, 181, 118, 216, 194, 123, 181,
                            174, 172, 147, 204, 26, 88, 82, 36, 40, 167, 179, 42, 18, 118, 8, 88,
                            96
                        ]
                    )
                    .unwrap()
                );
                assert_eq!(u64::from(result.height), 12u64);
            },
            "tx_search_no_prove" => {
                let result = endpoint::tx_search::Response::from_string(content).unwrap();
                assert_eq!(result.total_count as usize, result.txs.len());
                // Test a few selected attributes of the results.
                for tx in result.txs {
                    assert_ne!(tx.hash.as_bytes(), [0; 32]);
                    assert_eq!(tx.tx_result.code, abci::Code::Ok);
                    assert_eq!(tx.tx_result.events.len(), 2);
                    assert_eq!(tx.tx_result.events[0].kind, "app");
                    assert_eq!(tx.tx_result.gas_used, 0);
                    assert_eq!(tx.tx_result.gas_wanted, 0);
                    assert!(tx.tx_result.info.to_string().is_empty());
                    assert!(tx.tx_result.log.is_empty());
                    assert!(tx.proof.is_none());
                }
            },
            "tx_search_with_prove" => {
                let result = endpoint::tx_search::Response::from_string(content).unwrap();
                assert_eq!(result.total_count as usize, result.txs.len());
                // Test a few selected attributes of the results.
                for tx in result.txs {
                    assert_ne!(tx.hash.as_bytes(), [0; 32]);
                    assert_eq!(tx.tx_result.code, abci::Code::Ok);
                    assert_eq!(tx.tx_result.events.len(), 2);
                    assert_eq!(tx.tx_result.events[0].kind, "app");
                    assert_eq!(tx.tx_result.gas_used, 0);
                    assert_eq!(tx.tx_result.gas_wanted, 0);
                    assert!(tx.tx_result.info.to_string().is_empty());
                    assert!(tx.tx_result.log.is_empty());
                    let proof = tx.proof.unwrap();
                    assert_eq!(proof.data, tx.tx);
                    assert_eq!(proof.proof.total, 1);
                    assert_eq!(proof.proof.index, 0);
                    assert_ne!(proof.root_hash.as_bytes(), [0; 32]);
                }
            },
            _ => {
                panic!("cannot parse file name: {file_name}");
            },
        }
    }
}

fn check_event_attrs(events: &HashMap<String, Vec<String>>, app_key: &str, height: i64) {
    for (k, v) in events {
        match k.as_str() {
            "app.creator" => {
                assert_eq!(v.len(), 2);
                assert_eq!(v[0], "Cosmoshi Netowoko");
            },
            "app.index_key" => {
                assert_eq!(v.len(), 2);
                assert_eq!(v[0], "index is working");
            },
            "app.key" => {
                assert_eq!(v.len(), 2);
                assert_eq!(v[0], app_key);
            },
            "app.noindex_key" => {
                assert_eq!(v.len(), 2);
                assert_eq!(v[0], "index is working");
            },
            "tm.event" => {
                assert_eq!(v.len(), 1);
                assert_eq!(v[0], "Tx");
            },
            "tx.hash" => {
                assert_eq!(v.len(), 1);
                assert_eq!(v[0].len(), 64);
            },
            "tx.height" => {
                assert_eq!(v.len(), 1);
                assert_eq!(v[0], height.to_string());
            },
            _ => panic!("unknown event found {k}"),
        }
    }
}
