use clap::Args;
use interfaces::log_err;
use interfaces::models::metadata::DsmMetaDataBuilder;
use interfaces::models::{ClassMapper, DsmSets, MergedSet, Settings};
use serializer::DataForm;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use storage::Storage;

use crate::format::Format;

/// Parse the files in a known standard format
#[derive(Args, Debug)]
pub struct Add {
    /// Merge set to add datasets to from, must be: <NAME>=<VERSION>
    from_set: String,

    /// The name of the merged set
    version: String,

    /// Datasets from the local storage, must be: <NAME>=<VERSION>
    #[clap(short, long)]
    datasets: Vec<String>,

    /// Datasets from a path, must be: <FORMAT>:<PATH>
    #[clap(short, long)]
    paths: Vec<String>,

    /// Updated mapping file (default keep previous)
    #[clap(short, long)]
    mapping_file: Option<PathBuf>,
}

impl Add {
    pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
        let mut datasets = HashMap::new();
        let storage = Storage::Local {
            ledger_directory: settings.get_ledger_path(),
            store_directory: settings.get_store_path(),
        };
        let splits: Vec<&str> = self.from_set.split('=').collect();
        if splits.len() != 2 {
            return log_err!(format!("Can only have 1 equals sign in {}", self.from_set));
        }
        let merged_name = splits[0];
        let merged_version = splits[1];
        let mut mapping: ClassMapper;
        let merged = storage
            .read::<MergedSet>(merged_name.to_string(), merged_version.to_string())
            .await?;
        if let Some(m) = merged {
            for dsm in m.get_datasets_include() {
                datasets.insert(
                    format!(
                        "{}={}",
                        dsm.get_metadata().get_name(),
                        dsm.get_metadata().get_version()
                    ),
                    dsm.clone(),
                );
            }
            mapping = m.get_mapping().clone();
        } else {
            return log_err!(format!("Could not find {merged_name} v{merged_version}"));
        }
        if let Some(mapping_path) = &self.mapping_file {
            let content = match read_to_string(&mapping_path) {
                Ok(content) => content,
                Err(err) => {
                    return log_err!(format!(
                        "Could not read file '{}': {err}",
                        &mapping_path.display()
                    ))
                }
            };
            mapping = match toml::from_str(&content) {
                Ok(content) => content,
                Err(err) => {
                    return log_err!(format!(
                        "Could not deserialize toml mapping file '{}': {err}",
                        &mapping_path.display()
                    ))
                }
            };
        }
        for sets_label in self.datasets.iter() {
            if datasets.contains_key(sets_label) {
                return log_err!(format!("Cannot have twice the same dataset {}", sets_label));
            }
            let splits: Vec<&str> = sets_label.split('=').collect();
            if splits.len() != 2 {
                return log_err!(format!("Can only have 1 equals sign in {}", sets_label));
            }
            let name = splits[0];
            let version = splits[1];
            let local_set = storage.read(name.to_string(), version.to_string()).await?;
            if let Some(local_set) = local_set {
                datasets.insert(sets_label.clone(), local_set);
            } else {
                return log_err!(format!("Could not find {}", sets_label));
            }
        }
        for path in &self.paths {
            let splits: Vec<&str> = path.split(':').collect();
            if splits.len() != 2 {
                return log_err!(format!("Can only have 1 equals sign in {}", path));
            }
            let format: DataForm = Format::from(splits[0].trim().to_string()).into();
            let dataset_path = PathBuf::from(splits[1].trim());
            let dataset = format.read(&dataset_path, None, "0".to_string())?;
            for ds in dataset {
                storage.store(&ds, &dataset_path).await?;
                storage.ledge(&ds).await?;
                datasets.insert(ds.get_name().clone(), ds);
            }
        }
        let merge_set = MergedSet::new(
            datasets.clone(),
            merged_name.to_string(),
            self.version.clone(),
            mapping,
        );
        storage.ledge(&merge_set).await?;

        for set in datasets.values() {
            let builder =
                DsmMetaDataBuilder::from(set.get_metadata().clone()).add_contained_in_merged(
                    format!("{}~{}", merge_set.get_name(), merge_set.get_version()),
                );
            storage
                .ledge(&DsmSets::new(builder.build(), set.get_entries().clone()))
                .await?;
        }

        Ok(())
    }
}
