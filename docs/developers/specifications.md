# Specifications

## Request and Response

### Request Matching

Matching is performed based on:

- URL path (e.g. `/api/v1/function`)
- Headers
- Request body content (JSON only)

### Response Sources

Responses are typically defined using:

- `.json` / `.json5` files for static responses.
- `.csv` files are available as JSON responses including records.
- Optionally, rule sets `.toml` offers way to define both text response and HTTP status code response.

## ~~How startup works~~

Below is v3 (todo)

```mermaid
graph
    subgraph Startup workflow
        direction TB
        A[config mode if apimock.toml exists] --> B[config-less mode if apimock-dyn-route dir exists]
        B --> C['always' mode : fixed response]
        C --> D[middleware validation if exists]
    end
```

### ~~How response works~~

Below is v3 (todo)

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
