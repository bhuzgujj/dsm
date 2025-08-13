pub(crate) mod local;
mod migration;

pub use migration::*;

use interfaces::models::actions::add_action;
use interfaces::models::datasets::{Dataset, MetaDataBuilder};
use interfaces::models::remotes::Remote;
use interfaces::models::{MergedSet, Settings};
use local::Storable;
use log::{debug, info};
use std::collections::HashMap;
use std::fmt::Display;
use std::path::{Path, PathBuf};

#[derive(Debug)]
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
			ledger_directory: settings.ledger_path(),
			store_directory: settings.store_path(),
			action_log_limit: settings.action_count(),
		}
	}
}

impl Storage {
	pub async fn store<T: Storable>(
		&self,
		datasets: &T,
		datasets_path: &Path,
	) -> anyhow::Result<PathBuf> {
		info!("Storing {} into {self}", datasets.log_name());
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
		info!("Ledging {} into {self}", datasets.log_name());
		match self {
			Storage::Local {
				ledger_directory,
				store_directory: _store_directory,
				action_log_limit,
			} => {
				if !datasets.ledge(ledger_directory)? {
					debug!(
						"Update action taken for ledging {} into {self}",
						datasets.log_name()
					);
					add_action(datasets.to_action(), *action_log_limit)?;
				}
				Ok(())
			},

			Storage::Remote { service: _ } => todo!("remote::ledge::<T>(service, datasets).await"),
		}
	}

	pub async fn read<T: Storable>(
		&self,
		name: String,
		version: String,
	) -> anyhow::Result<Option<T>> {
		info!(
			"Reading {} dataset named '{name}' with version '{version}' from {self}",
			T::type_name()
		);
		match self {
			Storage::Local {
				ledger_directory,
				store_directory: _store_directory,
				action_log_limit: _,
			} => T::read(ledger_directory, name, version),

			Storage::Remote { service: _ } => {
				todo!("remote::read::<T>(service, name, version).await")
			},
		}
	}

	pub async fn list<T: Storable>(&self) -> anyhow::Result<Vec<T>> {
		info!("Listing {} into {self}", T::type_name());
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

impl Display for Storage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Storage::Local {
				ledger_directory,
				store_directory,
				action_log_limit,
			} => f.write_str(
				format!(
					"Local (ledger: '{}', store: '{}', action_log_limit: {})",
					ledger_directory.display(),
					store_directory.display(),
					action_log_limit
				)
				.as_str(),
			),
			Storage::Remote { service } => f.write_str(format!("Remote.{service}").as_str()),
		}
	}
}

pub async fn add_merge_link_to(
	datasets: &HashMap<String, (String, Dataset)>,
	storage: Storage,
	merge_set: MergedSet,
) -> anyhow::Result<()> {
	for (_, set) in datasets.values() {
		let merged_key = format!("{}~{}", merge_set.name(), merge_set.version());
		let builder =
			MetaDataBuilder::from(set.metadata().clone()).add_contained_in_merged(merged_key);
		storage
			.ledge(&Dataset::new(builder.build(), set.entries().clone()))
			.await?;
	}
	Ok(())
}
