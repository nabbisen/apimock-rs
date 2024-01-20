## Summary

Generating JSON Responses Server written in [Rust](https://www.rust-lang.org/)

### Features (Planned)

- Fast performance and low memory consumption based on [warp](https://github.com/seanmonstar/warp) / [hyper](https://hyper.rs/)
- Cross-platform support

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

