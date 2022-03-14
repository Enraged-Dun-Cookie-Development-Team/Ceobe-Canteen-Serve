use std::fs::OpenOptions;
use std::{io::Write, path::Path, sync::Mutex};

use log::LevelFilter;
use logger::{logger_info::LoggerInfo, LoggerAdapter};
use serde::Deserialize;

pub struct FileLogger(std::sync::Mutex<std::fs::File>);

impl LoggerAdapter for FileLogger {
    fn do_log<'a, 'b>(&self, info: LoggerInfo<'a, 'b>) {
        let _r = self
            .0
            .lock()
            .map(|mut f| {
                writeln!(
                    f,
                    "{} | {:<16} - {} => {}",
                    info.time, info.level, info.location, info.msg
                )
            })
            .ok();
    }

    fn flush(&self) {
        let _r = self
            .0
            .lock()
            .map(|mut f| {
                f.flush().ok();
            })
            .ok();
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "logger_target")]
pub enum LoggerConfig {
    #[serde(rename = "file")]
    File { to_file: String, level: LogLevel },
    #[serde(rename = "stdout")]
    Std {
        #[serde(default = "default_color")]
        enable_color: bool,
        level: LogLevel,
    },
}

fn default_color() -> bool {
    true
}

impl LoggerConfig {
    pub fn register_logger(&self) {
        match self {
            LoggerConfig::File { to_file, level } => {
                let path = Path::new(to_file);
                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(path)
                    .expect("无法打开日志文件");

                let adapter = FileLogger(Mutex::new(file));
                let conf = logger::LoggerConfig {
                    level_filter: level.into(),
                    enable_color: false,
                };
                logger::init(conf, adapter).expect("无法启动日志")
            }
            LoggerConfig::Std {
                enable_color,
                level,
            } => {
                logger::init_std(logger::LoggerConfig {
                    level_filter: level.into(),
                    enable_color: *enable_color,
                })
                .expect("Can not Start Logger");
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum LogLevel {
    #[serde(alias = "off")]
    Off,
    #[serde(alias = "error")]
    Error,
    #[serde(alias = "warn")]
    Warn,
    #[serde(alias = "info")]
    Info,
    #[serde(alias = "debug")]
    Debug,
    #[serde(alias = "trace")]
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl<'l> Into<LevelFilter> for &'l LogLevel {
    fn into(self) -> LevelFilter {
        match self {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}
