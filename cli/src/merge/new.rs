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
pub struct New {
	/// The name of the merged set
	name: String,

	/// Datasets from the local storage, must be: <NAME>=<VERSION>
	#[clap(short, long)]
	datasets: Vec<String>,

	/// Datasets from a path, must be: <FORMAT>:<PATH>
	#[clap(short, long)]
	paths: Vec<String>,

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
		let content = match read_to_string(&self.mapping_file) {
			Ok(content) => content,
			Err(err) => return log_err!(format!("Could not read file '{}': {err}", &self.mapping_file.display()))
		};
		let mapping: ClassMapper =  match toml::from_str(&content) {
			Ok(content) => content,
			Err(err) => return log_err!(format!("Could not deserialize toml mapping file '{}': {err}", &self.mapping_file.display()))
		};
		let merge_set = MergedSet::new(datasets.clone(), self.name.clone(), self.version.clone(),  mapping);
		storage.ledge(&merge_set).await?;

		for set in datasets.values() {
			let builder = DsmMetaDataBuilder::from(set.get_metadata().clone())
				.add_contained_in_merged(format!("{}~{}", merge_set.get_name(), merge_set.get_version()));
			storage.ledge(&DsmSets::new(builder.build(), set.get_entries().clone())).await?;
		}

		Ok(())
	}
}

