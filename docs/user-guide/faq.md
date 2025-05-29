# FAQ (Frequently Asked Questions)

**Q: Can I match only by URL path ?**    
A: Yes. `url_path`, `headers` and `body` are options.

**Q: Can I switch server port from the default ?**    
A: Yes. Two ways: run with `-p` | `--port` argument followed by specific port number. Alternatively, define it in `[listener]` section in `apimock.toml`, the root configuration.

**Q: Why do directory paths like `/api` return a 404 error ?**    
A: Directory paths such as `/api` or `/api/v1` return a 404 status code **unless they contain a special index file (e.g., `index.json`, `.json5`, or `.csv`. Also `.html`)**. If one of these files is present, its content will be served instead.

**Q: Can I return binary data ?**    
A: Yes, binary data such as image, audio, video and archive is returned with their specific content-type.

**Q: How are rules loaded ?**    
A: At startup (via TOML deserialization).

**Q: Can I dynamically generate responses ?**    
A: Yes, partially supported with rhai script to determine response file due to request condition. However, static, file-based or rule-based responses are expected to fulfill most cases.
