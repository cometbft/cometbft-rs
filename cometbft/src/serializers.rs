//! Serde serializers
//!
//! Serializers and deserializers for a transparent developer experience.
//!
//! CAUTION: There are no guarantees for backwards compatibility, this module should be considered
//! an internal implementation detail which can vanish without further warning. Use at your own
//! risk.
use serde::{Deserialize, Deserializer};
use std::vec::Vec;

pub use cometbft_proto::serializers::*;

pub mod apphash;
pub mod apphash_base64;
pub mod hash;
pub mod option_hash;
pub mod time;

pub fn null_to_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_else(Vec::new))
}
