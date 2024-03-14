use std::{fs, path::PathBuf};

use cometbft_rpc::{endpoint, Response};

use walkdir::WalkDir;

fn find_fixtures(in_out_folder_name: &str) -> Vec<PathBuf> {
    WalkDir::new(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("sei_fixtures")
            .join(in_out_folder_name),
    )
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| {
        e.file_type().is_file()
            && e.path().extension().is_some()
            && e.path().extension().unwrap() == "json"
    })
    .map(|e| e.into_path())
    .collect::<Vec<PathBuf>>()
}

#[test]
fn incoming_fixtures() {
    for json_file in find_fixtures("incoming") {
        let file_name = json_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".json")
            .unwrap();

        let content = fs::read_to_string(&json_file).unwrap();

        match file_name {
            "block_results_with_failed_txs" => {
                let r = endpoint::block_results::Response::from_string(&content);
                dbg!(&r);
                assert!(r.is_ok(), "block_results_with_failed_txs (v0.37+): {r:?}");

                let r = endpoint::block_results::v0_34::DialectResponse::from_string(&content);
                dbg!(&r);
                assert!(r.is_ok(), "block_results_with_failed_txs (v0.34): {r:?}");
            },
            _ => {
                panic!("unhandled incoming fixture: {file_name}");
            },
        }
    }
}
