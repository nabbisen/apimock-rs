# Rule-based routing

## Defining Rules

Now, let's dive into `apimock-rule-set.toml` and define some powerful rules ! Each rule checks incoming requests against specific conditions (like URL path, headers, or body content) and, if they match, sends a predefined response.

Here are examples of how you can set up different types of rules in `apimock-rule-set.toml`:

### Example 1: Match with Request URL Path

```toml
[[rules]]
when.request.url_path = ""
respond = { file_path = "root.json" }
```

<!-- 
### Example x: Match with HTTP Method

```toml
# (in the future)
# [[rules]]
# when.request.method = "GET"
# respond = { code = 403 }
```
-->

### Example 2: Match with Headers

```toml
[[rules]]
[rules.when.request.headers]
Authorization = { value = "Bearer eyJhb(...).(...).(...)" }
[rules.respond]
file_path = "authorized.json"
```

### Example 2: Match with Headers

```toml
[[rules]]
[rules.when.request.body.json]
"a.b.c" = { value = "d" }
[rules.respond]
file_path = "response.json"
```
