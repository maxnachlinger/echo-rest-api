# echo-rest-api

> A simple echo REST API

## How to run

- Clone this repo
- Run `cargo run`
- Service is running at: http://127.0.0.1:8080/echo

## Config

via environment variables:
- `HOST` (default `"127.0.0.1"`)
- `PORT` (default `8080`)
- `RUST_LOG` - default "info" via `env_logger`

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

## OpenAPI file

### available at GET /openapi
