# apimock-rs

[![crates.io](https://img.shields.io/crates/v/apimock?label=latest)](https://crates.io/crates/apimock)
[![Documentation](https://docs.rs/apimock/badge.svg?version=latest)](https://docs.rs/apimock/latest)
[![Dependency Status](https://deps.rs/crate/apimock/latest/status.svg)](https://deps.rs/crate/apimock/latest)
[![Releases Workflow](https://github.com/nabbisen/apimock-rs/actions/workflows/release.yml/badge.svg)](https://github.com/nabbisen/apimock-rs/actions/workflows/)
[![License](https://img.shields.io/github/license/nabbisen/apimock-rs)](https://github.com/nabbisen/apimock-rs/blob/main/LICENSE)

## Summary

Mocking helper to develop microservices and APIs. [hyper](https://hyper.rs/)-based HTTP server generating REST responses containing JSON ones. Written in [Rust](https://www.rust-lang.org/).

With small native binary, just run `./apimock` or `./apimock -c apimock.toml` to start the server !

- [Install and usage](docs/INSTALL.md)

## Screenshots

Server started to listen:

![server starts](.docs-assets/demo-01.png)

`curl` test result:

![server responds](.docs-assets/demo-02.png)

## Features

### 1. Basic

- GET / POST methods
- Multiple [`paths`](docs/CONFIGURE.md#urlpaths)
- Multiple .json/.json5 files treated as JSON Response
- based on hyper v1

### 2. Customization

- Custom HTTP response codes: 3xx as redirects, and 4xx and 5xx as errors
- Custom response [headers](docs/CONFIGURE.md#urlheaders) which are reusable
- Flexible responses with patterns and jsonpath queries. Even with the same API URL path, multiple responses can be returned. See [`url.paths_patterns`](docs/CONFIGURE.md#urlpaths_patterns)
- Can specify response time on all or each API path

### 3. Dynamic processing

- Can [switch data directory paths](docs/CONFIGURE.md#urldata_dir_query_path) manually in testing via specific HTTP request to make json responses flexible
- Dynamic path resolution with [`dyn_data_dir`](docs/CONFIGURE.md#generaldyn_data_dir)

### 4. Usability

- Validates configuration: Missing JSON files, duplicate paths etc.
- Prints out routing at startup

### Reference

- [Configuration and options](docs/CONFIGURE.md)
- [Design and specification](docs/SPECS.md)

## Acknowledgements

Depends on:

[tokio](https://github.com/tokio-rs/tokio) / [hyper](https://hyper.rs/) / [toml](https://github.com/toml-rs/toml) / [serde](https://serde.rs/) / [serde_json](https://github.com/serde-rs/json) / [json5](https://github.com/callum-oakley/json5-rs) / [console](https://github.com/console-rs/console). In addition, [mdbook](https://github.com/rust-lang/mdBook) (as to workflows)
