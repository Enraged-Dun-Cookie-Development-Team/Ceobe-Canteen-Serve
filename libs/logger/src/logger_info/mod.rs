mod location;
mod record_level;
use std::fmt::Arguments;

pub use self::{location::Location, record_level::RecordLevel, time::Time};

mod time;

pub struct LoggerInfo<'re, 'a> {
    pub time: Time,
    pub level: RecordLevel,
    pub location: Location<'re>,
    pub msg: &'re Arguments<'a>,
}
