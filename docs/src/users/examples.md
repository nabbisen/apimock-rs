# Examples

## Advanced Examples

### Example 1: Match with Headers

```toml
[[rule]]
[rule.when]
method = "GET"
path = "/api/check"
headers = { "X-Request-Id" = "abc123" }

[rule.respond]
status = 200
body = "Matched with header"
```

### Example 2: Static File Response

```toml
[[rule]]
[rule.when]
path = "/data"

[rule.respond]
status = 200
body = "@./test/fixtures/data.json"
```
