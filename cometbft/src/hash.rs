//! Hash functions and their outputs

use core::{
    convert::TryFrom,
    fmt::{self, Debug, Display},
    str::FromStr,
};

use bytes::Bytes;
use cometbft_proto::serializers::cow_str::CowStr;
use cometbft_proto::Protobuf;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use subtle_encoding::{base64, Encoding, Hex};

use crate::{error::Error, prelude::*};

/// Output size for the SHA-256 hash function
pub const SHA256_HASH_SIZE: usize = 32;

/// Hash algorithms
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Algorithm {
    /// SHA-256
    Sha256,
}

/// Hash digests
#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Default)]
pub enum Hash {
    /// SHA-256 hashes
    Sha256([u8; SHA256_HASH_SIZE]),
    /// Empty hash
    #[default]
    None,
}

impl Protobuf<Vec<u8>> for Hash {}

/// Default conversion from `Vec<u8>` is SHA256 Hash or `None`
impl TryFrom<Vec<u8>> for Hash {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Ok(Hash::None);
        }
        Hash::from_bytes(Algorithm::Sha256, &value)
    }
}

impl From<Hash> for Vec<u8> {
    fn from(value: Hash) -> Self {
        match value {
            Hash::Sha256(s) => s.to_vec(),
            Hash::None => vec![],
        }
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        match self {
            Hash::Sha256(ref h) => h.as_ref(),
            Hash::None => &[],
        }
    }
}

impl From<Hash> for Bytes {
    fn from(h: Hash) -> Self {
        Self::copy_from_slice(h.as_ref())
    }
}

impl TryFrom<Bytes> for Hash {
    type Error = Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Self::from_bytes(Algorithm::Sha256, value.as_ref())
    }
}

impl Hash {
    /// Create a new `Hash` with the given algorithm type
    pub fn from_bytes(alg: Algorithm, bytes: &[u8]) -> Result<Hash, Error> {
        if bytes.is_empty() {
            return Ok(Hash::None);
        }
        match alg {
            Algorithm::Sha256 => {
                if bytes.len() == SHA256_HASH_SIZE {
                    let mut h = [0u8; SHA256_HASH_SIZE];
                    h.copy_from_slice(bytes);
                    Ok(Hash::Sha256(h))
                } else {
                    Err(Error::invalid_hash_size())
                }
            },
        }
    }

    /// Decode a `Hash` from upper-case hexadecimal
    pub fn from_hex_upper(alg: Algorithm, s: &str) -> Result<Hash, Error> {
        if s.is_empty() {
            return Ok(Hash::None);
        }
        match alg {
            Algorithm::Sha256 => {
                let mut h = [0u8; SHA256_HASH_SIZE];
                Hex::upper_case()
                    .decode_to_slice(s.as_bytes(), &mut h)
                    .map_err(Error::subtle_encoding)?;
                Ok(Hash::Sha256(h))
            },
        }
    }

    /// Return the digest algorithm used to produce this hash
    pub fn algorithm(self) -> Algorithm {
        match self {
            Hash::Sha256(_) => Algorithm::Sha256,
            Hash::None => Algorithm::Sha256,
        }
    }

    /// Borrow the `Hash` as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Hash::Sha256(ref h) => h.as_ref(),
            Hash::None => &[],
        }
    }

    /// Convenience function to check for Hash::None
    pub fn is_empty(&self) -> bool {
        self == &Hash::None
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hash::Sha256(_) => write!(f, "Hash::Sha256({self})"),
            Hash::None => write!(f, "Hash::None"),
        }
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = match self {
            Hash::Sha256(ref h) => Hex::upper_case().encode_to_string(h).unwrap(),
            Hash::None => String::new(),
        };

        write!(f, "{hex}")
    }
}

impl FromStr for Hash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Self::from_hex_upper(Algorithm::Sha256, s)
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let hex = CowStr::deserialize(deserializer)?;

        if hex.is_empty() {
            Err(D::Error::custom("empty hash"))
        } else {
            Ok(Self::from_str(&hex).map_err(|e| D::Error::custom(format!("{e}")))?)
        }
    }
}

impl Serialize for Hash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

/// Serialization/deserialization for `Hash` that allows for empty hashes.
pub mod allow_empty {
    use super::*;

    /// Serialize [`Hash`](enum@crate::hash::Hash) into a string.
    pub fn serialize<S>(value: &Hash, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.to_string().serialize(serializer)
    }

    /// Deserialize [`Hash`](enum@crate::hash::Hash) from a string, allowing for
    /// empty hashes.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Hash, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex = CowStr::deserialize(deserializer)?;
        Hash::from_str(&hex).map_err(serde::de::Error::custom)
    }
}

/// AppHash is usually a SHA256 hash, but in reality it can be any kind of data
#[derive(Clone, PartialEq, Eq, Default)]
pub struct AppHash(Vec<u8>);

impl Protobuf<Vec<u8>> for AppHash {}

