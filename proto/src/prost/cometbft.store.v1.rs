/// BlockStoreState represents the state of the block store.
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockStoreState {
    #[prost(int64, tag = "1")]
    pub base: i64,
    #[prost(int64, tag = "2")]
    pub height: i64,
}
