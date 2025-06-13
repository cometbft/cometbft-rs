# Bisection test fixtures generator

## Usage

### NOTE

Currently, there's some discrepancy between the Go code and protobuf
declarations. For example, `Validator` struct contains `pub_key` while protobuf
version uses `pub_key_type` and `pub_key_bytes`. If you look at the code, you
can notice we're using a json marshaler that converts the `Validator` struct to
JSON. Ideally, we should first convert all structs to protobuf because Rust
uses serde and protobuf, and expects the same from Go. So this is kinda a hack
and something we should fix in the future. Specifically, we should:

1. Use [protojson](https://pkg.go.dev/google.golang.org/protobuf/encoding/protojson) to marshal the structs to JSON.
2. Double-check Go and Rust protos match.

But for now, you must fork CometBFT and the following code to `types/validator.go`:

```go
type ValidatorPB struct {
	Address     Address `json:"address"`
	VotingPower string  `json:"voting_power"`

	ProposerPriority string `json:"proposer_priority"`

	PubKeyType  string `json:"pub_key_type"`
	PubKeyBytes []byte `json:"pub_key_bytes"`
}

func (v *Validator) MarshalJSON() ([]byte, error) {
	t := ed25519.PubKeyName
	switch v.PubKey.Type() {
	case ed25519.KeyType:
		t = ed25519.PubKeyName
	case secp256k1.KeyType:
		t = secp256k1.PubKeyName
	default:
		panic(fmt.Sprintf("unsupported pubkey type %s", v.PubKey.Type()))
	}

	v2 := ValidatorPB{
		Address:          v.Address,
		PubKeyType:       t,
		PubKeyBytes:      v.PubKey.Bytes(),
		VotingPower:      fmt.Sprintf("%d", v.VotingPower),
		ProposerPriority: fmt.Sprintf("%d", v.ProposerPriority),
	}

	return json.Marshal(&v2)
}
```

Then in `go.mod`, add the following:

```
replace github.com/cometbft/cometbft v1.0.1 => {path-to-your-cometbft-repo}
```

Now, we can generate the bisection test fixtures.

```sh
$ go run .
```

Copy the resulting JSON files to the `light-client/tests/support/bisection` directory:

```sh
$ cp -R tests/json/bisection/* ../../light-client/tests/support/bisection/
```

## Testing

First, run all the previous steps to generate the bisection test fixtures.

Then, you can run the tests using:

```sh
$ go test -v ./...
```
