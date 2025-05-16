use serde::Deserialize;

/// log
#[derive(Clone, Default, Deserialize)]
pub struct LogConfig {
    pub verbose: VerboseConfig,
}

impl LogConfig {
    pub fn print(&self) {
        self.verbose.print();
    }
}

/// verbose logs
#[derive(Clone, Default, Deserialize)]
pub struct VerboseConfig {
    pub header: bool,
    pub body: bool,
}

impl VerboseConfig {
    pub fn print(&self) {
        log::info!(
            "[log.verbose] header = {}, body = {}",
            if self.header { "Yes" } else { "No" },
            if self.body { "Yes" } else { "No" }
        );
    }
}
