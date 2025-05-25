# Technical referrence

## Architecture Overview

```plaintext
Incoming Request
      |
      v
  Execute Rhai Script (if configured) ? ---> YES ---> Serve File
      |
      v
  Match Rule (URL, Headers, Body) ? ---> YES ---> Return Response
      |
      v
  Match File Path? --------> YES ---> Serve File
      |
      v
  Not Found (404)
```

## Three Modes of Response

1. **File-based** (static response from the filesystem)

   * If `/api/data.json` exists, it's served directly.
   * Extensions like `.json5`, `.csv` are also supported.

2. **Rule-based** (config-driven conditional responses)

   * Match URL path, headers, or JSON body to return different responses.

3. **Script-based** (dynamic Rhai scripts, optional)

   * A script returns the response path based on logic in the script.

## Rule Matching Logic

```plaintext
prefix: /api
   |
   |-- when:
   |     url_path == "/hello"
   |
   |-- respond:
         file_path: "hello.json"
```

## Prioritization

* First matching file wins.
* Define rules in the order of priority.
* Relative paths are based on the main config file.

This layered approach allows simple to advanced mock responses with clarity and control.
