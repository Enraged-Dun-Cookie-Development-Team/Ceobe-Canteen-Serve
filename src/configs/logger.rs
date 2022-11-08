use logger::{GetLogLevel, LogInit};
use serde::Deserialize;
use tracing::level_filters::LevelFilter;

#[derive(Debug, Deserialize)]
pub struct LoggerConfig {
    #[serde(default, flatten)]
    log_to: LogTo,
    level: LogLevel,
}

impl LoggerConfig {
    pub fn init_log(&self) -> Result<(), logger::Error> {
        'logger: {
            let init = LogInit::new(self);
            if self.log_to.to_file.is_some() && self.log_to.to_stdout {
                init.log_to_file(&self.log_to)?.log_to_stdout().apply()?;
                break 'logger;
            }

            if self.log_to.to_file.is_some() {
                init.log_to_file(&self.log_to)?.apply()?;
                break 'logger;
            }

            if self.log_to.to_stdout {
                init.log_to_stdout().apply()?;
                break 'logger;
            }
        };

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct LogTo {
    #[serde(default)]
    to_file: Option<String>,
    #[serde(default = "default_enable")]
    to_stdout: bool,
}
impl Default for LogTo {
    fn default() -> Self {
        Self {
            to_file: None,
            to_stdout: true,
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
            LogLevel::Off => LevelFilter::OFF,
            LogLevel::Error => LevelFilter::ERROR,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Trace => LevelFilter::TRACE,
        }
    }
}
impl GetLogLevel for LoggerConfig {
    fn get_level(&self) -> LevelFilter { (&self.level).into() }
}

impl logger::FileLoggerInfo for LogTo {
    fn log_file(&self) -> &str {
        self.to_file.as_deref().unwrap_or("log_out.log")
    }
}
