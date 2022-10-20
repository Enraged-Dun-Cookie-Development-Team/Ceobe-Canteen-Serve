mod log_to;
pub use log_to::{
    file::{FileLoggerInfo, LogToFile},
    stdout::LogToStdout,
    GetLogLevel,
};

#[cfg(test)]
mod test {
    use crate::log_to::stdout::LogToStdout;

    #[test]
    fn test() {
        LogToStdout::new(log::LevelFilter::Trace).apply().unwrap();

        log::info!("AAA");
        log::error!("AAA");
        log::warn!("AAA");
        log::trace!("AAA");
        log::debug!("AAA");
    }
}
