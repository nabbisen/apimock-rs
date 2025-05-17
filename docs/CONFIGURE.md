# Configuration and options

## Configuration

`apimock.toml`

```toml
[listener]
address = "127.0.0.1"
port = 3001

[general]
dyn_data_dir = "apimock-dyn-route"
# always = "{ greetings: \"Hello, world.\" }"
response_wait = 100
# verbose = { header = true, body = true }

[url]
data_dir = "apimock-data"
data_dir_query_path = "@@"
path_prefix = "api/v1"

[url.headers]
cookie_1 = { key = "Set-Cookie", value = "a=b; c=d" }
redirect_1 = { key = "Location", value = "/api/v1/home" }

# required when `always` is not specified
[url.paths] # `path_prefix` works
"home" = "home.json"
# "some/path" = "api.json5"
# custom headers
"some/path/w/header" = { src = "home.json", headers = ["cookie_1"] }
# errors / redirects * `code` must be defined as **unsigned integer** (instead of String)
"error/401" = { code = 401 }
"error/api-403" = { code = 403 }
"redirect/302" = { code = 302, headers = ["redirect_1"] }

[url.paths_patterns."some/path/w/matcher"."a.b.c"]
"=1" = "api.json5"
"=0" = "home.json"
[url.paths_patterns."some/path/w/matcher"."d.2.e"]
"=x=" = "api.json5"
[url.paths_patterns."some/path/w/matcher"."f"]
"=" = "api.json5"

[url.raw_paths] # `path_prefix` doesn't work
"/" = { code = 200, text = "{\"hello\":\"world\"}" }
```

### Properties

#### `listener.address`

IP Address listened to by server.

**Default**: "127.0.0.1"

#### `listener.port`

Port listened to by server.

**Default**: 3001

#### `general.dyn_data_dir`

If set, URL path without statically defined path matched is converted to file path in this directory. Server tries to find it out as either `.json` or `.json5`. When found, server returns the content as JSON response.

**Default**: empty

It works even without config toml. It is config-less mode.

#### `general.response_wait`

Specify in milliseconds. If specified, server waits for the time before returning response on each request.

**Default**: 0

#### `general.verbose`

Activates verbose log at each request which server gets.

**Default**: header = false, body = false

#### `url.data_dir`

Data directory used as where to look up files when HTTP response is built.

**Default**: executable directory

#### `url.data_dir_query_path`

Data directory can be switched manually via HTTP request. Access to http://127.0.0.1/(`url.data_dir_query_path`) to get the current value. Access to http://127.0.0.1/(`url.data_dir_query_path`)/some/path to change it.

**Default**: "@@"

#### `url.path_prefix`

Static paths are dealt with as those who have the prefix. Convenient when your service has path prefix.

**Default**: empty

#### `url.headers`

HTTP headers such as `Authorizaton: xxx` on auth and `Location: xxx` on redirection.
You can reuse them and easily attach headers in `url.paths` by defining here.

**Default**: None

#### `url.paths`

The key, the left-hand side, is URL path. The right-hand one is response definition.
Response definition consists of five optional parts:

- `code` as HTTP responses code
- `headers` as HTTP headers keys defined in `url.headers`
- **`src`** as data source file relative path in `url.data_dir`
- `text` as direct body text instead of `src`
- `wait_more` as additional milliseconds to [`general.response_wait`](#generalresponse_wait)

For example:

```toml
"url_path" = { code = 200, headers = ["header_key_1"], src = "response_1.json", wait_more = 700 }
```

It is able to omit code and headers:

```toml
"url_path" = "response_1.json"
```

It means **`src`** only, and is far simpler. `code` and `headers` are dealt with as their default: 200 as OK and no custom headers.

Only when either `src` or `text` is defined, the response `Content-Type` is set as `application/json`.

**Default**: None

#### `url.paths_patterns`

Pattern-matching-like options are available, which enable to dynamically specify response JSON file due to request body parameter.

The format is:

```toml
[url.paths_patterns."{API_PATH}"."{JSONPATH}"]
"={CASE_VALUE}" = "{DATA_SRC}"
```

For example, with the definition below, you can return "special.json" when "/some/matcher/path" is accessed to and the request body is like "{\"key_a\": {\"key_b\": {\"key_c\": 1}}, ...}":

```toml
[url.paths_patterns."/some/matcher/path"."key_a.key_b.key_c"]
"=1" = "special.json"
```

Of course, the body may include another parameter unrelated to the query.

Remember:

- Enclose API path and JSONPath with `"`
- Start with `=` in writing pattern value

Array is also available with index number specified. For example, when the request body is "{\"key_d\": [{}, {}, {\"key_e\": \"x=\"}]}", how to point to it is: 

```toml
[url.paths_patterns."/some/matcher/path"."key_d.2.key_e"]
"=x=" = "special.json"
```

`2` in "key_d.**2**.key_e" is the list index.

**Default**: None

#### `url.raw_paths`

Not affected by `url.path_prefix`. Everything else is the same to `url.paths`.

**Default**: None

## Executable arguments

### `-c` / `--config`

Config file path.
default: `apimock.toml`

### `-p` / `--port`

Listener port to overwrite config.
default: see: [listener.port](#listenerport)

### `--middleware`

Middleware file path.
default: `apimock-middleware.rhai`

### `--init`

When passed, initialize app files.    
`./apimock.toml` and `./apimock-middleware.rhai` will be generated if missing.

## Middleware

### Pre-defined variables available in `.rhai`

- `url_path`: Request URL path.
- `body`: Request Body JSON value defined only when exists.

### Request routing

#### URL path

```js
// print(url_path); // debug
if url_path == "/middleware-test" { ... }
```

#### Body

```js
if is_def_var("body") {
    // print(body); // debug
    // matches on json value dealed with as map
    if body.middleware == "isHere" { ... }
    // alternatively, case matching is available. if guard may be combined with
    switch (url_path) {
        "/middleware-test/dummy" if body.middleware == "isHere" => { ... },
        _ => ()
    }
```

### Response handling

Specify JSON file path.

```js
// return to middleware caller:
return "some/path/response.json";
// alternative statements:
let ret = "some/path/response.json";
exit(ret);
```

## Notes

### After server started

There are some modifiable settings on running server:

- `.json` / `.json5` content of `src` in `paths`, `raw_paths`, and those in `dyn_data_dir`
- `data_dir` in `paths` and `paths_patterns`
