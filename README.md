# apimock-rs

[![License](https://img.shields.io/github/license/nabbisen/apimock-rs)](https://github.com/nabbisen/apimock-rs/blob/main/LICENSE)

## Summary

Mocking helper to develop microservices and APIs. [hyper](https://hyper.rs/)-based HTTP server generating REST responses containing JSON ones. Written in [Rust](https://www.rust-lang.org/).

- [Install and usage](docs/INSTALL.md)

## Screenshots

Server started to listen:

![server starts](.docs-assets/demo-01.png)

`curl` test result:

![server responds](.docs-assets/demo-02.png)

## Features

- GET / POST methods
- Multiple paths
- Multiple .json/.json5 files treated as JSON Response
- Flexible responses with patterns and jsonpath queries. Even with the same api uri path, multiple responses can be returned. See [\[`url.paths_patterns`\]](docs/CONFIGURE.md#urlpaths_patterns).
- Can switch data directory paths manually in testing via specific HTTP request to make json responses flexible
- Dynamic path resolution with `dyn_data_dir`
- Custom responses codes (HTTP 3xx as redirects and 4xx and 5xx as errors)
- Custom headers and their reusabliblity
- Validates configuration: Missing JSON files, duplicate paths etc.
- Prints out routing at startup

### Description

- [Configuration and options](docs/CONFIGURE.md)
- [Design and specification](docs/SPECS.md)

## Acknowledgements

Depends on:

[tokio](https://github.com/tokio-rs/tokio) / [hyper](https://hyper.rs/) / [toml](https://github.com/toml-rs/toml) / [serde](https://serde.rs/) / [serde_json](https://github.com/serde-rs/json) / [json5](https://github.com/callum-oakley/json5-rs) / [console](https://github.com/console-rs/console). In addition, [mdbook](https://github.com/rust-lang/mdBook) (as to workflows)
