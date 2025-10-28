use interfaces::{
	models::{actions::Action, datasets::Dataset},
	paths::write_to_file,
};
use log::{debug, info};
use std::{
	fs::{create_dir_all, read_dir},
	path::PathBuf,
};
use bhomz::log_err;
use crate::local::Namable;

use super::{copy_recursively, read_dsm, Storable, FORMAT_FILE_NAME, RAW_SET, SEPARATOR};

impl Storable for Dataset {
	fn ledge(&self, ledger_directory: &std::path::Path) -> anyhow::Result<bool> {
		debug!("Serializing content of {} into json", self.log_name());
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
		debug!(
			"Creating ledger directory for raw sets if it does not exists at '{}'",
			ledger_raw_dir.display()
		);
		if let Err(err) = create_dir_all(&ledger_raw_dir) {
			return log_err!(format!(
				"Failed to create dir {}: {}",
				ledger_raw_dir.display(),
				err
			));
		}

		let json_filename = format!("{}{SEPARATOR}{}.json", self.name(), self.version());
		let datasets_ledger = ledger_raw_dir.join(&json_filename);
		let update = datasets_ledger.exists();
		debug!(
			"Writing into the raw datasets ledger at '{}'",
			datasets_ledger.display()
		);
		write_to_file(&datasets_ledger, content, true, true)?;
		Ok(update)
	}

	fn store(
		&self,
		store_directory: &std::path::Path,
		originals_path: &std::path::Path,
	) -> anyhow::Result<PathBuf> {
		let store_path = store_directory.join(self.name()).join(self.version());
		debug!(
			"Verifying if the datasets already exists at '{}'",
			store_path.display()
		);
		if store_path.exists() {
			return log_err!(format!("{} already exists", self.log_name()));
		}

		debug!(
			"Creating store directory for raw sets if it does not exists at '{}'",
			store_path.display()
		);
		if let Err(err) = create_dir_all(&store_path) {
			return log_err!(format!(
				"Failed to create dir {}: {}",
				store_path.display(),
				err
			));
		}

		debug!(
			"Copying datasets into the store directory at '{}'",
			store_path.display()
		);
		if let Err(err) = copy_recursively(originals_path, &store_path) {
			log_err!(format!(
				"Failed to copy datasets from {} to {}: {}",
				originals_path.display(),
				store_path.display(),
				err
			))
		} else {
			debug!(
				"Write metadata for migration at '{}'",
				&store_path.join(FORMAT_FILE_NAME).display()
			);
			write_to_file(
				&store_path.join(FORMAT_FILE_NAME),
				self.metadata().formatter().to_string(),
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
		debug!("Reading datasets ledger at '{}'", datasets_path.display());
		if !datasets_path.exists() {
			return Ok(None);
		}

		let dsm_sets: Dataset = read_dsm(&datasets_path)?;
		Ok(Some(dsm_sets))
	}

	fn list(ledger_directory: &std::path::Path) -> anyhow::Result<Vec<Self>> {
		let ledger_raw_dir = ledger_directory.join(RAW_SET);
		if !ledger_raw_dir.exists() {
			info!(
				"Ledger's raw datasets directory does not exist at '{}'",
				ledger_raw_dir.display()
			);
			return Ok(Vec::new());
		}

		debug!(
			"Read ledger's raw datasets directory at '{}'",
			ledger_raw_dir.display()
		);
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
