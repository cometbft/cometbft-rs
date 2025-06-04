//! CometBFT is a high-performance blockchain consensus engine that powers
//! Byzantine fault tolerant applications written in any programming language.
//! This crate provides core types for representing information about CometBFT
//! blockchain networks, including chain information types, secret connections,
//! and remote procedure calls (JSON-RPC).

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(
    warnings,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cometbft/cometbft-rs/main/img/logo-cometbft-rs_3961x4001.png"
)]

extern crate alloc;

#[cfg(any(feature = "std", test))]
extern crate std;

#[macro_use]
mod proto_macros;

pub mod error;

pub mod abci;
pub mod account;
pub mod block;
pub mod bytes;
pub mod chain;
pub mod channel;
pub mod consensus;
pub mod crypto;
pub mod duration;
pub mod evidence;
pub mod genesis;
pub mod hash;
pub mod merkle;
mod moniker;
pub mod node;
mod prelude;
pub mod private_key;
pub mod privval;
pub mod proposal;
pub mod public_key;
pub mod serializers;
pub mod signature;
pub mod time;
mod timeout;
pub mod trust_threshold;
pub mod tx;
pub mod validator;
mod version;
pub mod vote;

#[cfg(test)]
mod test;

pub use crate::{
    block::Block,
    error::Error,
    genesis::Genesis,
    hash::{AppHash, Hash},
    moniker::Moniker,
    private_key::PrivateKey,
    proposal::Proposal,
    public_key::{CometbftKey, PublicKey},
    signature::Signature,
    time::Time,
    timeout::Timeout,
    version::Version,
    vote::Vote,
};
