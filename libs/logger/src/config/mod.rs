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

impl Config {
    pub fn set_filter(mut self, filter: LevelFilter) -> Self {
        self.level_filter = filter;
        self
    }
    pub fn enable_color(mut self) -> Self {
        self.enable_color = true;
        self
    }
    pub fn disable_color(mut self) -> Self {
        self.enable_color = false;
        self
    }
}
