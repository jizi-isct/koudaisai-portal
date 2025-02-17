use confy::ConfyError;
use serde::{Deserialize, Serialize};
use tracing_core::LevelFilter;

pub fn init_config() -> Result<Config, ConfyError> {
    confy::load("koudaisai-portal", None)
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub logging: Logging,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Logging {
    pub log_level: LogLevel,
}

#[derive(Serialize, Deserialize)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

impl LogLevel {
    pub fn to_level_filter(&self) -> LevelFilter {
        match self {
            LogLevel::Trace => LevelFilter::TRACE,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}