# Server response

## Flow

```
get request
dump request by logger
if middlewares enabled
  for middleware
    run rhai
    if file path is returned
      return response
if rule sets are defined
  for rule set
    run `is_match()`
      for rule
        run `is_match()`
        if matched entry is found in rule
          return response
walk from `fallback_respond_dir`:    
for [empty (no ext checker), preset exts]
  if file bound to the file path is found
    return the file content
%% note: process never aborts as long as possible. when invalid request or server status, return client or server error
```
