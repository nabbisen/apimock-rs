use serde::Deserialize;

/// log
#[derive(Clone, Default, Deserialize)]
pub struct LogConfig {
    pub verbose: VerboseConfig,
}

/// verbose logs
#[derive(Clone, Default, Deserialize)]
pub struct VerboseConfig {
    pub header: bool,
    pub body: bool,
}
