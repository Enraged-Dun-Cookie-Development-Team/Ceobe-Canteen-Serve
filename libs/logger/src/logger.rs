use chrono::{Datelike, Timelike};
use log::Level;

pub struct Logger;

impl Logger {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !Self::enabled(&self, record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Error => yansi::Color::Red,
            Level::Warn => yansi::Color::Yellow,
            Level::Info => yansi::Color::Green,
            Level::Debug => yansi::Color::Magenta,
            Level::Trace => yansi::Color::Blue,
        };
        let time = chrono::Utc::now();

        
        println!(
            "{}-{}-{} {}:{}:{} | [{}] - {}:{} => {}",
            // time
            time.year(),
            time.month(),
            time.day(),
            time.hour(),
            time.minute(),
            time.second(),
            // level
            color.paint(record.level()),
            // local
            record.file().unwrap_or("-"),
            record.line().unwrap_or(0),
            // msg
            record.args()
        )
    }

    fn flush(&self) {}
}
