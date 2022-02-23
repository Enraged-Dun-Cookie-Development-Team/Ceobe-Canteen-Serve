mod config;
mod logger;
mod logger_adapter;
mod logger_info;

use std::io::stdout;

pub use config::Config;

pub use logger::Logger;
pub use logger_adapter::LoggerAdapter;

pub fn init_std(config: Config) -> Result<(), log::SetLoggerError> {
    init(config, stdout())
}

pub fn init<A: LoggerAdapter + Sync + Send + 'static>(
    config: Config,
    adapter: A,
) -> Result<(), log::SetLoggerError> {
    let filter = config.level_filter.clone();
    let logger = Logger::new(config, adapter);

    log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(filter))
}
