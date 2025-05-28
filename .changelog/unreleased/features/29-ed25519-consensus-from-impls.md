- `[cometbft]` Add the following impls for `ed25519-consensus`:
  * `From<ed25519_consensus::SigningKey` for `cometbft::PrivateKey`
  * `From<ed25519_consensus::SigningKey>` for `cometbft::SigningKey`
  * `From<ed25519_consensus::VerificationKey>` for `cometbft::PublicKey`
  * `From<ed25519_consensus::VerificationKey>` for `cometbft::VerificationKey`
  ([\#29](https://github.com/cometbft/cometbft-rs/issues/29))
