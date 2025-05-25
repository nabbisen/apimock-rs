# FAQ (Frequently Asked Questions)

**Q: Can I match only by URL path ?**
A: Yes. `url_path`, `headers` and `body` are options.

**Q: Can I switch server port from the default ?**
A: Yes. Two ways: run with `-p` | `--port` argument followed by specific port number. Alternatively, define it in `[listener]` section in `apimock.toml`, the root configuration.

**Q: Can I return binary data ?**
A: Yes, binary data such as image, audio, video and archive is returned with their specific content-type.

**Q: How are rules loaded ?**
A: At startup (via TOML deserialization).

**Q: Can I dynamically generate responses ?**
A: Yes, partially supported with rhai script to determine response file due to request condition. However, static, file-based or rule-based responses are expected to fulfill most cases.
