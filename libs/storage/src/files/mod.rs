mod copy;

use crate::files::copy::copy_recursively;
use anyhow::Error;
use interfaces::models::{DsmSets, MergedSet, StorableMerged};
use std::fs::{create_dir_all, read_dir, read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

const RAW_SET: &'static str = "raw_sets";
const MERGED_SET: &'static str = "merged_sets";

pub(crate) async fn store(
	store_directory: &PathBuf,
	ledger_directory: &PathBuf,
	datasets_path: &PathBuf,
	datasets: &DsmSets,
) -> anyhow::Result<()> {
	let subset = format!("v{}", datasets.get_version());
	let store_path = store_directory.join(datasets.get_name()).join(&subset);
	if store_path.exists() {
		return Err(Error::msg("Datasets already exists"))
	}

	let content = serde_json::to_string_pretty(&datasets)?;
	let ledger_raw_dir = ledger_directory.join(RAW_SET);
	create_dir_all(&ledger_raw_dir)?;
	let datasets_ledger = ledger_raw_dir.join(format!("{}-{}.json", datasets.get_name(), subset));
	if datasets_ledger.exists() {
		return Err(Error::msg("Datasets already exists"))
	}
	if ledger_directory.join(MERGED_SET).join(format!("{}-{}.json", datasets.get_name(), subset)).exists() {
		return Err(Error::msg("Datasets has the same name and version than a merged set"))
	}
	OpenOptions::new()
		.create(true)
		.write(true)
		.open(&datasets_ledger)?
		.write_all(content.as_bytes())?;
	create_dir_all(&store_path)?;
	copy_recursively(&datasets_path, &store_path)?;
	Ok(())
}

pub(crate) async fn read(_store_directory: &PathBuf, ledger_directory: &PathBuf, name: String, version: String) -> anyhow::Result<Option<DsmSets>> {
	let ledger_raw_dir = ledger_directory.join(RAW_SET);
	let datasets_ledger = ledger_raw_dir.join(format!("{}-v{}.json", name, version));
	if !datasets_ledger.exists() {
		return Ok(None)
	}
	let content = read_to_string(datasets_ledger)?;
	let datasets: DsmSets = serde_json::from_str(&content)?;
	Ok(Some(datasets))
}

pub(crate) async fn read_merged(_store_directory: &PathBuf, ledger_directory: &PathBuf, name: String, version: String) -> anyhow::Result<Option<MergedSet>> {
	let ledger_raw_dir = ledger_directory.join(RAW_SET);
	let datasets_ledger = ledger_directory.join(MERGED_SET).join(format!("{}-v{}.json", name, version));
	if !datasets_ledger.exists() {
		return Ok(None)
	}

	let content = read_to_string(datasets_ledger)?;
	let storable: StorableMerged = serde_json::from_str(&content)?;
	let mut datasets = Vec::new();
	for set in storable.datasets {
		let content = read_to_string(ledger_raw_dir.join(set))?;
		let ds: DsmSets = serde_json::from_str(&content)?;
		datasets.push(ds);
	}
	Ok(Some(MergedSet::from_vec(datasets, name, u32::from_str(version.as_str())?, storable.class_mapper, storable.license_mapper)?))
}

pub(crate) async fn list_raw(_store_directory: &PathBuf, ledger_directory: &PathBuf) -> anyhow::Result<Vec<DsmSets>> {
	let ledger_raw_dir = ledger_directory.join(RAW_SET);
	if !ledger_raw_dir.exists() {
		return Ok(Vec::new())
	}
	let mut datasets = Vec::new();
	for entry in read_dir(ledger_raw_dir)? {
		let entry = entry?;
		let path = entry.path();
		if path.is_dir() {
			continue;
		}
		let content = read_to_string(path)?;
		let dataset: DsmSets = serde_json::from_str(&content)?;
		datasets.push(dataset);
	}
	Ok(datasets)
}

pub(crate) async fn store_merged(_store_directory: &PathBuf, ledger_directory: &PathBuf, merged: &MergedSet) -> anyhow::Result<()> {
	let storable = merged.to_storable();
	let merged_file = format!("{}-v{}.json", merged.get_name(), merged.get_version());
	let merged_dir = ledger_directory.join(MERGED_SET);
	let merged_path = merged_dir.join(&merged_file);
	if merged_path.exists() {
		return Err(Error::msg("Merged set name with version already exists"))
	}
	if ledger_directory.join(RAW_SET).join(merged_file).exists() {
		return Err(Error::msg("Datasets has the same name and version than a merged set"))
	}
	create_dir_all(&merged_dir)?;
	let content = serde_json::to_string_pretty(&storable)?;
	OpenOptions::new()
		.create(true)
		.write(true)
		.open(merged_path)?
		.write_all(content.as_bytes())?;
	Ok(())
}