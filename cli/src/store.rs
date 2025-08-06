use crate::format::Format;
use clap::Args;
use interfaces::models::{DsmLocation, Settings};
use log::debug;
use serializer::DataForm;
use std::path::PathBuf;
use storage::Storage;

/// Parse the files in a known standard format and store the set in a storage
#[derive(Args, Debug)]
pub struct Store {
	/// Root directory of the files
	path: String,

	/// In which format the files will be read as.
	///
	/// If it is not a known standard, it will pick a script in scripts directory.
	///
	/// Standard supported:
	/// - coco-1-0
	/// - yolo-1-1
	///
	/// To use a custom reader, prefix it with custom:<NAME>
	/// The name needs to be in the settings
	#[clap(short, long, verbatim_doc_comment)]
	formats: String,

	/// Datasets registered name
	#[clap(short, long)]
	name: Option<String>,

	/// Datasets registered version
	#[clap(short, long, default_value = "1")]
	version: String,
}

impl Store {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		let path = PathBuf::from(self.path.clone());
		let formatter: DataForm = Format::try_from(self.formats.clone())?.into();
		debug!("Storing dataset '{}' with '{}'", path.display(), formatter);
		let mut datasets = formatter.read(
			&path,
			self.name.clone(),
			self.version.clone(),
			settings.interpreters(),
		)?;
		let storage = Storage::local(settings);
		for dataset in datasets.iter_mut() {
			let save_path = storage.store(dataset, &path).await?;
			let new_location = DsmLocation::Local {
				path: save_path
					.canonicalize()?
					.to_str()
					.expect("Save path incorrect")
					.to_string(),
			};
			dataset.update_location(new_location);
			storage.ledge(dataset).await?;
		}
		Ok(())
	}
}
