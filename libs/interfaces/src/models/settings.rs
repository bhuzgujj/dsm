use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    str::FromStr,
};
use std::path::PathBuf;
use log::{trace, LevelFilter};
use serde::{Deserialize, Serialize};

use crate::paths::dsm_dir;

const SETTINGS_FILENAME: &'static str = "settings.toml";
const STORE: &'static str = "image_store";

#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    log_level: String,
    dataset_database: String,
    image_store: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            log_level: LevelFilter::Info.to_string(),
            dataset_database: dsm_dir().to_string_lossy().to_string(),
            image_store: dsm_dir().join(STORE).to_string_lossy().to_string(),
        }
    }
}

impl Settings {
    pub fn get_log_level(&self) -> LevelFilter {
        LevelFilter::from_str(self.log_level.as_str()).unwrap_or(LevelFilter::Warn)
    }

    pub fn set_log_level(&mut self, log_level: LevelFilter) {
        self.log_level = log_level.to_string();
    }

    pub fn get_image_store(&self) -> PathBuf {
        PathBuf::from_str(self.image_store.as_str())
            .expect(format!("Invalid dataset database location: {}", self.dataset_database).as_str())
    }
    
    pub fn get_dataset_database(&self) -> PathBuf {
        PathBuf::from_str(self.dataset_database.as_str())
            .expect(format!("Invalid dataset database location: {}", self.dataset_database).as_str())
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
        trace!("Saving settings \n\"\"\" Path: '{}'\n{}\n\"\"\"", settings_filename.display(), content);
        Ok(OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(settings_filename)?
            .write_all(content.as_bytes())?)
    }
}
