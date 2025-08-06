use std::{
	fs::{create_dir_all, read_dir},
	path::PathBuf,
};

use interfaces::{
	log_err,
	models::{
		actions::{Action, SetInfo},
		DsmSets, MergedSet, StorableMerged,
	},
	paths::write_to_file,
};

use super::{read_dsm, Storable, MERGED_SET, RAW_SET, SEPARATOR};

impl Storable for MergedSet {
	fn ledge(&self, ledger_directory: &std::path::Path) -> anyhow::Result<bool> {
		let storable: interfaces::models::StorableMerged = self.to_storable();
		let merged_file = format!("{}{SEPARATOR}{}.json", self.name(), self.version());
		let merged_dir = ledger_directory.join(MERGED_SET);
		let merged_path = merged_dir.join(&merged_file);
		if merged_path.exists() {
			return log_err!(format!(
				"Merged set '{} v{}' already exists",
				self.name(),
				self.version()
			));
		}
		if ledger_directory.join(RAW_SET).join(merged_file).exists() {
			return log_err!(format!(
				"Raw set already exists with '{} v{}'",
				self.name(),
				self.version()
			));
		}
		if let Err(err) = create_dir_all(&merged_dir) {
			return log_err!(format!(
				"Failed to create dir {}: {}",
				merged_dir.display(),
				err
			));
		}
		match serde_json::to_string_pretty(&storable) {
			Ok(content) => {
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

	fn store(
		&self,
		_store_directory: &std::path::Path,
		_originals_path: &std::path::Path,
	) -> anyhow::Result<PathBuf> {
		todo!("Not implemented")
	}

	fn read(
		ledger_directory: &std::path::Path,
		name: String,
		version: String,
	) -> anyhow::Result<Option<Self>> {
		let ledger_raw_dir = ledger_directory.join(RAW_SET);
		let datasets_ledger = ledger_directory
			.join(MERGED_SET)
			.join(format!("{}{SEPARATOR}{}.json", name, version));
		if !datasets_ledger.exists() {
			return Ok(None);
		}
		let storable: StorableMerged = read_dsm(&datasets_ledger)?;

		let mut datasets: Vec<(String, DsmSets)> = Vec::new();
		for (name, set) in storable.datasets() {
			datasets.push((
				set.group().to_owned(),
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

	fn list(ledger_directory: &std::path::Path) -> anyhow::Result<Vec<Self>> {
		let ledger_raw_dir = ledger_directory.join(RAW_SET);
		let ledger_merged_dir = ledger_directory.join(MERGED_SET);
		if !ledger_raw_dir.exists() || !ledger_merged_dir.exists() {
			return Ok(Vec::new());
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
					for (set, group) in storable_set.datasets() {
						datasets.push((
							group.group().to_owned(),
							read_dsm(&ledger_raw_dir.join(format!("{}.json", set)))?,
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
