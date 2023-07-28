use tracing::{level_filters::LevelFilter, Subscriber};
use tracing_subscriber::{
    layer::Layered, prelude::__tracing_subscriber_SubscriberExt,
    registry::LookupSpan, util::SubscriberInitExt, EnvFilter, Layer,
    Registry,
};

use crate::{LogToFile, LogToStdout};

pub struct LogInit<S>(S);
pub trait GetLogLevel {
    fn get_level(&self) -> LevelFilter { LevelFilter::TRACE }
}

impl LogInit<Layered<EnvFilter, Registry>> {
    pub fn new<C: GetLogLevel>(cfg: &C) -> Self {
        Self(
            tracing_subscriber::registry().with(
                EnvFilter::builder()
                    .with_default_directive(cfg.get_level().into())
                    .parse_lossy(""),
            ),
        )
    }
}
impl<S> LogInit<S>
where
    S: Subscriber,
    for<'a> S: LookupSpan<'a>,
{
    pub fn log_to_file<C: crate::FileLoggerInfo>(
        self, cfg: &C,
    ) -> Result<LogInit<Layered<impl Layer<S>, S>>, crate::error::Error> {
        Ok(LogInit(self.0.with(LogToFile::init(cfg)?)))
    }

    pub fn log_to_stdout(self) -> LogInit<Layered<impl Layer<S>, S>> {
        LogInit(self.0.with(LogToStdout::init()))
    }

    pub fn apply(self) -> Result<(), crate::error::Error>
    where
        S: SubscriberInitExt,
    {
        self.0.try_init()?;
        Ok(())
    }
}
