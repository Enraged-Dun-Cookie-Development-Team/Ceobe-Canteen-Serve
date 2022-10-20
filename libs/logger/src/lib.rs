mod error;
mod log_init;
mod log_to;
pub use error::Error;
pub use log_init::{GetLogLevel, LogInit};
pub use log_to::{
    file::{FileLoggerInfo, LogToFile},
    stdout::LogToStdout,
};

#[cfg(test)]
mod test {
    use crate::LogToStdout;

    #[test]
    fn test() {
        LogToStdout::new().apply().unwrap();

        log::info!("AAA");
        log::error!("AAA");
        log::warn!("AAA");
        log::trace!("AAA");
        log::debug!("AAA");
    }
}
