use std::{env::var, fs, path::PathBuf, process};

use tempfile::tempdir;

mod buf_build;
use crate::buf_build::{export_dep_module, read_locked_deps};

mod functions;
use functions::{copy_files, find_proto_files, generate_cometbft_mod, get_commitish};

mod constants;
use constants::{
    COMETBFT_COMMITISH, COMETBFT_REPO, CUSTOM_FIELD_ATTRIBUTES, CUSTOM_TYPE_ATTRIBUTES,
};

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target_dir = ["..", "..", "proto", "src"].iter().collect::<PathBuf>();
    let comet_dir = PathBuf::from(var("COMETBFT_DIR").unwrap_or_else(|_| {
        root.join("..")
            .join("target")
            .join("cometbft")
            .to_str()
            .unwrap()
            .to_string()
    }));

    println!(
        "[info] => Fetching {} at {} into {comet_dir:?}",
        COMETBFT_REPO, COMETBFT_COMMITISH,
    );
    get_commitish(&comet_dir, COMETBFT_REPO, COMETBFT_COMMITISH); // This panics if it fails.

    let proto_path = comet_dir.join("proto");

    let mut proto_includes_paths = vec![comet_dir.join("proto")];

    let buf_lock_path = proto_path.join("buf.lock");

    let _temp_dirs = match read_locked_deps(&buf_lock_path) {
        Ok(deps) => deps
            .iter()
            .map(|dep| {
                let mod_dir = tempdir().unwrap();
                if let Err(e) = export_dep_module(dep, mod_dir.path()) {
                    eprintln!(
                        "Failed to export module {}/{}/{}: {}",
                        dep.remote, dep.owner, dep.repository, e,
                    );
                    process::exit(1);
                }
                proto_includes_paths.push(mod_dir.path().to_owned());
                mod_dir
            })
            .collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Failed to read {}: {}", buf_lock_path.display(), e);
            process::exit(1);
        },
    };

    // List available proto files
    let protos = find_proto_files(&proto_path);

    let ver_target_dir = target_dir.join("prost");

    let out_dir = var("OUT_DIR")
        .map(PathBuf::from)
        .or_else(|_| tempdir().map(|d| d.into_path()))
        .unwrap();

    let mut pb = prost_build::Config::new();

    // Use shared Bytes buffers for ABCI messages:
    pb.bytes([".cometbft.abci"]);

    // Compile proto files with added annotations, exchange prost_types to our own
    pb.out_dir(&out_dir);
    for type_attribute in CUSTOM_TYPE_ATTRIBUTES {
        pb.type_attribute(type_attribute.0, type_attribute.1);
    }
    for field_attribute in CUSTOM_FIELD_ATTRIBUTES {
        pb.field_attribute(field_attribute.0, field_attribute.1);
    }
    // The below in-place path redirection replaces references to the Duration
    // and Timestamp WKTs with our own versions that have valid doctest comments.
    // See also https://github.com/danburkert/prost/issues/374 .
    pb.extern_path(
        ".google.protobuf.Duration",
        "crate::google::protobuf::Duration",
    );
    pb.extern_path(
        ".google.protobuf.Timestamp",
        "crate::google::protobuf::Timestamp",
    );

    println!("[info] => Creating structs and interfaces.");
    let builder = tonic_build::configure()
        .out_dir(&out_dir)
        .build_server(true)
        .build_client(false)
        .server_mod_attribute("cometbft.abci.v1beta1", "#[cfg(feature = \"grpc-server\")]")
        .server_mod_attribute("cometbft.abci.v1beta2", "#[cfg(feature = \"grpc-server\")]")
        .server_mod_attribute("cometbft.abci.v1beta3", "#[cfg(feature = \"grpc-server\")]")
        .server_mod_attribute("cometbft.abci.v1", "#[cfg(feature = \"grpc-server\")]")
        .server_mod_attribute(
            "cometbft.rpc.grpc.v1beta1",
            "#[cfg(feature = \"grpc-server\")]",
        )
        .server_mod_attribute(
            "cometbft.rpc.grpc.v1beta2",
            "#[cfg(feature = \"grpc-server\")]",
        )
        .server_mod_attribute(
            "cometbft.rpc.grpc.v1beta3",
            "#[cfg(feature = \"grpc-server\")]",
        )
        .server_mod_attribute(
            "cometbft.services.block.v1",
            "#[cfg(feature = \"grpc-server\")]",
        )
        .server_mod_attribute(
            "cometbft.services.block_results.v1",
            "#[cfg(feature = \"grpc-server\")]",
        )
        .server_mod_attribute(
            "cometbft.services.pruning.v1",
            "#[cfg(feature = \"grpc-server\")]",
        )
        .server_mod_attribute(
            "cometbft.services.version.v1",
            "#[cfg(feature = \"grpc-server\")]",
        );
    // TODO: this is tracked in https://github.com/informalsystems/tendermint-rs/issues/1134
    //.server_mod_attribute("cometbft.privval.v1", "#[cfg(feature = \"grpc-server\")]")

    match builder.compile_with_config(pb, &protos, &proto_includes_paths) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        },
    }

    println!(
        "[info] => Removing old structs and copying new structs to {}",
        ver_target_dir.to_string_lossy(),
    );
    fs::create_dir_all(&target_dir).unwrap();
    copy_files(&out_dir, &ver_target_dir); // This panics if it fails.
    generate_cometbft_mod(&out_dir, &target_dir.join("cometbft.rs")).unwrap();

    println!("[info] => Done!");
}
