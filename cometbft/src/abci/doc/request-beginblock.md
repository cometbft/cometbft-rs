Signals the beginning of a new block.

Called prior to any [`DeliverTx`]s. The `header` contains the height,
timestamp, and more -- it exactly matches the CometBFT block header.

**This is only called by CometBFT prior to version v0.38.0.**

[ABCI documentation](https://docs.cometbft.com/v0.37/spec/abci/abci++_methods#beginblock)
