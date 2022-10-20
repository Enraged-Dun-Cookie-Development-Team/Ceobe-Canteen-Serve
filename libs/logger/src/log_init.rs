use fern::Dispatch;
use log::LevelFilter;

use crate::{LogToFile, LogToStdout};

pub struct LogInit(Dispatch);
pub trait GetLogLevel {
    fn get_level(&self) -> LevelFilter { LevelFilter::Trace }
}

impl LogInit {
    pub fn new<C: GetLogLevel>(cfg: &C) -> Self {
        Self(Dispatch::new().level(cfg.get_level()))
    }

    pub fn log_to_file<C: crate::FileLoggerInfo>(
        self, cfg: &C,
    ) -> Result<Self, crate::error::Error> {
        Ok(Self(self.0.chain(LogToFile::init(cfg)?)))
    }

    pub fn log_to_stdout(self) -> Self {
        Self(self.0.chain(LogToStdout::init()))
    }

    pub fn apply(self) -> Result<(), crate::error::Error> {
        self.0.apply()?;
        Ok(())
    }
}
