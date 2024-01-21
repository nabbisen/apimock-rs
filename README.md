# json-responder

Generating JSON Responses Server written in [Rust](https://www.rust-lang.org/)

[![License](https://img.shields.io/github/license/nabbisen/json-responder-rs)](https://github.com/nabbisen/json-responder-rs/blob/main/LICENSE)

## Summary

Designed in mind with:

- Fast performance and low memory consumption
- Cross-platform support

### Features

- [JSON5](https://json5.org/) support
- Multiple paths
- Multiple .json/.json5 files treated as JSON Response

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
port = 3001                                 # optional
data_dir = "./"                             # optional
always = "{ greetings: \"Hello, world.\" }" # optional

[url]
path_prefix = "api/v1" # optional
[url.paths]            # required when `always` is not specified
home = "home.json"
"some/path" = "subdir/some_path.json5"
```

### Options (todo: planned)

Available are config file path and those in `[general]` only.

#### `-c` / `--config`

config file path (todo)
default: json-responder.toml

#### `-p` / `--port`

port number (todo)
default: 3001

#### `-a` / `--always`

always returns fixed response (todo)
default: None (= disabled)

## Acknowledgements

Depends on:

[tokio](https://github.com/tokio-rs/tokio) / [hyper](https://hyper.rs/) [toml](https://github.com/toml-rs/toml) / [serde](https://serde.rs/) / [serde_json](https://github.com/serde-rs/json) / [json5](https://github.com/callum-oakley/json5-rs) / [console](https://github.com/console-rs/console)
