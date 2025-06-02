# Bisection test fixtures generator

## Usage

```sh
$ go run .
```

Copy the resulting JSON files to the `light-client/tests/support/bisection` directory:

```sh
$ cp -R tests/json/bisection/* ../../light-client/tests/support/bisection/
```

## Testing

```sh
$ go run .
$ go test -v ./...
```
