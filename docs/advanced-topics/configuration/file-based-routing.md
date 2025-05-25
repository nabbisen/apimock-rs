# File-based routing

```
(root)/
└── api/
    └── v1/
        ├── status.json
        ├── user/
        │   ├── 1.json
        │   └── 2.json5
        └── users.csv
```

- A request to `/api/v1/status` returns `status.json`.
- `/api/v1/user/1` returns the content of `1.json`.
- `/api/v1/user/2` returns the content of `2.json5`. `.json5` is equivalent to `.json` to the server.
- `/api/v1/users` returns the content of `users.csv` as list.




`fallback-respond-dir`








## Tutorials

### Create a Complete Mock API

1. Create a new file `rules.toml`
2. Add rules for `/login`, `/logout`, `/profile`
3. Run server and use curl/Postman to test.

```toml
[[rules]]
[rules.when]
url_path = "/login"

[rules.respond]
status = 200
text = "{\"token\":\"abc123\"}"
```

### Test a Frontend App

1. Run `apimock` alongside your frontend dev server.
2. Configure frontend API base to `http://localhost:3001`
3. Mock endpoints as needed to simulate API behavior.

