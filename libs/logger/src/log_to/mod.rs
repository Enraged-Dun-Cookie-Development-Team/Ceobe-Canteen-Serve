use tracing_subscriber::fmt::{format, time::FormatTime};

pub mod file;
pub mod stdout;

struct TimeFormat;

impl FormatTime for TimeFormat {
    fn format_time(&self, w: &mut format::Writer<'_>) -> std::fmt::Result {
        write!(
            w,
            "{time}",
            time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        )
    }
}