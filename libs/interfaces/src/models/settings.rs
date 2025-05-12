use log::{Level, LevelFilter};

pub struct Settings {
    log_level: Level,
}

impl Settings {
    pub fn get_log_level(&self) -> LevelFilter {
        self.log_level.to_level_filter()
    }
}
