use serde::Deserialize;

/// log
#[derive(Clone, Default, Deserialize)]
pub struct LogConfig {
    pub verbose: VerboseConfig,
}

impl std::fmt::Display for LogConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{}", self.verbose);
        Ok(())
    }
}

/// verbose logs
#[derive(Clone, Default, Deserialize)]
pub struct VerboseConfig {
    pub header: bool,
    pub body: bool,
}

impl std::fmt::Display for VerboseConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(
            f,
            "[log.verbose] header = {}, body = {}",
            if self.header { "Yes" } else { "No" },
            if self.body { "Yes" } else { "No" }
        );
        Ok(())
    }
}
