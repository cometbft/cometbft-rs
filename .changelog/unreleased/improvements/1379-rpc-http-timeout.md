- Allow specifying a request timeout for the RPC `HttpClient` ([#20](https://github.com/cometbft/cometbft-rs/issues/20))
  The `http::Builder` struct now provides a `.timeout(Duration)` method to specify the request timeout. 
  If not specified, the default value is 30 seconds
