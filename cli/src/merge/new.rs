use clap::Args;
use interfaces::logger::error;
use interfaces::models::{ClassMapper, MergedSet, Settings};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
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
	version: String,
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
				return error(format!("Cannot have twice the same dataset {}", sets_label));
			}
			let splits: Vec<&str> = sets_label.split('=').collect();
			if splits.len() != 2 {
				return error(format!("Can only have 1 equals sign in {}", sets_label));
			}
			let name = splits[0];
			let version = splits[1];
			let local_set = storage.read(name.to_string(), version.to_string()).await?;
			if let Some(local_set) = local_set {
				datasets.insert(sets_label.clone(), local_set);
			} else {
				return error(format!("Could not find {}", sets_label));
			}
		}
		let content = match read_to_string(&self.mapping_file) {
			Ok(content) => content,
			Err(err) => return error(format!("Could not read file '{}': {err}", &self.mapping_file.display()))
		};
		let mapping: ClassMapper =  match toml::from_str(&content) {
			Ok(content) => content,
			Err(err) => return error(format!("Could not deserialize toml mapping file '{}': {err}", &self.mapping_file.display()))
		};
		let merge_set = MergedSet::new(datasets, self.name.clone(), self.version.clone(),  mapping);
		storage.ledge(&merge_set).await?;

		Ok(())
	}
}

