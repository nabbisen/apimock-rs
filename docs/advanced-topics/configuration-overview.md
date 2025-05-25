# Configuration overview

```mermaid
classDiagram
    direction LR
    class Config {
        +Table listener
        +Table log.verbose
        +Table service
    }
    class ListenerConfig {
        +String ip_address
        +Integer port
    }
    class LogConfig.VerboseConfig {
        +Boolean header
        +Boolean body
    }
    class ServiceConfig {
        +Array~RuleSet~ rule_sets
        +Array~RuleSet~ middlewares
        +String fallback_respond_dir
    }

    Config --|> ListenerConfig : contains 1
    Config --|> LogConfig.VerboseConfig : contains 1
    Config --|> ServiceConfig : contains 1
```

Here's an overview of the rule data structure in a nested Markdown format:

- `apimock.toml`
    - `[listener]` (Table): Server listener.
        - `ip_address`
        - `port`
    - `[log]` (Table): Logger.
        - `verbose.header`: Verbose on request header.
        - `verbose.body`: Verbose on request body.
    - `[service]` (Table): App service
        - **`rule_sets`:** Rule-based routing. The detail is [here](rule-set-config-structure/rules/).
        - `middlewares`
        - **`fallback_respond_dir`:** File-based routing base. The default is `.`, your current directory.
