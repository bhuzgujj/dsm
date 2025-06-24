use std::fs::{create_dir_all, read_dir};

use interfaces::{log_err, models::{DsmSets, MergedSet, StorableMerged}, paths::write_to_file};

use super::{Localable, read_dsm, MERGED_SET,SEPARATOR, RAW_SET};

impl Localable for MergedSet {
    fn ledge(&self, ledger_directory: &std::path::Path) -> anyhow::Result<()> {
        let storable: interfaces::models::StorableMerged = self.to_storable();
        let merged_file = format!("{}{SEPARATOR}{}.json", self.get_name(), self.get_version());
        let merged_dir = ledger_directory.join(MERGED_SET);
        let merged_path = merged_dir.join(&merged_file);
        if merged_path.exists() {
            return log_err!(format!("Merged set '{} v{}' already exists", self.get_name(), self.get_version()))
        }
        if ledger_directory.join(RAW_SET).join(merged_file).exists() {
            return log_err!(format!("Raw set already exists with '{} v{}'", self.get_name(), self.get_version()))
        }
        if let Err(err) = create_dir_all(&merged_dir) {
            return log_err!(format!("Failed to create dir {}: {}", merged_dir.display(), err))
        }
        match serde_json::to_string_pretty(&storable) {
            Ok(content) => write_to_file(&merged_path, content, true, true),
            Err(err) => log_err!(format!("Failed to serialize datasets {}: {}", merged_dir.display(), err))
        }
    }

    fn store(
        &self,
        _store_directory: &std::path::Path,
        _originals_path: &std::path::Path,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn read(
        ledger_directory: &std::path::Path,
        name: String,
        version: String,
    ) -> anyhow::Result<Option<Self>> {
        let ledger_raw_dir = ledger_directory.join(RAW_SET);
        let datasets_ledger = ledger_directory.join(MERGED_SET).join(format!("{}{SEPARATOR}{}.json", name, version));
        if !datasets_ledger.exists() {
            return Ok(None)
        }
        let storable: StorableMerged = read_dsm(&datasets_ledger)?;

        let mut datasets: Vec<DsmSets> = Vec::new();
        for (set, _) in storable.datasets {
            datasets.push(read_dsm(&ledger_raw_dir.join(format!("{}.json", &set)))?);
        }
        Ok(Some(MergedSet::from_vec(datasets, name, version, storable.class_mapper, storable.license_mapper)))
    }

    fn list(ledger_directory: &std::path::Path) -> anyhow::Result<Vec<Self>> {
        let ledger_raw_dir = ledger_directory.join(RAW_SET);
        let ledger_merged_dir = ledger_directory.join(MERGED_SET);
        if !ledger_raw_dir.exists() || !ledger_merged_dir.exists() {
            return Ok(Vec::new())
        }
        match read_dir(&ledger_merged_dir) {
            Ok(dir) => {
                let mut mergedsets = Vec::new();
                for entry in dir {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        continue;
                    }
                    let storable_set: StorableMerged = read_dsm(&path)?;
                    let mut datasets = Vec::new();
                    for (set, _) in storable_set.datasets {
                        datasets.push(read_dsm(&ledger_raw_dir.join(format!("{}.json", set)))?);
                    }
                    mergedsets.push(MergedSet::from_vec(
                        datasets,
                        storable_set.name,
                        storable_set.version,
                        storable_set.class_mapper,
                        storable_set.license_mapper
                    ));
                }
                Ok(mergedsets)
            }
            Err(err) => {
                log_err!(format!("Failed to read dataset directory '{}': {err}", ledger_merged_dir.display()))
            }
        }
    }
}