# Rule-based Configuration

Routing rules are written in a `.toml` file and specify how to respond to specific requests.

### File: `apimock-rule-set.toml`

```toml
[[rules]]
[rules.when]
url_path = "/hello"

[rules.respond]
code = 200
text = "Hello, world!"
```

### `when` Section

Defines matching conditions:

```toml
[rules.when]
url_path = "/login"
headers = { "Content-Type" = "application/json" }
body = '{"user":"admin"}'
```

### `respond` Section

Defines mock response:

```toml
[rule.respond]
status = 401
headers = { "Content-Type" = "application/json" }
body = '{"error":"unauthorized"}'
```

You may define multiple `[[rules]]` entries.
