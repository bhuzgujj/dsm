use crate::format::Format;
use clap::Args;
use interfaces::models::Settings;
use serializer::DataForm;
use std::path::PathBuf;
use anyhow::anyhow;
use log::error;
use storage::Storage;

/// Parse the files in a known standard format
#[derive(Args, Debug)]
pub struct Std {
	/// In which format the files will be read as
	formats: Format,

	/// Root directory of the files
	path: String,

	/// Datasets to include
	#[clap(short, long)]
	datasets: String,

	/// Datasets registered version
	#[clap(short, long)]
	version: String,
}

impl Std {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		if self.datasets.is_empty() {
			error!("Require at least one dataset");
			return Err(anyhow!("Require at least one dataset"));
		}
		let formatter: DataForm = self.formats.clone().into();
		let path = PathBuf::from(&self.path);
		let storage = Storage::Local {
			ledger_directory: settings.get_ledger_path(),
			store_directory: settings.get_store_path(),
		};
		let datasets = storage.read(self.datasets.clone(), self.version.clone()).await?;
		let new_set = if let Some(ds) = datasets {
			ds
		} else {
			storage.read_merged(self.datasets.clone(), self.version.clone()).await?
				.expect("Could not find datasets")
				.to_dataset(formatter.to_data_form())?
		};
		formatter.write(&path, &settings.get_store_path(), &new_set)?;
		Ok(())
	}
}

