use confy::ConfyError;
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use tracing_core::LevelFilter;

pub fn init_config() -> Result<Config, ConfyError> {
    confy::load("koudaisai-portal", None)
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub logging: Logging,
    pub web: Web,
    pub db: Db
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Logging {
    pub log_level: LogLevel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LogLevel {
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

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Web {
    pub server: Server,
    pub auth: Auth
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
}
impl Default for Server {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".parse().unwrap(),
            port: 8080,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub password_salt: String,
    pub activation_salt: String,
    pub stretch_cost: u8
}

impl Default for Auth {
    fn default() -> Self {
        let mut rng = rand::rng();
        Self {
            password_salt: Alphanumeric.sample_string(&mut rng, 16),
            activation_salt: Alphanumeric.sample_string(&mut rng, 16),
            stretch_cost: 13,
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Db {
    pub address: String
}