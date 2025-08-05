mod local;

use interfaces::models::actions::add_action;
use interfaces::models::metadata::DsmMetaDataBuilder;
use interfaces::models::remotes::Remote;
use interfaces::models::{DsmSets, MergedSet, Settings};
use local::Storable;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub enum Storage {
    Local {
        ledger_directory: PathBuf,
        store_directory: PathBuf,
        action_log_limit: usize,
    },
    Remote {
        service: Remote,
    },
}

impl Storage {
    pub fn local(settings: &Settings) -> Self {
        Self::Local {
            ledger_directory: settings.get_ledger_path(),
            store_directory: settings.get_store_path(),
            action_log_limit: settings.get_action_count(),
        }
    }
}

impl Storage {
    pub async fn store<T: Storable>(
        &self,
        datasets: &T,
        datasets_path: &Path,
    ) -> anyhow::Result<PathBuf> {
        match self {
            Storage::Local {
                ledger_directory: _ledger_directory,
                store_directory,
                action_log_limit: _,
            } => datasets.store(store_directory, datasets_path),

            Storage::Remote { service: _ } => todo!("remote::store::<T>(service, datasets).await"),
        }
    }

    pub async fn ledge<T: Storable>(&self, datasets: &T) -> anyhow::Result<()> {
        match self {
            Storage::Local {
                ledger_directory,
                store_directory: _store_directory,
                action_log_limit,
            } => {
                if !datasets.ledge(ledger_directory)? {
                    add_action(datasets.to_action(), *action_log_limit)?;
                }
                Ok(())
            }

            Storage::Remote { service: _ } => todo!("remote::ledge::<T>(service, datasets).await"),
        }
    }

    pub async fn read<T: Storable>(
        &self,
        name: String,
        version: String,
    ) -> anyhow::Result<Option<T>> {
        match self {
            Storage::Local {
                ledger_directory,
                store_directory: _store_directory,
                action_log_limit: _,
            } => T::read(ledger_directory, name, version),

            Storage::Remote { service: _ } => {
                todo!("remote::read::<T>(service, name, version).await")
            }
        }
    }

    pub async fn list<T: Storable>(&self) -> anyhow::Result<Vec<T>> {
        match self {
            Storage::Local {
                ledger_directory,
                store_directory: _store_directory,
                action_log_limit: _,
            } => T::list(ledger_directory),

            Storage::Remote { service: _ } => todo!("remote::list::<T>(service).await"),
        }
    }
}

pub async fn add_merge_link_to(
    datasets: &HashMap<String, (String, DsmSets)>,
    storage: Storage,
    merge_set: MergedSet,
) -> anyhow::Result<()> {
    for (_, set) in datasets.values() {
        let merged_key = format!("{}~{}", merge_set.get_name(), merge_set.get_version());
        let builder = DsmMetaDataBuilder::from(set.get_metadata().clone())
            .add_contained_in_merged(merged_key);
        storage
            .ledge(&DsmSets::new(builder.build(), set.get_entries().clone()))
            .await?;
    }
    Ok(())
}
