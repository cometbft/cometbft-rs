# Docker Configurations

This folder contains `Dockerfile` configurations that are used during
development and testing.

The created images are uploaded to DockerHub, under the informaldev
organization. For example: `informaldev/cometbft:0.34.0`.

## cometbft

This image is used during CI testing in the cometbft-rs crate and it can be
used during fixture creation with `rpc-probe`. It tests compatibility with the
CometBFT Go implementation. It is a GitHub Actions "Services"-compatible
image: a standalone image that can run on its own. It can create its own
configuration if one was not provided. This ensures that the configuration file
is always compatible with the CometBFT version built into it.

There are two configurations in this folder:

- [`cometbft`](./cometbft/) - Builds a CometBFT image based off of the
  `CMTVERSION` argument configured in the `Dockerfile`. The corresponding release
  will automatically be downloaded from
  [GitHub](https://github.com/cometbft/cometbft/releases) during image
  build.
- [`cometbft-custom-bin`](./cometbft-custom-bin/) - Builds a CometBFT
  image from a custom-built CometBFT binary. Assumes the `cometbft` binary
  is in the same directory as the `Dockerfile` and is built for Linux/AMD64.

## gaiad

This image will be used for `rpc-probe`, to generate fixtures for CI testing
from a gaiad node.

Contrary to the `cometbft` image, the configuration here is pre-created so the
genesis file can be populated with additional wallets. The corresponding private
keys are also saved into a test keyring.

All the configuration is in the `n0` folder. Two wallets are created `c0` and
`c1` (the validator's key is `n0`.) Both wallets have `uatom`, `stake` and
`n0token` added.

Both wallets have an initial signed transaction created for easier population of
the network before testing. These transactions will send uatom tokens from c0 ->
c1 and vice versa. They are both signed as `sequence 0` in the wallet, so they
can only be executed as the first transaction of the corresponding wallet.
