use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    str::FromStr,
};

use log::LevelFilter;
use serde::{Deserialize, Serialize};

use crate::paths::dsm_dir;

const SETTINGS_FILENAME: &'static str = "settings.toml";

#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    log_level: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            log_level: LevelFilter::Info.to_string(),
        }
    }
}

impl Settings {
    pub fn get_log_level(&self) -> LevelFilter {
        LevelFilter::from_str(self.log_level.as_str()).unwrap_or(LevelFilter::Warn)
    }

    pub fn load() -> Self {
        let settings_filename = dsm_dir().join(SETTINGS_FILENAME);
        if let Ok(content) = read_to_string(settings_filename) {
            toml::from_str(content.as_str()).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let settings_filename = dsm_dir().join(SETTINGS_FILENAME);
        let content = toml::to_string(self)?;
        Ok(OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(settings_filename)?
            .write_all(content.as_bytes())?)
    }
}
