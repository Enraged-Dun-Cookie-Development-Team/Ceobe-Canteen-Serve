use std::{
    io::{stdout, BufWriter, Stdout},
    sync::Mutex,
};

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
                    .with_timer(TimeFormat)
                    .with_source_location(true)
                    .with_thread_ids(true)
                    .with_thread_names(true),
            )
            .pipe(|layer| {
                #[cfg(not(debug_assertions))]
                {
                    layer.with_writer(BufferStdout::default())
                }
                #[cfg(debug_assertions)]
                {
                    layer
                }
            })
    }
}

pub struct BufferStdout(Mutex<BufWriter<Stdout>>);

impl<'writer> MakeWriter<'writer> for BufferStdout {
    type Writer = <Mutex<BufWriter<Stdout>> as MakeWriter<'writer>>::Writer;

    fn make_writer(&'writer self) -> Self::Writer { self.0.make_writer() }
}

impl Default for BufferStdout {
    fn default() -> Self {
        Self(Mutex::new(BufWriter::with_capacity(
            32 * 1024 * 1024,
            stdout(),
        )))
    }
}
