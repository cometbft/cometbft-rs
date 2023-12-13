mod client;
mod common;
mod error;
mod gaia;
mod kvstore;
mod plan;
mod request;
mod subscription;
mod utils;

use std::{path::PathBuf, str::FromStr};

use log::LevelFilter;
use simple_logger::SimpleLogger;
use structopt::StructOpt;
use tokio::time::Duration;

// Set default value of `--output` to rpc crate test folder
#[derive(Debug)]
struct OutputPathBuf(pub PathBuf);
impl Default for OutputPathBuf {
    fn default() -> Self {
        Self(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("rpc")
                .join("tests")
                .join("kvstore_fixtures"),
        )
    }
}
impl FromStr for OutputPathBuf {
    type Err = structopt::clap::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(PathBuf::from(s)))
    }
}
impl ToString for OutputPathBuf {
    fn to_string(&self) -> String {
        self.0.to_str().unwrap_or("").to_string()
    }
}

/// A utility application that primarily aims to assist in testing
/// compatibility between cometbft.rs (https://github.com/cometbft/cometbft-rs)
/// and CometBFT (https://github.com/cometbft/cometbft).
///
/// Running this application will execute a "quick probe" against a running
/// CometBFT node. This executes a number of RPC requests against the node,
/// saving both the requests and responses to the desired output folder.
#[derive(Debug, StructOpt)]
struct Opts {
    /// The address of the CometBFT node's WebSocket-based RPC endpoint.
    #[structopt(default_value = "ws://127.0.0.1:26657/websocket", long)]
    pub addr: String,

    /// The output path in which to store the received responses.
    #[structopt(default_value, short, long)]
    pub output: OutputPathBuf,

    /// How long to wait between requests, in milliseconds.
    #[structopt(default_value = "1000", long)]
    pub request_wait: u64,

    /// Increase output logging verbosity.
    #[structopt(short, long)]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Execute a quick probe of a CometBFT node running the `kvstore` ABCI
    /// application.
    Kvstore,
    /// Execute a probe of a Gaia node.
    Gaia,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();
    let log_level = if opts.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    SimpleLogger::new().with_level(log_level).init().unwrap();

    let request_wait = Duration::from_millis(opts.request_wait);
    match opts.cmd {
        Command::Kvstore => kvstore::quick_probe_plan(&opts.output.0, request_wait)?,
        Command::Gaia => gaia::query_plan(&opts.output.0, request_wait)?,
    }
    .execute(&opts.addr)
    .await?;

    Ok(())
}
