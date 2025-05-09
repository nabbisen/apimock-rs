pub const APP_NAME: &str = "API mock";

pub mod args {
    pub const CONFIG_FILEPATH_OPTION_NAMES: [&str; 2] = ["-c", "--config"];
    pub const CONFIG_LISTENER_PORT_OPTION_NAMES: [&str; 2] = ["-p", "--port"];
    pub const MIDDLEWARE_FILEPATH_OPTION_NAMES: [&str; 1] = ["--middleware"];

    pub const DEFAULT_CONFIG_FILENAME: &str = "apimock.toml";
    pub const DEFAULT_MIDDLEWARE_FILEPATH: &str = "./middleware.rhai";
}

pub mod config {
    pub const CONFIG_SECTION_LISTENER: &str = "listener";
    pub const CONFIG_SECTION_GENERAL: &str = "general";
    pub const CONFIG_SECTION_URL: &str = "url";
    pub const CONFIG_SECTION_URL_HEADERS: &str = "headers";
    pub const CONFIG_SECTION_URL_PATHS: &str = "paths";
    pub const CONFIG_SECTION_URL_PATHS_JSONPATH_PATTERNS: &str = "paths_patterns";
    pub const CONFIG_SECTION_URL_RAW_PATH: &str = "raw_paths";

    pub const DEFAULT_LISTENER_IP_ADDRESS: &str = "127.0.0.1";
    pub const DEFAULT_LISTENER_PORT: u16 = 3001;

    pub const DEFAULT_DYN_DATA_DIR: &str = "apimock-dyn-data";

    pub const ALWAYS_DEFAULT_MESSAGES: &str = "Hello, world from API Mock.\n(Responses can be modified with either config toml file or dynamic data directory.)";
}
