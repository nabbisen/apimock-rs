# Features

Aims to help developers to easily get responses from dummy API, especially microservice API, according to several paths.

Each single executable on Win/Mac/Linux are available, thanks to Rust and their cross-platform support. Installation is available via `npm install` or getting [Assets](https://github.com/apimokka/apimock-rs/releases/latest) in GitHub Releases. They are "out-of-box".

## 1. Basic

- GET / POST methods
- File-based routing frees us around hard setup
- Support `.json` / `.json5` / `.csv` files treated as JSON Response
- based on hyper v1

## 2. Customization

- Rule-based routing empowers us
- Can specify response time on all or each API path
- Custom HTTP response codes: 3xx as redirects, and 4xx and 5xx as errors

## 3. Dynamic processing

- Flexible responses with condition combination. Even with the same API URL path, multiple responses can be returned.
- Optionally, middleware as Rhai scripts is available to customize request routing and response handling

## 4. Safe and observable usage

- Validates configuration: Missing JSON files, duplicate paths etc.
- Prints out routing at startup
- Describes request content on both HTTP headers and body (json or plain text) when [`verbose`](docs/CONFIGURE.md#generalverbose) on log config is enabled
- Integrated test cases for app stability and robustness

## 5. Rust crate features

- `spawn`: when activated, the server is available as subprocess. The output will be returned via tokio mpsc queue.
