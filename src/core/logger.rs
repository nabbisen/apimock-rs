use std::sync::OnceLock;

use log::{Level, Metadata, Record, SetLoggerError};
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
enum LogOutput {
    Stdout,
    Sender(Sender<String>),
}

#[derive(Clone)]
struct AppLogger {
    output: LogOutput,
}

impl log::Log for AppLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        match &self.output {
            // default feature
            LogOutput::Stdout => {
                println!("{}", record.args());
            }
            // spawn feature
            LogOutput::Sender(tx) => {
                // message with log level
                let msg = format!(
                    "[{}] {}",
                    record.level().to_string().chars().next().unwrap(),
                    record.args()
                );

                let tx = tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(msg).await;
                });
            }
        }
    }

    fn flush(&self) {}
}

static LOGGER: OnceLock<AppLogger> = OnceLock::new();

pub fn init_logger(tx: Option<Sender<String>>) -> Result<(), SetLoggerError> {
    let output = if let Some(tx) = tx {
        LogOutput::Sender(tx)
    } else {
        LogOutput::Stdout
    };
    LOGGER.set(AppLogger { output }).ok();
    log::set_logger(LOGGER.get().unwrap())?;
    log::set_max_level(log::LevelFilter::Info);
    Ok(())
}
