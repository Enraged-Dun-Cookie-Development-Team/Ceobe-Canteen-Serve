mod config;
mod logger;
mod logger_adapter;
mod logger_info;

use std::io::stdout;

use config::Config;

pub use logger::Logger;

pub fn init(config: Config) -> Result<(), log::SetLoggerError> {
    let filter = config.level_filter.clone();
    let logger = Logger::new(config, stdout());

    log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(filter))
}
