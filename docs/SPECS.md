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
        A[config mode if apimock.toml exists] --> B[config-less mode if apimock-dyn-route dir exists]
        B --> C['always' mode : fixed response]
        C --> D[middleware validation if exists]
    end
```

### How response works

```mermaid
graph
    subgraph Response workflow
        direction TB
        A[middleware if exists] --> B['always' is activated ?]
        B --> C[one of the commands is accessed ?]
        C --> D['path.urls' have the path ?]
        D --> E[matcher exists in jsonpath patterns ?]
        E --> F[exists in 'dyn_data_dir' ?]
    end
```
