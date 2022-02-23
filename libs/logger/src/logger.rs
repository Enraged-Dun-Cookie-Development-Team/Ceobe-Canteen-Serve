use chrono::Local;

use crate::{
    config::Config,
    logger_adapter::LoggerAdapter,
    logger_info::{LoggerInfo, RecordLevel, Time},
};

pub struct Logger<A>(Config, A);

impl<A: LoggerAdapter> Logger<A> {
    pub(crate) fn new(cfg: Config, adapter: A) -> Self {
        Self(cfg, adapter)
    }
}

impl<A: LoggerAdapter> log::Log for Logger<A> {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !Self::enabled(&self, record.metadata()) {
            return;
        }
        let level = if self.0.enable_color {
            Into::<RecordLevel>::into(record.level())
        } else {
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

        self.1.do_log(info);
    }

    fn flush(&self) {
        self.1.flush()
    }
}
