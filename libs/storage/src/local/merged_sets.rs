use std::{
	fs::{create_dir_all, read_dir},
	path::{Path, PathBuf},
};
use bhomz::log_err;
use interfaces::{
	models::{
		actions::{Action, SetInfo},
		datasets::Dataset,
		MergedSet, StorableMerged,
	},
	namable::Namable,
	paths::write_to_file,
};
use log::{debug, info};

use super::{read_dsm, Storable, MERGED_SET, RAW_SET, SEPARATOR};

impl Storable for MergedSet {
	fn ledge(&self, ledger_directory: &Path) -> anyhow::Result<bool> {
		let storable: StorableMerged = self.to_storable();
		let merged_file = format!("{}{SEPARATOR}{}.json", self.name(), self.version());
		let merged_dir = ledger_directory.join(MERGED_SET);
		let merged_path = merged_dir.join(&merged_file);

		debug!(
			"Verifying if a {} already exists at '{}'",
			MergedSet::type_name(),
			merged_path.display()
		);
		if merged_path.exists() {
			return log_err!(format!(
				"{} already has the name '{}' and version '{}'",
				MergedSet::type_name(),
				self.name(),
				self.version()
			));
		}

		debug!(
			"Verifying if a {} already exists at '{}'",
			Dataset::type_name(),
			merged_path.display()
		);
		if ledger_directory.join(RAW_SET).join(merged_file).exists() {
			return log_err!(format!(
				"{} already has the name '{}' and version '{}'",
				Dataset::type_name(),
				self.name(),
				self.version()
			));
		}

		debug!(
			"Creating ledger directory for raw sets if it does not exists at '{}'",
			merged_dir.display()
		);
		if let Err(err) = create_dir_all(&merged_dir) {
			return log_err!(format!(
				"Failed to create dir {}: {}",
				merged_dir.display(),
				err
			));
		}

		debug!("Serializing content of {} into json", self.log_name());
		match serde_json::to_string_pretty(&storable) {
			Ok(content) => {
				debug!(
					"Writing into the raw datasets ledger at '{}'",
					merged_path.display()
				);
				write_to_file(&merged_path, content, true, true)?;
				Ok(false)
			},
			Err(err) => log_err!(format!(
				"Failed to serialize datasets {}: {}",
				merged_dir.display(),
				err
			)),
		}
	}

	fn store(&self, _store_directory: &Path, _originals_path: &Path) -> anyhow::Result<PathBuf> {
		todo!("Not implemented, It should be capable of querying dsm from remotes")
	}

	fn read(
		ledger_directory: &Path,
		name: String,
		version: String,
	) -> anyhow::Result<Option<Self>> {
		let ledger_raw_dir = ledger_directory.join(RAW_SET);
		let datasets_ledger = ledger_directory
			.join(MERGED_SET)
			.join(format!("{name}{SEPARATOR}{version}.json"));

		debug!(
			"Reading {} ledger at '{}'",
			MergedSet::type_name(),
			datasets_ledger.display()
		);
		if !datasets_ledger.exists() {
			return Ok(None);
		}

		let storable: StorableMerged = read_dsm(&datasets_ledger)?;
		let mut datasets: Vec<(String, Dataset)> = Vec::new();
		debug!(
			"Reading all {} included {}.",
			storable.datasets().len(),
			Dataset::type_name()
		);
		for (name, storable_group_set) in storable.datasets() {
			datasets.push((
				storable_group_set.group().to_owned(),
				read_dsm(&ledger_raw_dir.join(format!("{}.json", &name)))?,
			));
		}
		Ok(Some(MergedSet::from_storable(
			datasets,
			name,
			version,
			storable.class_mapper().to_owned(),
			storable.license_mapper().to_owned(),
		)))
	}

	fn list(ledger_directory: &Path) -> anyhow::Result<Vec<Self>> {
		let ledger_raw_dir = ledger_directory.join(RAW_SET);
		let ledger_merged_dir = ledger_directory.join(MERGED_SET);
		if !ledger_raw_dir.exists() {
			info!(
				"Ledger's {} directory at '{}' does not exists",
				Dataset::type_name(),
				ledger_raw_dir.display()
			);
			return Ok(Vec::new());
		}
		if !ledger_merged_dir.exists() {
			info!(
				"Ledger's {} directory at '{}' does not exists",
				MergedSet::type_name(),
				ledger_merged_dir.display()
			);
			return Ok(Vec::new());
		}

		debug!(
			"Read ledger's {} directory at '{}'",
			MergedSet::type_name(),
			ledger_merged_dir.display()
		);
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
					for (set, group) in storable_set.datasets() {
						datasets.push((
							group.group().to_owned(),
							read_dsm(&ledger_raw_dir.join(format!("{set}.json")))?,
						));
					}
					mergedsets.push(MergedSet::from_storable(
						datasets,
						storable_set.name().to_owned(),
						storable_set.version().to_owned(),
						storable_set.class_mapper().to_owned(),
						storable_set.license_mapper().to_owned(),
					));
				}
				Ok(mergedsets)
			},
			Err(err) => {
				log_err!(format!(
					"Failed to read dataset directory '{}': {err}",
					ledger_merged_dir.display()
				))
			},
		}
	}

	fn to_action(&self) -> Action {
		let mut datasets = Vec::new();
		for dsms in self.datasets_include().values() {
			for dsm in dsms {
				datasets.push(SetInfo::new(
					dsm.name().to_owned(),
					dsm.version().to_owned(),
				));
			}
		}
		Action::merge(self.name().to_string(), self.version().to_owned(), datasets)
	}
}
