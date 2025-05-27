# CORS

## Default

const DEFAULT_ALLOWED_METHODS: &str = "GET, POST, PUT, DELETE, OPTIONS";

pub const DEFAULT_RESPONSE_HEADERS: &[(&str, &str)] = &[
    ("access-control-allow-credentials", "true"),
    ("access-control-allow-headers", "*"),
    ("access-control-allow-methods", DEFAULT_ALLOWED_METHODS),
    ("access-control-max-age", "86400"),
    ("cache-control", "no-store"),
    ("connection", "keep-alive"),
    ("x-content-type-options", "nosniff"),
];

### special headers

- content-length : body hint len

- access-control-allow-origin and vary
    if successful to get origin from request
      (origin), "Origin"
    else
      "*", "*"

## OPTIONS method request

- default headers

and

- status 204 NO_CONTENT
- content-length 0

return quick response
