# json-responder

Generating JSON Responses Server written in [Rust](https://www.rust-lang.org/)

[![License](https://img.shields.io/github/license/nabbisen/json-responder-rs)](https://github.com/nabbisen/json-responder-rs/blob/main/LICENSE)

## Summary

Designed in mind with:

- Fast performance and low memory consumption
- Cross-platform support

Features as planned are below:

- Will support [JSON5](https://json5.org/)
- Will support multiple path classes
- Will support multiple .json/.json5 files treated as JSON Response

## Usage

### Build

```
cargo build --release
```

### Run

```
./target/release/json-responder
```

### Configure (todo)

`json-responder.toml`

```toml
[general]
port = 3001 # optional
data_dir = "."
always = "{ greetings: \"Hello, world.\" }"
[url]
path_prefix = "api/v1"
[url.paths]
home = "home.json"
"some/path" = "subdir/some_path.json5"
```

### Options (todo)

Available are config file path and those in `[general]` only.

#### `-c` / `--config`

config file path (todo)

#### `-p` / `--port`

port number (todo)

#### `-a` / `--always`

always returns fixed response (todo)

## Acknowledgements

Depends on [tokio](https://github.com/tokio-rs/tokio), and [warp](https://github.com/seanmonstar/warp) which is built on top of [hyper](https://hyper.rs/).
