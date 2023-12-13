# Light-Client API for JavaScript

At present this just exposes the [CometBFT Light Client]'s verification logic
via WASM. This allows simple access to verification from JavaScript:

```javascript
import * as LightClient from 'cometbft-light-client-js';

// Verify an untrusted block against a trusted one, given the specified options
// and current date/time.
let verdict = LightClient.verify(untrusted, trusted, options, now);
```

For an example of how to use this, please see the [verifier-web example].

[CometBFT Light Client]: ../light-client/
[verifier-web example]: ./examples/verifier-web/
