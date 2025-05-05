print(uri_path);

if uri_path == "/middleware-test" {
    // case `cargo run`:
    // return "examples/config/apimock-data/middleware-test.json"
    // case `cargo test`:
    return "apimock-data/middleware-test.json"
}

return;
