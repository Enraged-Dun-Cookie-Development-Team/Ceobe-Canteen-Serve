use std::{
    fs::{File, OpenOptions},
    io::{self, BufWriter, Write},
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
use parking_lot::{Mutex, MutexGuard};
struct LoggerFile(Mutex<BufWriter<File>>);

impl LoggerFile {
    fn with_file(path: &str) -> io::Result<Self> {
        Ok(Self(Mutex::new(BufWriter::new(
            OpenOptions::new().create(true).append(true).open(path)?,
        ))))
    }
}

impl<'writer> MakeWriter<'writer> for LoggerFile {
    type Writer = BufWriterGuard<'writer>;

    fn make_writer(&'writer self) -> Self::Writer {
        BufWriterGuard(self.0.lock())
    }
}

struct BufWriterGuard<'writer>(MutexGuard<'writer, BufWriter<File>>);

impl<'writer> Write for BufWriterGuard<'writer> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.0.write(buf) }

    fn flush(&mut self) -> io::Result<()> { self.0.flush() }

    fn write_vectored(
        &mut self, bufs: &[io::IoSlice<'_>],
    ) -> io::Result<usize> {
        self.0.write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> io::Result<()> {
        self.0.write_fmt(fmt)
    }
}
