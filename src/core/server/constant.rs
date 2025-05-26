pub const CSV_RECORDS_DEFAULT_KEY: &str = "records";

pub const DEFAULT_RESPONSE_HEADERS: &[(&str, &str)] = &[
    ("cache-control", "no-store"),
    ("access-control-allow-credentials", "true"),
    ("access-control-allow-headers", "*"),
    (
        "access-control-allow-methods",
        "GET, POST, PUT, DELETE, OPTIONS",
    ),
    ("x-content-type-options", "nosniff"),
    ("connection", "keep-alive"),
    ("transfer-encoding", "chunked"),
];
