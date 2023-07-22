use std::io::{stdout, Stdout, StdoutLock};

use tap::Pipe;
use tracing::Subscriber;
use tracing_subscriber::{
    fmt::{format, MakeWriter},
    registry::LookupSpan,
    Layer,
};

use super::TimeFormat;

pub struct LogToStdout;

impl LogToStdout {
    pub fn init<S>() -> impl Layer<S>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        tracing_subscriber::fmt::layer()
            .event_format(
                format()
                    .pretty()
                    .with_ansi(true)
                    .with_level(true)
                    .with_timer(TimeFormat),
            )
            .pipe(|layer| layer.with_writer(BufferStdout::default()))
    }
}

pub struct BufferStdout(Stdout);

impl<'writer> MakeWriter<'writer> for BufferStdout {
    type Writer = StdoutLock<'writer>;

    fn make_writer(&'writer self) -> Self::Writer {
        self.0.lock()
    }
}

impl Default for BufferStdout {
    fn default() -> Self {
        Self(stdout())
    }
}
