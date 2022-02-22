mod logger;

use log::LevelFilter;
pub use logger::Logger;

pub fn init(level: LevelFilter) -> Result<(), log::SetLoggerError> {
    let logger = Logger::new();

    log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(level.into()))
}
