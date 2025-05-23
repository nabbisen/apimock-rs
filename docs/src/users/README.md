# API mock (apimock-rs) User Documentation

## 🗺️ Overview

This guide walks through how to get started with `apimock-rs`, a TOML-configurable mock HTTP server for testing APIs. It covers basic setup, configuration, tips, frequently asked questions, advanced examples, and tutorials.

## 🏯 Architecture

**API mock (apimock-rs)** is a lightweight and developer-friendly HTTP mock server built in Rust. It provides file-based mechanism for mocking RESTful APIs using static JSON responses. Optionally, dynamic routing is supported: rule-based matching via `toml` and scripting via `rhai`.

### Key Features

- **Zero-config start** – Just point to a folder with JSON files and run.
- **File-based mocks** – Uses folder paths and `.json` files to simulate real API endpoints.
- **Rule-based mocks** – Supports conditional responses with `when`-`respond` rules defined in `.toml`.
- **Scriptable mocks** – Supports `.rhai` scripts for conditional responses.
- **High-performance** – Built with async Rust using `tokio` and `hyper`.
- **Simple setup** – Usable via a single CLI command, no compilation needed via npm-distributed binary.

## 😺 What's next ?

- 👟 [Getting started](./getting-started.md)
- 🧳 [Examples](./examples.md)
- 🍹 [FAQ](./faq.md)
- 🎒 [Configuration](./configuration/README.md)
    - 🍪 [File-based](./configuration//file-based.md)
    - 🍨 [Rule-based](./configuration/rule-based.md)
    - 🧁 [Scripting matching](./configuration//scripting-mappings.md)
