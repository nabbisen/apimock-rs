# API Mock (apimock-rs)

## 🪄 Mock APIs easily — no setup stress, just JSON and go

If you’re building or testing APIs, this tool makes mocking painless. You don’t need to write any config files — just use folders and JSON. It’s super fast, efficient, and flexible when you need it to be.

    🎈 No config needed to get started

    🥷 Fast to launch, light on memory, out of your way

    🧩 Moreover, advanced matching and custom scripting supported

It’s rebuilt from the ground up in version 4. Designed to help developers of all levels.

![demo](https://github.com/apimokka/apimock-rs/blob/main/docs/.assets/demo.gif?raw=true)

```sh
# install
npm install -D apimock-rs
# and go
npx apimock
```

```sh
# just use folders and JSON
mkdir -p api/v1/
echo '{"hello": "world"}' > api/v1/hello.json
npx apimock

# response
curl http://localhost:3001/api/v1/hello
# --> {"hello":"world"}
```

```sh
# also, there's room to tweak things later
npx apimock --init
```

For more details, check out [the docs](https://apimokka.github.io/apimock-rs/).

---

## Open-source, with care

This project is lovingly built and maintained by volunteers.  
We hope it helps streamline your API development.  
Please understand that the project has its own direction — while we welcome feedback, it might not fit every edge case 🌱

## Acknowledgements

Depends on [tokio](https://github.com/tokio-rs/tokio) / [hyper](https://hyper.rs/) / [toml](https://github.com/toml-rs/toml) / [serde](https://serde.rs/) / [serde_json](https://github.com/serde-rs/json) / [json5](https://github.com/callum-oakley/json5-rs) / [console](https://github.com/console-rs/console) / [rhai](https://github.com/rhaiscript/rhai). In addition, [mdbook](https://github.com/rust-lang/mdBook) (as to workflows).
