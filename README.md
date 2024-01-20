# json5-server

Generating JSON Responses Server written in [Rust](https://www.rust-lang.org/)

[![License](https://img.shields.io/github/license/nabbisen/json5-server-rs)](https://github.com/nabbisen/json5-server-rs/blob/main/LICENSE)

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
./target/release/json5-server
```

## Acknowledgements

Depends on [tokio](https://github.com/tokio-rs/tokio), and [warp](https://github.com/seanmonstar/warp) which is built on top of [hyper](https://hyper.rs/).
