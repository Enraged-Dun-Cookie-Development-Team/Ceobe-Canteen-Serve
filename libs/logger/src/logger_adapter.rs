use std::io::Stdout;

use crate::logger_info::LoggerInfo;

pub trait LoggerAdapter: Sync + Send {
    fn do_log<'a, 'b>(&self, info: LoggerInfo<'a, 'b>);
    fn flush(&self);
}

impl LoggerAdapter for Stdout {
    fn do_log<'a, 'b>(&self, info: LoggerInfo<'a, 'b>) {
        println!(
            "{} | {:<16} - {} => {}",
            // time
            info.time,
            // level
            info.level,
            // local
            info.location,
            // msg
            info.msg
        )
    }

    fn flush(&self) {}
}
