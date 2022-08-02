# echo-rest-api

> A simple echo REST and rRPC API

## How to run

- Clone this repo
- Run `cargo run`
- Service is running at: http://127.0.0.1:8080/echo

## Config

via environment variables:
- `HOST` (default `"127.0.0.1"`)
- `PORT` (default `8080`)
- `RUST_LOG` - (default "info" via `env_logger`)
- `SOCKET_ADDRESS` - (default `"127.0.0.1:10000"`)

## Tests

```shell
cargo test
```

## REST calls

### POST /echo

```shell
curl -v \
-d '{"message": "test"}' \
-H 'content-type: application/json' \
-H 'accept: application/json' \
http://127.0.0.1:8080/echo
```

### GET /echo


```shell
curl -v http://127.0.0.1:8080/echo
```

```shell
curl -v \
"http://127.0.0.1:8080/echo?message=test+with+spaces"
```

## gRPC calls

(via [`grpcurl`](https://github.com/fullstorydev/grpcurl))

```shell
grpcurl -plaintext -import-path ./proto -proto echo.proto \
-d '{"message": "test message"}' \
[::]:10000 echo.Echo/EchoMessage
```

[Bloom RPC](https://github.com/uw-labs/bloomrpc) is also an awesome client for gRPC :)

## OpenAPI file

### available at GET /openapi
