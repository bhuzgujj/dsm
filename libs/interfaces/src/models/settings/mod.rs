pub mod actions;
pub mod remotes;

use crate::log_err;
use crate::paths::{dsm_dir, write_to_file};
use log::{trace, LevelFilter};
use remotes::Remote;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs::read_to_string, str::FromStr};

const SETTINGS_FILENAME: &str = "settings.toml";
const STORE: &str = "datasets-store";
const LEDGER_DIRECTORY: &str = "files-ledger";

fn default_log_level() -> String {
    LevelFilter::Info.to_string()
}

fn default_ledger_path() -> String {
    dsm_dir()
        .join(LEDGER_DIRECTORY)
        .to_string_lossy()
        .to_string()
}

fn default_store_path() -> String {
    dsm_dir().join(STORE).to_string_lossy().to_string()
}

fn default_action_count() -> usize {
    2048
}

fn default_remotes() -> HashMap<String, Remote> {
    HashMap::new()
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    #[serde(default = "default_log_level")]
    log_level: String,

    #[serde(default = "default_ledger_path")]
    ledger_path: String,

    #[serde(default = "default_store_path")]
    store_path: String,

    #[serde(default = "default_action_count")]
    action_count: usize,

    #[serde(default = "default_remotes")]
    remotes: HashMap<String, Remote>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
            ledger_path: default_ledger_path(),
            store_path: default_store_path(),
            action_count: default_action_count(),
            remotes: default_remotes(),
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

    pub fn get_action_count(&self) -> usize {
        self.action_count
    }

    pub fn get_store_path(&self) -> PathBuf {
        PathBuf::from_str(self.store_path.as_str())
            .unwrap_or_else(|_| panic!("Invalid dataset database location: {}", self.ledger_path))
    }

    pub fn get_ledger_path(&self) -> PathBuf {
        PathBuf::from_str(self.ledger_path.as_str())
            .unwrap_or_else(|_| panic!("Invalid dataset database location: {}", self.ledger_path))
    }

    pub fn load() -> Self {
        let settings_filename = dsm_dir().join(SETTINGS_FILENAME);
        if let Ok(content) = read_to_string(&settings_filename) {
            toml::from_str(content.as_str()).unwrap_or_default()
        } else {
            trace!(
                "Settings file not found at '{}'",
                settings_filename.display()
            );
            Self::default()
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let settings_filename = dsm_dir().join(SETTINGS_FILENAME);
        let content = match toml::to_string(self) {
            Ok(ctnt) => ctnt,
            Err(err) => return log_err!(format!("Could not serialize in toml settings: {err}")),
        };
        trace!(
            "Saving settings \n\"\"\" Path: '{}'\n{}\n\"\"\"",
            settings_filename.display(),
            content
        );
        write_to_file(&settings_filename, content, true, true)
    }

    pub fn get_remotes(&mut self) -> &mut HashMap<String, Remote> {
        &mut self.remotes
    }

    pub fn get_remote(&self, name: String) -> Option<&Remote> {
        self.remotes.get(&name)
    }
}
