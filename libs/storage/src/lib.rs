mod files;

use interfaces::models::{DsmSets, MergedSet};
use std::path::{Path, PathBuf};

pub enum Storage {
	Local {
		ledger_directory: PathBuf,
		store_directory: PathBuf,
	},
	Remote(String)
}

impl Storage {
	pub async fn store(&self, datasets: &DsmSets, datasets_path: &Path) -> anyhow::Result<()> {
		match self {
			Storage::Local { ledger_directory, store_directory}  => {
				files::store(store_directory, ledger_directory, datasets_path, datasets).await
			}
			Storage::Remote(_) => {
				todo!()
			}
		}
	}
	pub async fn store_merged(&self, datasets: &MergedSet) -> anyhow::Result<()> {
		match self {
			Storage::Local { ledger_directory, store_directory}  => {
				files::store_merged(store_directory, ledger_directory, datasets).await
			}
			Storage::Remote(_) => {
				todo!()
			}
		}
	}

	pub async fn read(&self, name: String, version: String) -> anyhow::Result<Option<DsmSets>> {
		match self {
			Storage::Local{ledger_directory, store_directory} => {
				files::read_raw(store_directory, ledger_directory, name, version).await
			}
			Storage::Remote(_) => {
				todo!()
			}
		}
	}

	pub async fn read_merged(&self, name: String, version: String) -> anyhow::Result<Option<MergedSet>> {
		match self {
			Storage::Local{ledger_directory, store_directory} => {
				files::read_merged(store_directory, ledger_directory, name, version).await
			}
			Storage::Remote(_) => {
				todo!()
			}
		}
	}
	
	pub async fn list_raw(&self) -> anyhow::Result<Vec<DsmSets>> {
		match self {
			Storage::Local{ledger_directory, store_directory} => {
				files::list_raw(store_directory, ledger_directory).await
			}
			Storage::Remote(_) => {
				todo!()
			}
		}
	}
	
	pub async fn list_merged(&self) -> anyhow::Result<Vec<MergedSet>> {
		match self {
			Storage::Local{ledger_directory, store_directory} => {
				files::list_merged(store_directory, ledger_directory).await
			}
			Storage::Remote(_) => {
				todo!()
			}
		}
	}
}