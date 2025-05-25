mod copy;

use crate::files::copy::copy_recursively;
use interfaces::logger::error;
use interfaces::models::{DsmSets, MergedSet, StorableMerged};
use interfaces::paths::write_to_file;
use serde::de::DeserializeOwned;
use std::fs::{create_dir_all, read_dir, read_to_string};
use std::path::PathBuf;

const RAW_SET: &str = "raw_sets";
const MERGED_SET: &str = "merged_sets";

pub(crate) async fn store(
	store_directory: &PathBuf,
	ledger_directory: &PathBuf,
	datasets_path: &PathBuf,
	datasets: &DsmSets,
) -> anyhow::Result<()> {
	let subset = format!("v{}", datasets.get_version());
	let store_path = store_directory.join(datasets.get_name()).join(&subset);
	let json_filename = format!("{}-{}.json", datasets.get_name(), subset);
	if store_path.exists() {
		return error(format!("Datasets has the same name and version than a raw set: {json_filename}"))
	}

	let content = match serde_json::to_string_pretty(&datasets) {
		Ok(content) => content,
		Err(err) => return error(format!("Failed to serialize datasets {}: {}", datasets.get_name(), err))
	};
	let ledger_raw_dir = ledger_directory.join(RAW_SET);
	if let Err(err) = create_dir_all(&ledger_raw_dir) {
		return error(format!("Failed to create dir {}: {}", ledger_raw_dir.display(), err))
	}
	let datasets_ledger = ledger_raw_dir.join(&json_filename);
	if datasets_ledger.exists() {
		return error(format!("Datasets has the same name and version: {json_filename}"))
	}
	if ledger_directory.join(MERGED_SET).join(&json_filename).exists() {
		return error(format!("Datasets has the same name and version than a merged set: {json_filename}"))
	}
	write_to_file(&datasets_ledger, content, true, true)?;
	if let Err(err) = create_dir_all(&store_path) {
		return error(format!("Failed to create dir {}: {}", store_path.display(), err))
	}
	if let Err(err) = copy_recursively(datasets_path, &store_path) {
		error(format!("Failed to copy datasets from {} to {}: {}", datasets_path.display(), store_path.display(), err))
	} else {
		Ok(())
	}
}

pub(crate) async fn read_raw(_store_directory: &PathBuf, ledger_directory: &PathBuf, name: String, version: String) -> anyhow::Result<Option<DsmSets>> {
	let ledger_raw_dir = ledger_directory.join(RAW_SET);
	let datasets_path = ledger_raw_dir.join(format!("{}-v{}.json", name, version));
	if !datasets_path.exists() {
		return Ok(None)
	}
	let dsm_sets: DsmSets = read_dsm(&datasets_path)?;
	Ok(Some(dsm_sets))
}

pub(crate) async fn read_merged(_store_directory: &PathBuf, ledger_directory: &PathBuf, name: String, version: String) -> anyhow::Result<Option<MergedSet>> {
	let ledger_raw_dir = ledger_directory.join(RAW_SET);
	let datasets_ledger = ledger_directory.join(MERGED_SET).join(format!("{}-v{}.json", name, version));
	if !datasets_ledger.exists() {
		return Ok(None)
	}
	let storable: StorableMerged = read_dsm(&datasets_ledger)?;

	let mut datasets: Vec<DsmSets> = Vec::new();
	for set in storable.datasets {
		datasets.push(read_dsm(&ledger_raw_dir.join(&set))?);
	}
	Ok(Some(MergedSet::from_vec(datasets, name, version, storable.class_mapper, storable.license_mapper)))
}

pub(crate) async fn list_raw(_store_directory: &PathBuf, ledger_directory: &PathBuf) -> anyhow::Result<Vec<DsmSets>> {
	let ledger_raw_dir = ledger_directory.join(RAW_SET);
	if !ledger_raw_dir.exists() {
		return Ok(Vec::new())
	}
	match read_dir(&ledger_raw_dir) {
		Ok(dir) => {
			let mut datasets = Vec::new();
			for entry in dir {
				let entry = entry?;
				let path = entry.path();
				if path.is_dir() || path.extension().is_none_or(|ext| ext != "json") {
					continue;
				}
				let dataset: DsmSets = read_dsm(&path)?;
				datasets.push(dataset);
			}
			Ok(datasets)
		}
		Err(err) => {
			error(format!("Failed to read dataset directory '{}': {err}", ledger_raw_dir.display()))
		}
	}
}

pub(crate) async fn list_merged(_store_directory: &PathBuf, ledger_directory: &PathBuf) -> anyhow::Result<Vec<MergedSet>> {
	let ledger_raw_dir = ledger_directory.join(RAW_SET);
	let ledger_merged_dir = ledger_directory.join(MERGED_SET);
	if !ledger_raw_dir.exists() || !ledger_merged_dir.exists() {
		return Ok(Vec::new())
	}
	match read_dir(&ledger_merged_dir) {
		Ok(dir) => {
			let mut mergedsets = Vec::new();
			for entry in dir {
				let entry = entry?;
				let path = entry.path();
				if path.is_dir() {
					continue;
				}
				let storable_set: StorableMerged = read_dsm(&path)?;
				let mut datasets = Vec::new();
				for set in storable_set.datasets {
					datasets.push(read_dsm(&ledger_raw_dir.join(set))?);
				}
				mergedsets.push(MergedSet::from_vec(
					datasets,
					storable_set.name,
					storable_set.version,
					storable_set.class_mapper,
					storable_set.license_mapper
				));
			}
			Ok(mergedsets)
		}
		Err(err) => {
			error(format!("Failed to read dataset directory '{}': {err}", ledger_merged_dir.display()))
		}
	}
}

pub(crate) async fn store_merged(_store_directory: &PathBuf, ledger_directory: &PathBuf, merged: &MergedSet) -> anyhow::Result<()> {
	let storable = merged.to_storable();
	let merged_file = format!("{}-v{}.json", merged.get_name(), merged.get_version());
	let merged_dir = ledger_directory.join(MERGED_SET);
	let merged_path = merged_dir.join(&merged_file);
	if merged_path.exists() {
		return error(format!("Merged set '{} v{}' already exists", merged.get_name(), merged.get_version()))
	}
	if ledger_directory.join(RAW_SET).join(merged_file).exists() {
		return error(format!("Raw set already exists with '{} v{}'", merged.get_name(), merged.get_version()))
	}
	if let Err(err) = create_dir_all(&merged_dir) {
		return error(format!("Failed to create dir {}: {}", merged_dir.display(), err))
	}
	match serde_json::to_string_pretty(&storable) {
		Ok(content) => write_to_file(&merged_path, content, true, true),
		Err(err) => error(format!("Failed to serialize datasets {}: {}", merged_dir.display(), err))
	}
}

#[inline]
fn read_dsm<T: DeserializeOwned>(path: &PathBuf) -> anyhow::Result<T> {
	let content = match read_to_string(path) {
		Ok(ctnt) => ctnt,
		Err(err) => return error(format!("Failed to read datasets {}: {}", path.display(), err))
	};
	match serde_json::from_str(content.as_str()) {
		Ok(datasets) => Ok(datasets),
		Err(err) => error(format!("Failed to deserialize datasets {}: {}", path.display(), err))
	}
}