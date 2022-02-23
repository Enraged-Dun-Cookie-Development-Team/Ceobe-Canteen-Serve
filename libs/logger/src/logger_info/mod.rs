mod location;
mod record_level;
use std::fmt::Arguments;

pub use self::{time::Time, record_level::RecordLevel, location::Location};

mod time;

pub struct LoggerInfo<'re,'a>{
    pub time:Time,
    pub level:RecordLevel,
    pub location:Location<'re>,
    pub msg:& 're Arguments<'a>
}

