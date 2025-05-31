# Response headers

## Default response headers

### For connection stability

| Header | Value |
| --- | --- |
| `Date` | Response date |
| `Content-Length` | Calculated from content body |
| `Connection` | `keep-alive` |
| `Cache-Control` | `no-store` |

### For security

| Header | Value |
| --- | --- |
| `x-content-type-options` | `nosniff` |
