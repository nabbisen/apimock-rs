# Architecture

## Designed in mind with

- Performance
    - Fast speed, low memory consumption.
- Easy setup / usage
    - Built as single (and small) executable, integrated configuration. (No need to write scripts, config-less mode is also supported.)
- Cross-platform support

## Core Components

- **`src/core/server/routing.rs`**  
  Contains core logic including request routing, matching rules, response rendering, and script evaluation.

- **`src/server.rs`**  
  The HTTP server entry point powered by `hyper`. It handles incoming requests and delegates them to the core logic.

- **`src/config.rs`**  
  Defines configuration structures for customizable parameters like the server port, root path, logging levels, etc.

- **`examples/` directory**  
  Includes sample API directory structures and mock files to help you get started quickly. Also, `cargo test` resources in `config/tests/`.
