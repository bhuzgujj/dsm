use clap::Args;
use interfaces::models::{ClassMapper, MergedSet, Settings};
use std::collections::HashMap;
use std::path::PathBuf;
use storage::{add_merge_link_to, Storage};

use crate::merge::{extract_dsm_from_path, extract_dsm_from_storage};

/// Create a merged dataset from other dataset
#[derive(Args, Debug)]
pub struct New {
	/// The name of the merged set
	#[clap(short, long)]
	name: String,

	/// Datasets from the local storage, must be: <NAME>=<VERSION>=<ASSIGN_GROUP>
	#[clap(short, long)]
	datasets: Vec<String>,

	/// Datasets from a path, must be: <FORMAT>:<PATH>:<ASSIGN_GROUP>
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
		let storage = Storage::local(settings);

		extract_dsm_from_storage(&self.datasets, &mut datasets, &storage).await?;
		extract_dsm_from_path(&self.paths, &mut datasets, &storage, settings).await?;

		let mapping = ClassMapper::read_from_file(&self.mapping_file)?;
		let merge_set = MergedSet::new(
			datasets.clone(),
			self.name.clone(),
			self.version.clone(),
			mapping,
		);
		storage.ledge(&merge_set).await?;

		add_merge_link_to(&datasets, storage, merge_set).await
	}
}
