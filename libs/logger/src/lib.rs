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
    use tracing::{debug, error, info, trace, warn};
    use tracing_subscriber::{
        prelude::__tracing_subscriber_SubscriberExt, registry,
        util::SubscriberInitExt,
    };

    use crate::LogToStdout;

    #[test]
    fn test() {
        registry().with(LogToStdout::init()).init();

        info!("AAA");
        error!("AAA");
        warn!("AAA");
        trace!("AAA");
        debug!("AAA");
    }
}
