use std::{borrow::Cow, sync::OnceLock};

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
    includes_ansi_codes: bool,
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
                let args = record.args().to_string();
                let msg = if !self.includes_ansi_codes {
                    // omit ansi escape codes for console text color
                    strip_ansi_codes(args.as_ref())
                } else {
                    Cow::from(args)
                };
                // message with log level
                let msg_with_log_level = format!(
                    "[{}] {}",
                    record.level().to_string().chars().next().unwrap(),
                    msg
                );

                let tx = tx.clone();
                tokio::spawn(async move {
                    let _ = tx.send(msg_with_log_level).await;
                });
            }
        }
    }

    /// flush
    fn flush(&self) {}
}

/// init logger
pub fn init_logger(
    tx: Option<Sender<String>>,
    includes_ansi_codes: bool,
) -> Result<(), SetLoggerError> {
    let output = if let Some(tx) = tx {
        LogOutput::Sender(tx)
    } else {
        LogOutput::Stdout
    };

    LOGGER
        .set(AppLogger {
            output,
            includes_ansi_codes,
        })
        .ok();
    log::set_logger(LOGGER.get().unwrap())?;
    log::set_max_level(log::LevelFilter::Info);

    Ok(())
}
