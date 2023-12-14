Signals the beginning of a new block.

Called prior to any [`DeliverTx`]s. The `header` contains the height,
timestamp, and more -- it exactly matches the CometBFT block header.

[ABCI documentation](https://docs.cometbft.com/v1/spec/abci/abci.html#beginblock)