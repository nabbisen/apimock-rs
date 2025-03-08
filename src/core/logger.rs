use std::sync::OnceLock;

use console::strip_ansi_codes;
use log::{Level, Metadata, Record, SetLoggerError};
use tokio::sync::mpsc::Sender;

/// logger
static LOGGER: OnceLock<AppLogger> = OnceLock::new();

/// log output
#[derive(Clone)]
enum LogOutput {
    Stdout,
    Sender(Sender<String>),
}

/// app logger
#[derive(Clone)]
struct AppLogger {
    output: LogOutput,
}

impl log::Log for AppLogger {
    /// check if it is enabled
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    /// log print out
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
                    // omit ansi escape codes for console text color
                    strip_ansi_codes(record.args().to_string().as_ref())
                );

                let tx = tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(msg).await;
                });
            }
        }
    }

    /// flush
    fn flush(&self) {}
}

/// init logger
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
