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
const STORE: &'static str = "datasets-store";
const LEDGER_DIRECTORY: &'static str = "files-ledger";

#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    log_level: String,
    ledger_path: String,
    store_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            log_level: LevelFilter::Info.to_string(),
            ledger_path: dsm_dir().join(LEDGER_DIRECTORY).to_string_lossy().to_string(),
            store_path: dsm_dir().join(STORE).to_string_lossy().to_string(),
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

    pub fn get_store_path(&self) -> PathBuf {
        PathBuf::from_str(self.store_path.as_str())
            .expect(format!("Invalid dataset database location: {}", self.ledger_path).as_str())
    }
    
    pub fn get_ledger_path(&self) -> PathBuf {
        PathBuf::from_str(self.ledger_path.as_str())
            .expect(format!("Invalid dataset database location: {}", self.ledger_path).as_str())
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
