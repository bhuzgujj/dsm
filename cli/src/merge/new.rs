use std::collections::HashMap;
use std::fs::read_to_string;
use clap::Args;
use interfaces::models::{ClassMapper, MergedSet, Settings};
use std::path::PathBuf;
use anyhow::anyhow;
use log::error;
use storage::Storage;

/// Parse the files in a known standard format
#[derive(Args, Debug)]
pub struct New {
	/// The name of the merged set
	name: String,

	/// Datasets must be named <NAME>=<VERSION>
	#[clap(short, long)]
	datasets: Vec<String>,

	/// Mapping for the classes if needed
	#[clap(short, long)]
	mapping_file: PathBuf,

	/// The version of the merged set
	#[clap(short, long, default_value = "1")]
	version: u32,
}

impl New {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		let mut datasets = HashMap::new();
		let storage = Storage::Local {
			ledger_directory: settings.get_ledger_path(),
			store_directory: settings.get_store_path(),
		};
		for sets_label in self.datasets.iter() {
			if datasets.contains_key(sets_label) {
				error!("Cannot have twice the same dataset {}", sets_label);
				return Err(anyhow!("Cannot have twice the same dataset {}", sets_label));
			}
			let splits: Vec<&str> = sets_label.split('=').collect();
			if splits.len() != 2 {
				error!("Can only have 1 equals sign in {}", sets_label);
				return Err(anyhow!("Can only have 1 equals sign in {}", sets_label));
			}
			let name = splits[0];
			let version = splits[1];
			let local_set = storage.read(name.to_string(), version.to_string()).await?;
			if let Some(local_set) = local_set {
				datasets.insert(name.to_string(), local_set);
			} else {
				error!("Could not find {}", sets_label);
				return Err(anyhow!("Could not find {}", sets_label));
			}
		}
		let content = read_to_string(&self.mapping_file)?;
		let mapping = toml::from_str::<ClassMapper>(&content)?;
		let merge_set = MergedSet::new(datasets, self.name.clone(), self.version,  mapping)?;
		storage.store_merged(&merge_set).await?;

		Ok(())
	}
}

