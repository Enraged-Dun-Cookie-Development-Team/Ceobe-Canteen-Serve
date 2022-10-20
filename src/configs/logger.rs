use log::LevelFilter;
use logger::{GetLogLevel, LogInit};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoggerConfig {
    #[serde(default)]
    log_to: LogTo,
    level: LogLevel,
}

impl LoggerConfig {
    pub fn init_log(&self) -> Result<(), logger::Error> {
        let mut init = LogInit::new(self);
        if self.log_to.file.is_some() {
            init = init.log_to_file(&self.log_to)?
        }
        if self.log_to.stdout {
            init = init.log_to_stdout();
        }

        init.apply()
    }
}

#[derive(Debug, Deserialize)]
pub struct LogTo {
    #[serde(default)]
    file: Option<String>,
    #[serde(default = "default_enable")]
    stdout: bool,
}

impl Default for LogTo {
    fn default() -> Self {
        Self {
            file: None,
            stdout: true,
        }
    }
}

fn default_enable() -> bool { true }

#[derive(Debug, Deserialize, Clone, Default, Copy)]
pub enum LogLevel {
    #[serde(alias = "off")]
    Off,
    #[serde(alias = "error")]
    Error,
    #[serde(alias = "warn")]
    Warn,
    #[serde(alias = "info")]
    #[default]
    Info,
    #[serde(alias = "debug")]
    Debug,
    #[serde(alias = "trace")]
    Trace,
}

impl<'l> From<&'l LogLevel> for LevelFilter {
    fn from(val: &'l LogLevel) -> Self {
        match val {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}
impl GetLogLevel for LoggerConfig {
    fn get_level(&self) -> log::LevelFilter { (&self.level).into() }
}

impl logger::FileLoggerInfo for LogTo {
    fn log_file(&self) -> &str { self.file.as_deref().unwrap_or("") }
}
