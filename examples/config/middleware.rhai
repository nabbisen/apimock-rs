//! pre-defined variables are available:
//! - uri_path: request uri path
//! - body: request body json value defined only when exists

// case `cargo run`:
// let returned_json_filepath = "examples/config/apimock-data/middleware-test.json";
// case `cargo test`:
let returned_json_filepath = "apimock-data/middleware-test.json";

// print(uri_path);
if uri_path == "/middleware-test" {
    return returned_json_filepath;
}

if is_def_var("body") {
    // print(body);
    switch (uri_path) {
        "/middleware-test/dummy" if body.middleware == "isHere" => {
            // exit() is useful when run in fn (here, equivalent to return statement)
            exit(returned_json_filepath);
        },
        _ => ()
    }
}

return;
