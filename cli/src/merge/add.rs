use clap::Args;
use interfaces::log_err;
use interfaces::models::metadata::DsmMetaDataBuilder;
use interfaces::models::{ClassMapper, DsmSets, MergedSet, Settings};
use serializer::DataForm;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use storage::{add_merge_link_to, Storage};

use crate::format::Format;
use crate::merge::{extract_dsm_from_path, extract_dsm_from_storage};

/// Add datasets to a merge set and create a new version out of it
#[derive(Args, Debug)]
pub struct Add {
    /// Merge set to add datasets to from, must be: <NAME>=<VERSION>
    from_set: String,

    /// The name of the new set (default will be the previous name)
    #[clap(short, long)]
    name: Option<String>,

    /// The version of the new set
    #[clap(short, long)]
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
            mapping = ClassMapper::read_from_file(&mapping_path)?;
        }

        extract_dsm_from_storage(&self.datasets, &mut datasets, &storage).await?;
        extract_dsm_from_path(&self.paths, &mut datasets, &storage).await?;

        let merge_set = MergedSet::new(
            datasets.clone(),
            self.name.clone().unwrap_or(merged_name.to_string()),
            self.version.clone(),
            mapping
        );
        storage.ledge(&merge_set).await?;

        add_merge_link_to(&datasets, storage, merge_set).await
    }
}
