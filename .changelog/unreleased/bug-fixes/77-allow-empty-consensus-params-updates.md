- `[cometbft-rpc]` Deserialize an empty JSON object as `None` for the
  `consensus_param_updates` field in the `/block_results` response. Deserialize
  version in consensus params as `None` if it is an empty object, null or not
  found.
  ([\#77](https://github.com/cometbft/cometbft-rs/issues/77))
