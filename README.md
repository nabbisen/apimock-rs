# json-responder

Server generating JSON responses written in [Rust](https://www.rust-lang.org/).
Aims to help dev to easily get dummy API responses due to several paths.

[![License](https://img.shields.io/github/license/nabbisen/json-responder)](https://github.com/nabbisen/json-responder/blob/main/LICENSE)

## Summary

[hyper](https://hyper.rs/)-based HTTP server.

### Features

- GET / POST methods
- Multiple .json/.json5 files treated as JSON Response
- Multiple paths
- Error responses (HTTP 4xx and 5xx)
- Validates configuration: Missing JSON files, duplicate paths etc.
- Prints out routing at startup

#### Designed in mind with

- Performance
    - Fast speed
    - Low memory consumption
- Easy usage
    - Built as single (and small) executable
    - Integrated configuration
- Cross-platform support

### Screenshots

Server started to listen:

![server starts](.docs-assets/demo-01.png)

`curl` test result:

![server responds](.docs-assets/demo-02.png)

## Usage

[Releases](../../releases) are available. Also able to [build manually](#build-manually).

After downloading it or building, run `json-server` with your configuration file (`./json-server.toml` by default).

### Configure

`json-responder.toml`

```toml
[general]
port = 3001                                   # optional
data_dir = "tests"                            # optional
# always = "{ greetings: \"Hello, world.\" }" # optional

[url]
path_prefix = "api/v1" # optional
[url.paths] # required when `always` is not specified
home = "home.json"
# "some/path" = "subdir/some_path.json5"
[url.errors]
401 = ["api-401"]
403 = ["some/path/to/fail"]
404 = []
```

### Options

#### `-c` / `--config`

Config file path.
default: `json-responder.toml`

### After server started

What is modifiable:

- content of `.json` / `.json5`

What is NOT modifiable:

- `always` config
- routing on `paths` / `errors`

## Build manually

```
cargo build --release
```

Then run to start the server:

```
./target/release/json-responder
```

Alternatively, just running `cargo run` works.

## Acknowledgements

Depends on:

[tokio](https://github.com/tokio-rs/tokio) / [hyper](https://hyper.rs/) / [toml](https://github.com/toml-rs/toml) / [serde](https://serde.rs/) / [serde_json](https://github.com/serde-rs/json) / [json5](https://github.com/callum-oakley/json5-rs) / [console](https://github.com/console-rs/console). In addition, [mdbook](https://github.com/rust-lang/mdBook) (as to workflows)
