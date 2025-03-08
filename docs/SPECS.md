# Specs

## Designed in mind with

- Performance
    - Fast speed, low memory consumption.
- Easy setup/usage
    - Built as single (and small) executable, integrated configuration. (No need to write scripts, config-less mode is also supported.)
- Cross-platform support

## How startup works

```mermaid
graph
    subgraph Startup workflow
        direction TB
        A[config mode if apimock.toml exists] --> B[config-less mode if apimock-dyn-data dir exists]
        B --> C[`always` mode : fixed response]
    end
```

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
