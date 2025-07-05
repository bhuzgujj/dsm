mod local;
mod storable;

use interfaces::models::metadata::DsmMetaDataBuilder;
use interfaces::models::remotes::Remote;
use interfaces::models::{DsmSets, MergedSet};
use local::Localable;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub enum Storage {
    Local {
        ledger_directory: PathBuf,
        store_directory: PathBuf,
    },
    Remote {
        service: Remote,
    },
}

impl Storage {
    pub async fn store<T: Localable>(
        &self,
        datasets: &T,
        datasets_path: &Path,
    ) -> anyhow::Result<()> {
        match self {
            Storage::Local {
                ledger_directory: _ledger_directory,
                store_directory,
            } => datasets.store(store_directory, datasets_path),

            Storage::Remote { service: _ } => todo!("remote::store::<T>(service, datasets).await"),
        }
    }

    pub async fn ledge<T: Localable>(&self, datasets: &T) -> anyhow::Result<()> {
        match self {
            Storage::Local {
                ledger_directory,
                store_directory: _store_directory,
            } => datasets.ledge(ledger_directory),

            Storage::Remote { service: _ } => todo!("remote::ledge::<T>(service, datasets).await"),
        }
    }

    pub async fn read<T: Localable>(
        &self,
        name: String,
        version: String,
    ) -> anyhow::Result<Option<T>> {
        match self {
            Storage::Local {
                ledger_directory,
                store_directory: _store_directory,
            } => T::read(ledger_directory, name, version),

            Storage::Remote { service: _ } => todo!("remote::read::<T>(service, name, version).await"),
        }
    }

    pub async fn list<T: Localable>(&self) -> anyhow::Result<Vec<T>> {
        match self {
            Storage::Local {
                ledger_directory,
                store_directory: _store_directory,
            } => T::list(ledger_directory),

            Storage::Remote { service: _ } => todo!("remote::list::<T>(service).await"),
        }
    }
}

pub async fn add_merge_link_to(
    datasets: &HashMap<String, DsmSets>,
    storage: Storage,
    merge_set: MergedSet,
) -> anyhow::Result<()> {
    for set in datasets.values() {
        let merged_key = format!("{}~{}", merge_set.get_name(), merge_set.get_version());
        let builder = DsmMetaDataBuilder::from(set.get_metadata().clone())
            .add_contained_in_merged(merged_key);
        storage
            .ledge(&DsmSets::new(builder.build(), set.get_entries().clone()))
            .await?;
    }
    Ok(())
}
