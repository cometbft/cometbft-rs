- `[cometbft]` Change `EventAttribute`'s `key` and `value` fields from `String` to `Vec<u8>
` for CometBFT v0.34, as enforced by the Protobuf schema for CometBFT v0.34.
  `cometbft::abci::EventAttribute` is now an enum, to account for version 0.34 and 0.37+, t
herefore the `key`, `value` and `index` fields now have to be retrieved through the `key_str(
)`/`key_bytes`, `value_str()`/`value_bytes()` and `index()` methods.
  ([\#31](https://github.com/cometbft/cometbft-rs/issues/31)).
