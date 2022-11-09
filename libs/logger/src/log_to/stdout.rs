use tracing::Subscriber;
use tracing_subscriber::{fmt::format, registry::LookupSpan, Layer};

use super::TimeFormat;

pub struct LogToStdout;

impl LogToStdout {
    pub fn init<S>() -> impl Layer<S>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        tracing_subscriber::fmt::layer().event_format(
            format()
                .with_ansi(true)
                .with_level(true)
                .with_timer(TimeFormat)
                .with_source_location(true)
                .with_thread_ids(true)
                .with_thread_names(true),
        )
    }
}
