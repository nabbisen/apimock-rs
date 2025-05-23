# Features

## Introduction

Aims to help developers to easily get responses from dummy API, especially microservice API, according to several paths.

Each single executable on Win/Mac/Linux are available, thanks to Rust and their cross-platform support. [Assets](https://github.com/apimokka/apimock-rs/releases/latest) in Releases are "out-of-the-box" coming with default config `apimock.toml`.

## 1. Basic

- GET / POST methods
- Multiple [`paths`](docs/CONFIGURE.md#urlpaths)
- Multiple .json/.json5 files treated as JSON Response
- based on hyper v1

## 2. Customization

- Custom HTTP response codes: 3xx as redirects, and 4xx and 5xx as errors
- Custom response [headers](docs/CONFIGURE.md#urlheaders) which are reusable
- Can specify response time on all or each API path
- [Middleware](docs/CONFIGURE.md#middleware) as [`rhai`](https://github.com/rhaiscript/rhai) script (ref: [Rhai book](https://rhai.rs/book/language/statements.html) (Statements chapter)) is available to customize request routing and response handling

## 3. Dynamic processing

- Flexible responses with patterns and jsonpath queries. Even with the same API URL path, multiple responses can be returned. See [`url.paths_patterns`](docs/CONFIGURE.md#urlpaths_patterns)
- Dynamic path resolution with [`dyn_data_dir`](docs/CONFIGURE.md#generaldyn_data_dir)

## 4. Safe and observable usage

- Validates configuration: Missing JSON files, duplicate paths etc.
- Prints out routing at startup
- Describes request content on both HTTP headers and body (json or plain text) when [`verbose`](docs/CONFIGURE.md#generalverbose) log is activated

## ~~. Test helper~~

- ~~Can [switch data directory paths](docs/CONFIGURE.md#urldata_dir_query_path) manually in testing via specific HTTP request to make json responses flexible~~

Commands support might come in the future. (todo)

## 6. crate features

- `spawn`: when activated, the server is available as subprocess. The output will be returned via tokio mpsc queue.