impl TryFrom<Vec<u8>> for AppHash {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(AppHash(value))
    }
}
impl From<AppHash> for Vec<u8> {
    fn from(value: AppHash) -> Self {
        value.0
    }
}

impl TryFrom<Bytes> for AppHash {
    type Error = Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(AppHash(value.to_vec()))
    }
}
impl From<AppHash> for Bytes {
    fn from(value: AppHash) -> Self {
        value.0.into()
    }
}

impl AppHash {
    /// Return the hash bytes as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Decode a `Hash` from upper-case hexadecimal
    pub fn from_hex_upper(s: &str) -> Result<Self, Error> {
        if s.len() % 2 != 0 {
            return Err(Error::invalid_app_hash_length());
        }
        let mut h = vec![0; s.len() / 2];
        Hex::upper_case()
            .decode_to_slice(s.as_bytes(), &mut h)
            .map_err(Error::subtle_encoding)?;
        Ok(AppHash(h))
    }

    /// Decode a `Hash` from base64-encoded string
    pub fn from_base64(s: &str) -> Result<Self, Error> {
        let h = base64::decode(s).map_err(Error::subtle_encoding)?;
        Ok(AppHash(h))
    }
}

impl AsRef<[u8]> for AppHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Debug for AppHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AppHash({})",
            Hex::upper_case().encode_to_string(&self.0).unwrap()
        )
    }
}

impl Display for AppHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Hex::upper_case().encode_to_string(&self.0).unwrap()
        )
    }
}

impl FromStr for AppHash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Self::from_hex_upper(s).or_else(|_| Self::from_base64(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, serde::Deserialize)]
    struct AppHashTest {
        #[serde(default)]
        #[serde(with = "crate::serializers::apphash")]
        pub app_hash: AppHash,
    }

    #[derive(Debug, serde::Deserialize)]
    struct HashTest {
        hash: Hash,
        #[serde(with = "super::allow_empty")]
        empty_hash: Hash,
    }

    #[test]
    fn apphash_decode_base64() {
        let test = serde_json::from_str::<AppHashTest>(
            r#"{"app_hash":"MfX9f+bYoI8IioRb4YT/8/VhPvtNjgWFgTi4mmMSkBc="}"#,
        )
        .unwrap();

        assert_eq!(
            test.app_hash.as_ref(),
            &[
                0x31, 0xF5, 0xFD, 0x7F, 0xE6, 0xD8, 0xA0, 0x8F, 0x08, 0x8A, 0x84, 0x5B, 0xE1, 0x84,
                0xFF, 0xF3, 0xF5, 0x61, 0x3E, 0xFB, 0x4D, 0x8E, 0x05, 0x85, 0x81, 0x38, 0xB8, 0x9A,
                0x63, 0x12, 0x90, 0x17
            ]
        );
    }

    #[test]
    fn apphash_decode_hex() {
        let test = serde_json::from_str::<AppHashTest>(
            r#"{"app_hash":"31F5FD7FE6D8A08F088A845BE184FFF3F5613EFB4D8E05858138B89A63129017"}"#,
        )
        .unwrap();

        assert_eq!(
            test.app_hash.as_ref(),
            &[
                0x31, 0xF5, 0xFD, 0x7F, 0xE6, 0xD8, 0xA0, 0x8F, 0x08, 0x8A, 0x84, 0x5B, 0xE1, 0x84,
                0xFF, 0xF3, 0xF5, 0x61, 0x3E, 0xFB, 0x4D, 0x8E, 0x05, 0x85, 0x81, 0x38, 0xB8, 0x9A,
                0x63, 0x12, 0x90, 0x17
            ]
        );
    }

    #[test]
    fn hash_decode_hex() {
        let s = r#"{
            "hash": "9F86D081884C7D659A2FEAA0C55AD015A3BF4F1B2B0B822CD15D6C15B0F00A08",
            "empty_hash": ""
        }"#;

        let expected_hash = &[
            0x9F, 0x86, 0xD0, 0x81, 0x88, 0x4C, 0x7D, 0x65, 0x9A, 0x2F, 0xEA, 0xA0, 0xC5, 0x5A,
            0xD0, 0x15, 0xA3, 0xBF, 0x4F, 0x1B, 0x2B, 0x0B, 0x82, 0x2C, 0xD1, 0x5D, 0x6C, 0x15,
            0xB0, 0xF0, 0x0A, 0x08,
        ];

        let test = serde_json::from_str::<HashTest>(s).unwrap();
        assert_eq!(test.hash.as_ref(), expected_hash);
        assert_eq!(test.empty_hash, Hash::None);

        // Test issue 1474
        let json_value = serde_json::from_str::<serde_json::Value>(s).unwrap();
        let test = serde_json::from_value::<HashTest>(json_value).unwrap();
        assert_eq!(test.hash.as_ref(), expected_hash);
        assert_eq!(test.empty_hash, Hash::None);
    }
}
