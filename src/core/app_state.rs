use super::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub middleware: Option<String>,
}
