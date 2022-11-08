use std::{
    fs::{File, OpenOptions},
    io::{self, BufWriter},
};

use tracing::Subscriber;
use tracing_subscriber::{
    fmt::{format, MakeWriter},
    registry::LookupSpan,
    Layer,
};

use super::TimeFormat;

pub struct LogToFile;

impl LogToFile {
    pub fn init<S>(cfg: &impl FileLoggerInfo) -> io::Result<impl Layer<S>>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let layer = tracing_subscriber::fmt::layer()
            .event_format(
                format()
                    .pretty()
                    .with_ansi(false)
                    .with_level(true)
                    .with_timer(TimeFormat)
                    .with_source_location(true)
                    .with_thread_ids(true)
                    .with_thread_names(true),
            )
            .with_writer(LoggerFile::with_file(cfg.log_file())?);

        Ok(layer)
    }
}

pub trait FileLoggerInfo {
    fn log_file(&self) -> &str;
}

struct LoggerFile(File);

impl LoggerFile {
    fn with_file(path: &str) -> io::Result<Self> {
        Ok(Self(
            OpenOptions::new().create(true).append(true).open(path)?,
        ))
    }
}

impl<'writer> MakeWriter<'writer> for LoggerFile {
    type Writer = BufWriter<&'writer File>;

    fn make_writer(&'writer self) -> Self::Writer {
        BufWriter::with_capacity(1024 * 8, self.0.make_writer())
    }
}
