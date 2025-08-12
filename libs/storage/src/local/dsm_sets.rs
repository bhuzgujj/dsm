use interfaces::{
	log_err,
	models::{actions::Action, datasets::Dataset},
	paths::write_to_file,
};
use std::{
	fs::{create_dir_all, read_dir},
	path::PathBuf,
};

use super::{copy_recursively, read_dsm, Storable, FORMAT_FILE_NAME, RAW_SET, SEPARATOR};

impl Storable for Dataset {
	fn ledge(&self, ledger_directory: &std::path::Path) -> anyhow::Result<bool> {
		let json_filename = format!("{}{SEPARATOR}{}.json", self.name(), self.version());

		let content = match serde_json::to_string_pretty(&self) {
			Ok(content) => content,
			Err(err) => {
				return log_err!(format!(
					"Failed to serialize datasets {}: {}",
					self.name(),
					err
				));
			},
		};
		let ledger_raw_dir = ledger_directory.join(RAW_SET);
		if let Err(err) = create_dir_all(&ledger_raw_dir) {
			return log_err!(format!(
				"Failed to create dir {}: {}",
				ledger_raw_dir.display(),
				err
			));
		}
		let datasets_ledger = ledger_raw_dir.join(&json_filename);
		let update = datasets_ledger.exists();
		write_to_file(&datasets_ledger, content, true, true)?;
		Ok(update)
	}

	fn store(
		&self,
		store_directory: &std::path::Path,
		originals_path: &std::path::Path,
	) -> anyhow::Result<PathBuf> {
		let json_filename = format!("{}{SEPARATOR}{}.json", self.name(), self.version());
		let store_path = store_directory.join(self.name()).join(self.version());
		if store_path.exists() {
			return log_err!(format!("Datasets already exists: {json_filename}"));
		}
		if let Err(err) = create_dir_all(&store_path) {
			return log_err!(format!(
				"Failed to create dir {}: {}",
				store_path.display(),
				err
			));
		}
		if let Err(err) = copy_recursively(originals_path, &store_path) {
			log_err!(format!(
				"Failed to copy datasets from {} to {}: {}",
				originals_path.display(),
				store_path.display(),
				err
			))
		} else {
			write_to_file(
				&store_path.join(FORMAT_FILE_NAME),
				self.metadata.formatter.to_string(),
				true,
				true,
			)?;
			Ok(store_path.clone())
		}
	}

	fn read(
		ledger_directory: &std::path::Path,
		name: String,
		version: String,
	) -> anyhow::Result<Option<Self>> {
		let ledger_raw_dir = ledger_directory.join(RAW_SET);
		let datasets_path = ledger_raw_dir.join(format!("{name}{SEPARATOR}{version}.json"));
		if !datasets_path.exists() {
			return Ok(None);
		}
		let dsm_sets: Dataset = read_dsm(&datasets_path)?;
		Ok(Some(dsm_sets))
	}

	fn list(ledger_directory: &std::path::Path) -> anyhow::Result<Vec<Self>> {
		let ledger_raw_dir = ledger_directory.join(RAW_SET);
		if !ledger_raw_dir.exists() {
			return Ok(Vec::new());
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
					let dataset: Dataset = read_dsm(&path)?;
					datasets.push(dataset);
				}
				Ok(datasets)
			},
			Err(err) => {
				log_err!(format!(
					"Failed to read dataset directory '{}': {err}",
					ledger_raw_dir.display()
				))
			},
		}
	}

	fn to_action(&self) -> Action {
		Action::store(self.name().clone(), self.version().clone())
	}
}
