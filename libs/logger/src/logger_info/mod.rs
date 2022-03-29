mod location;
mod record_level;
use std::fmt::Arguments;

use chrono::Local;

pub use self::{location::Location, record_level::RecordLevel, time::Time};
use crate::LoggerConfig;

mod time;

#[derive(Debug, Clone)]
pub struct LoggerInfo<'re, 'a> {
    pub time: Time,
    pub level: RecordLevel,
    pub location: Location<'re>,
    pub msg: &'re Arguments<'a>,
}

impl<'re, 'a> LoggerInfo<'re, 'a> {
    pub fn from_record(
        record: &'re log::Record<'a>, cfg: &LoggerConfig,
    ) -> Self {
        let level = if cfg.enable_color {
            Into::<RecordLevel>::into(record.level())
        }
        else {
            RecordLevel::no_color(record.level())
        };

        let info = LoggerInfo {
            time: Time::from_time(Local::now()),
            level,
            location: crate::logger_info::Location::new(
                record.module_path().unwrap_or("Unknown"),
                record.file().unwrap_or("Unknown"),
                record.line().unwrap_or_default(),
            ),
            msg: record.args(),
        };

        info
    }
}
