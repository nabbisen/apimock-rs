pub const CSV_RECORDS_DEFAULT_KEY: &str = "records";

const DEFAULT_ALLOWED_METHODS: &str = "GET, POST, PUT, DELETE, OPTIONS";
pub const DEFAULT_RESPONSE_HEADERS: &[(&str, &str)] = &[
    ("cache-control", "no-store"),
    ("access-control-allow-credentials", "true"),
    ("access-control-allow-headers", "*"),
    ("access-control-allow-methods", DEFAULT_ALLOWED_METHODS),
    ("x-content-type-options", "nosniff"),
    ("connection", "keep-alive"),
];
