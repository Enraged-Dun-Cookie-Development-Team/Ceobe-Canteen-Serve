use std::io;

use log::LevelFilter;

pub struct LogToFile;

impl LogToFile {
    pub fn new(
        cfg: &impl FileLoggerInfo, filter: LevelFilter,
    ) -> io::Result<fern::Dispatch> {
        Ok(fern::Dispatch::new()
        .level(filter)
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{time} | {level} - ({target})[{file}:{line}] => \
                     {message}",
                    time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    level = record.level(),
                    target = record.target(),
                    file = record.file().unwrap_or("<unknown>"),
                    line = record.line().unwrap_or(0)
                ))
            })
            .chain(fern::log_file(cfg.log_file())?))
    }
}

pub trait FileLoggerInfo {
    fn log_file(&self) -> &str;
}
