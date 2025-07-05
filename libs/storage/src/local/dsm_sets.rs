use std::fs::{create_dir_all, read_dir};

use interfaces::{log_err, models::DsmSets, paths::write_to_file};

use super::{copy_recursively, Localable, read_dsm, SEPARATOR, RAW_SET};

impl Localable for DsmSets {
    fn ledge(&self, ledger_directory: &std::path::Path) -> anyhow::Result<()> {
        let json_filename = format!("{}{SEPARATOR}{}.json", self.get_name(), self.get_version());

        let content = match serde_json::to_string_pretty(&self) {
            Ok(content) => content,
            Err(err) => return log_err!(format!("Failed to serialize datasets {}: {}", self.get_name(), err))
        };
        let ledger_raw_dir = ledger_directory.join(RAW_SET);
        if let Err(err) = create_dir_all(&ledger_raw_dir) {
            return log_err!(format!("Failed to create dir {}: {}", ledger_raw_dir.display(), err))
        }
        let datasets_ledger = ledger_raw_dir.join(&json_filename);
        write_to_file(&datasets_ledger, content, true, true)
    }

    fn store(&self, store_directory: &std::path::Path, originals_path: &std::path::Path) -> anyhow::Result<()> {
        let json_filename = format!("{}{SEPARATOR}{}.json", self.get_name(), self.get_version());
        let store_path = store_directory.join(self.get_name()).join(self.get_version());
        if store_path.exists() {
            return log_err!(format!("Datasets already exists: {json_filename}"))
        }
        if let Err(err) = create_dir_all(&store_path) {
            return log_err!(format!("Failed to create dir {}: {}", store_path.display(), err))
        }
        if let Err(err) = copy_recursively(originals_path, &store_path) {
            log_err!(format!("Failed to copy datasets from {} to {}: {}", originals_path.display(), store_path.display(), err))
        } else {
            Ok(())
        }
    }

    fn read(
        ledger_directory: &std::path::Path,
        name: String,
        version: String,
    ) -> anyhow::Result<Option<Self>> {
        let ledger_raw_dir = ledger_directory.join(RAW_SET);
        let datasets_path = ledger_raw_dir.join(format!("{}{SEPARATOR}{}.json", name, version));
        if !datasets_path.exists() {
            return Ok(None)
        }
        let dsm_sets: DsmSets = read_dsm(&datasets_path)?;
        Ok(Some(dsm_sets))
    }

    fn list(ledger_directory: &std::path::Path) -> anyhow::Result<Vec<Self>> {
        let ledger_raw_dir = ledger_directory.join(RAW_SET);
        if !ledger_raw_dir.exists() {
            return Ok(Vec::new())
        }
        match read_dir(&ledger_raw_dir) {
            Ok(dir) => {
                let mut datasets = Vec::new();
                for entry in dir {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() || path.extension().is_none_or(|ext| ext != "json") {
                        continue;
                    }
                    let dataset: DsmSets = read_dsm(&path)?;
                    datasets.push(dataset);
                }
                Ok(datasets)
            }
            Err(err) => {
                log_err!(format!("Failed to read dataset directory '{}': {err}", ledger_raw_dir.display()))
            }
        }
    }
}