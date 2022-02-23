use log::LevelFilter;

pub struct Config {
    pub level_filter: LevelFilter,
    pub enable_color: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            level_filter: LevelFilter::Trace,
            enable_color: true,
        }
    }
}
