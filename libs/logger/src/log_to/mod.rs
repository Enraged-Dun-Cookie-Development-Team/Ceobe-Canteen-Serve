use log::LevelFilter;

pub mod file;
pub mod stdout;

pub trait GetLogLevel {
    fn get_level(&self) -> LevelFilter { LevelFilter::Trace }

}
