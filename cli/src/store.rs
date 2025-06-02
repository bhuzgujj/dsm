use crate::format::Format;
use clap::Args;
use interfaces::models::Settings;
use serializer::DataForm;
use std::path::PathBuf;
use log::{debug};
use storage::Storage;

/// Parse the files in a known standard format
#[derive(Args, Debug)]
pub struct Store {
	/// In which format the files will be read as.
	/// If it is not a known standard, it will pick a script in scripts directory.
	/// Standard supported: 
	///  - coco-1-0
	///  - yolo-1-1
	#[clap(verbatim_doc_comment)]
	formats: String,

	/// Root directory of the files
	path: String,

	/// Datasets registered name
	#[clap(short, long)]
	name: Option<String>,

	/// Datasets registered version
	#[clap(short, long, default_value="1")]
	version: String,
}

impl Store {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		let path = PathBuf::from(self.path.clone());
		let formatter: DataForm = Format::from(self.formats.clone()).into();
		debug!("Storing dataset '{}' with '{}'", path.display(), formatter);
		let datasets = formatter.read(&path, self.name.clone(), self.version.clone())?;
		let storage = Storage::Local {
			ledger_directory: settings.get_ledger_path(),
			store_directory: settings.get_store_path(),
		};
		for dataset in datasets.iter() {
			storage.store(dataset, &path).await?;
			storage.ledge(dataset).await?;
		}
		Ok(())
	}
}
