# apimock-rs

Mocking helper to develop microservices and APIs. [hyper](https://hyper.rs/)-based HTTP server generating REST responses containing JSON ones. Written in [Rust](https://www.rust-lang.org/).

[![License](https://img.shields.io/github/license/nabbisen/apimock-rs)](https://github.com/nabbisen/apimock-rs/blob/main/LICENSE)

## Summary

Aims to help developers to easily get responses from dummy API, especially microservice API,  according to several paths.
Each single executable on Win/Mac/Linux are available, thanks to Rust and their cross-platform support. [Releases](../../releases) are "out-of-the-box" coming with default config `apimock.toml`.

\* Renamed from `json-responder`. Now more than returning JSON data.

### Screenshots

Server started to listen:

![server starts](.docs-assets/demo-01.png)

`curl` test result:

![server responds](.docs-assets/demo-02.png)

### Designed in mind with

- Performance
    - Fast speed, low memory consumption.
- Easy setup/usage
    - Built as single (and small) executable, integrated configuration. (No need to write scripts, config-less mode is also supported.)
- Cross-platform support

### Features

- GET / POST methods
- Multiple paths
- Multiple .json/.json5 files treated as JSON Response
- Flexible responses with patterns and jsonpath queries. Even with the same api uri path, multiple responses can be returned. See [\[`url.paths_patterns`\]](#urlpaths_patterns).
- Can switch data directory paths manually in testing via specific HTTP request to make json responses flexible
- Dynamic path resolution with `dyn_data_dir`
- Custom responses codes (HTTP 3xx as redirects and 4xx and 5xx as errors)
- Custom headers and their reusabliblity
- Validates configuration: Missing JSON files, duplicate paths etc.
- Prints out routing at startup

## Installation

- [Releases](../../releases) are available.
  - Create your configuration file (`./apimock.toml` by default) and run `apimock` with it.
- Via cargo: `cargo install apimock`
- Also able to build manually.
  - Run `cargo build --release`. Then run to start the server: `./target/release/apimock`.
  - Alternatively, just running `cargo run` works.

Running `apimock` without both `apimock.toml` and `apimock-data/` directory results in `always` option activated.

### How startup works

```mermaid
graph
    subgraph Startup workflow
        direction TB
        A[config mode if apimock.toml exists] --> B[config-less mode if apimock-data dir exists]
        B --> C[`always` mode : fixed response]
    end
```

## Usage

### Configure

`apimock.toml`

```toml
[general]
port = 3001
dyn_data_dir = "apimock-data"
# always = "{ greetings: \"Hello, world.\" }"

[url]
data_dir = "tests"
data_dir_query_path = "@@"
path_prefix = "api/v1"

[url.headers]
cookie_1 = { key = "Set-Cookie", value = "a=b; c=d" }
redirect_1 = { key = "Location", value = "/api/v1/home" }

# required when `always` is not specified
[url.paths] # `path_prefix` works
"home" = "home.json"
# "some/path" = "api.json5"
# custom headers
"some/path/w/header" = { src = "home.json", headers = ["cookie_1"] }
# errors / redirects * `code` must be defined as **unsigned integer** (instead of String)
"error/401" = { code = 401 }
"error/api-403" = { code = 403 }
"redirect/302" = { code = 302, headers = ["redirect_1"] }

[url.paths_patterns."some/path/w/matcher"."a.b.c"]
"=1" = "api.json5"
"=0" = "home.json"
[url.paths_patterns."some/path/w/matcher"."d.2.e"]
"=x=" = "api.json5"
[url.paths_patterns."some/path/w/matcher"."f"]
"=" = "api.json5"

[url.raw_paths] # `path_prefix` doesn't work
"/" = { text = "{ Hello: world }", code = 301, headers = ["cookie_1", "redirect_1"] }
```

#### Properties

##### `general.dyn_data_dir`

If set, URL path without statically defined path matched is converted to file path in this directory. Server tries to find it out as either `.json` or `.json5`. When found, server returns the content as JSON response.    
**Default**: empty

It works even without config toml. It is config-less mode.

##### `url.data_dir`

Data directory used as where to look up files when HTTP response is built.    
**Default**: executable directory

##### `url.data_dir_query_path`

Data directory can be switched manually via HTTP request. Access to http://127.0.0.1/(`url.data_dir_query_path`) to get the current value. Access to http://127.0.0.1/(`url.data_dir_query_path`)/some/path to change it.
**Default**: "@@"

##### `url.path_prefix`

Static paths are dealt with as those who have the prefix. Convenient when your service has path prefix.    
**Default**: empty

##### `url.headers`

HTTP headers such as `Authorizaton: xxx` on auth and `Location: xxx` on redirection.
You can reuse them and easily attach headers in `url.paths` by defining here.    
**Default**: None

##### `url.paths`

The key, the left-hand side, is URL path. The right-hand one is response definition.
Response definition consists of four optional parts: `code` as HTTP code, `headers` as HTTP headers keys defined in `url.headers`, `src` as data source file relative path in `url.data_dir` and `text` as direct body text instead of `src`. For example:

```toml
"url_path" = { code = 200, headers = ["header_key_1"], src = "response_1.json" }
```

It is able to omit code and headers. For example:

```toml
"url_path" = "response_1.json"
```

It means `src` and it's far simpler. `code` and `headers` are dealt with as their default: 200 as OK and no custom headers.

Only when either `src` or `text` is defined, the response `Content-Type` is set as `application/json`.

##### `url.paths_patterns`

You can define patterns with which response JSON file is dynamically decided due to request body parameter.

The format is:

```toml
[url.paths_patterns."{API_PATH}"."{JSONPATH}"]
"={CASE_VALUE}" = "{DATA_SRC}"
```

For example, with the definition below, you can return "special.json" when "/some/matcher/path" is accessed to and the request body is "{\"key_a\": {\"key_b\": {\"key_c\": 1}}}":

```toml
[url.paths_patterns."/some/matcher/path"."key_a.Key_b.key_c"]
"=1" = "special.json"
```

Remember:

- Enclose API path and JSONPath with `"`
- Start with `=` in writing pattern value

Array is also available with index number specified. For example, when the request body is "{\"key_d\": [{}, {}, {\"key_e\": \"x=\"}]}", how to point to it is: 

```toml
[url.paths_patterns."/some/matcher/path"."key_d.2.key_e"]
"=x=" = "special.json"
```

`2` in "key_d.**2**.key_e" is the index.

##### `url.raw_paths`

Not affected by `url.path_prefix`. Everything else is the same to `url.paths`.

### Options

#### `-c` / `--config`

Config file path.
default: `apimock.toml`

### After server started

What is modifiable:

- content of path data src: `.json` / `.json5`

What is NOT modifiable:

- `always` config
- routing on `paths`
- `code` / `headers` / data text on each path

### How response works

```mermaid
graph
    subgraph Response workflow
        direction TB
        A[`always` is activated ?] --> B[`data_dir_query_path` accessed ?]
        B --> C[`path.urls` have the path ?]
        C --> D[matcher exists in jsonpath patterns ?]
        D --> E[exists in `dyn_data_dir` ?]
    end
```

## How to embed to development environment

With Node.js project, `scripts` in `package.json` is available.
For example, run `npm run apimock` with `package.json` written in as below:

```json
{
  "scripts": {
    "apimock": "./apimock"
  }
}
```

## Acknowledgements

Depends on:

[tokio](https://github.com/tokio-rs/tokio) / [hyper](https://hyper.rs/) / [toml](https://github.com/toml-rs/toml) / [serde](https://serde.rs/) / [serde_json](https://github.com/serde-rs/json) / [json5](https://github.com/callum-oakley/json5-rs) / [console](https://github.com/console-rs/console). In addition, [mdbook](https://github.com/rust-lang/mdBook) (as to workflows)
