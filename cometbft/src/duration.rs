use core::convert::{TryFrom, TryInto};

use crate::error::Error;
use crate::serializers;

use cometbft_proto::google::protobuf::Duration as RawDuration;
use cometbft_proto::Protobuf;
use serde::{Deserialize, Serialize};

/// Duration is a wrapper around core::time::Duration
/// essentially, to keep the usages look cleaner
/// i.e. you can avoid using serde annotations everywhere
/// Todo: harmonize google::protobuf::Duration, core::time::Duration and this. Too many structs.
/// <https://github.com/informalsystems/tendermint-rs/issues/741>
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Duration(#[serde(with = "serializers::time_duration")] pub core::time::Duration);

impl From<Duration> for core::time::Duration {
    fn from(d: Duration) -> core::time::Duration {
        d.0
    }
}

impl Protobuf<RawDuration> for Duration {}

impl TryFrom<RawDuration> for Duration {
    type Error = Error;

    fn try_from(value: RawDuration) -> Result<Self, Self::Error> {
        Ok(Self(core::time::Duration::new(
            value.seconds.try_into().map_err(Error::integer_overflow)?,
            value.nanos.try_into().map_err(Error::integer_overflow)?,
        )))
    }
}

impl From<Duration> for RawDuration {
    fn from(value: Duration) -> Self {
        // Todo: make the struct into a proper domaintype so this becomes infallible.
        Self {
            seconds: value.0.as_secs() as i64,
            nanos: value.0.subsec_nanos() as i32,
        }
    }
}
