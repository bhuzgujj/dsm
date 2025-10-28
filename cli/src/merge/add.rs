use clap::Args;
use interfaces::models::{ClassMapper, MergedSet, Settings};
use std::collections::HashMap;
use std::path::PathBuf;
use bhomz::log_err;
use storage::{add_merge_link_to, Storage};

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

	/// Datasets from the local storage, must be: <NAME>=<VERSION>=<ASSIGN_GROUP>
	#[clap(short, long)]
	datasets: Vec<String>,

	/// Datasets from a path, must be: <FORMAT>=<PATH>=<ASSIGN_GROUP>
	#[clap(short, long)]
	paths: Vec<String>,

	/// Updated mapping file (default keep previous)
	#[clap(short, long)]
	mapping_file: Option<PathBuf>,
}

impl Add {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		let mut datasets = HashMap::new();
		let storage = Storage::local(settings);
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
			for (group, dsms) in m.datasets_include() {
				for dsm in dsms {
					datasets.insert(
						format!("{}={}", dsm.name(), dsm.version()),
						(group.clone(), dsm.clone()),
					);
				}
			}
			mapping = m.class_mapping().clone();
		} else {
			return log_err!(format!("Could not find {merged_name} v{merged_version}"));
		}
		if let Some(mapping_path) = &self.mapping_file {
			mapping = ClassMapper::read_from_file(mapping_path)?;
		}

		extract_dsm_from_storage(&self.datasets, &mut datasets, &storage).await?;
		extract_dsm_from_path(&self.paths, &mut datasets, &storage, settings).await?;

		let merge_set = MergedSet::new(
			datasets.clone(),
			self.name.clone().unwrap_or(merged_name.to_string()),
			self.version.clone(),
			mapping,
		);
		storage.ledge(&merge_set).await?;

		add_merge_link_to(&datasets, storage, merge_set).await
	}
}
