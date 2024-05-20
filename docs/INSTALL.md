# Install

## Introduction

Aims to help developers to easily get responses from dummy API, especially microservice API, according to several paths.
Each single executable on Win/Mac/Linux are available, thanks to Rust and their cross-platform support. [Releases](../../../releases) are "out-of-the-box" coming with default config `apimock.toml`.

## Installation

- [Releases](../../../releases) are available.
  - Create your configuration file (`./apimock.toml` by default) and run `apimock` with it.
- Via cargo: `cargo install apimock`
- Also able to build manually.
  - Run `cargo build --release`. Then run to start the server: `./target/release/apimock`.
  - Alternatively, just running `cargo run` works.

Running `apimock` without both `apimock.toml` and `apimock-data/` directory results in `always` option activated.

## Usage

### After server started

There are some modifiable settings on running server:

- `.json` / `.json5` content of `src` in `paths`, `raw_paths`, and those in `dyn_data_dir`
- `data_dir` in `paths` and `paths_patterns`

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
