- `[cometbft]` Bump `prost` and `prost-types` to their latest versions in the
  `cometbft` crate. This was missed in
  [#64](https://github.com/cometbft/cometbft-rs/pull/64), which only updated the
  two dependencies in `cometbft-rpc`, leading to duplicate versions of both
  crates to be present in the dependency graph.
  ([\#84](https://github.com/cometbft/cometbft-rs/issues/84))
