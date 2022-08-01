use std::{ops::Deref, time::Duration};

use crate::{
    config::Config, logger_adapter::LoggerAdapter, logger_info::LoggerInfo,
};

pub struct Logger<A>(Config, A);

impl<A: LoggerAdapter> Logger<A> {
    pub(crate) fn new(cfg: Config, adapter: A) -> Self { Self(cfg, adapter) }
}

impl<A: LoggerAdapter> log::Log for Logger<A> {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.0.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !Self::enabled(self, record.metadata()) {
            return;
        }

        let info = LoggerInfo::from_record(record, &self.0);

        self.1.do_log(info);
    }

    fn flush(&self) { self.1.flush() }
}

pub(crate) fn panic_hook(panic_info: &std::panic::PanicInfo) {
    let (file, line) = panic_info
        .location()
        .map(|l| (l.file(), l.column()))
        .unwrap_or(("<unknown>", 0));

    let cause = panic_info
        .payload()
        .downcast_ref::<String>()
        .map(Deref::deref)
        .or_else(|| panic_info.payload().downcast_ref::<&str>().copied())
        .unwrap_or("<cause unknown>");

    log::error!("工口发生 [{},{}] 原因 : {}", file, line, cause);
    std::thread::sleep(Duration::from_millis(200));
}
