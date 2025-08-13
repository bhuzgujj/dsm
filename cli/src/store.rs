use crate::format::Format;
use clap::Args;
use interfaces::{
	log_err,
	models::{Location, Settings},
};
use log::debug;
use serializer::Serializer;
use std::path::PathBuf;
use storage::Storage;

/// Read a directory using a format and store a copy into the store directory in settings. It will also create a json representation of the dataset in the ledger directory
#[derive(Args, Debug)]
pub struct Store {
	/// Root directory of the dataset to store
	path: String,

	/// In which format the dataset will be read as.
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

	/// Name in which the dataset will be stored as (default: directory name)
	#[clap(short, long)]
	name: Option<String>,

	/// Version in which the dataset will be stored
	#[clap(short, long, default_value = "1")]
	version: String,
}

impl Store {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		let path = PathBuf::from(self.path.clone());
		let formatter: Serializer = Format::try_from(self.formats.clone())?.into();

		debug!(
			"Storing datasets at '{}' using format '{}'",
			path.display(),
			formatter
		);
		let mut dataset = formatter.read(
			&path,
			self.name.clone(),
			self.version.clone(),
			settings.interpreters(),
		)?;
		let storage = Storage::local(settings);
		let save_path = storage.store(&dataset, &path).await?;
		match save_path.canonicalize()?.to_str() {
			Some(path) => {
				debug!(
					"Update location of images of {} to {}",
					dataset.name(),
					&path
				);
				let new_location = Location::Local {
					path: path.to_string(),
				};
				dataset.update_location(new_location);
				storage.ledge(&dataset).await
			},
			None => log_err!(format!(
				"Could not stringify the path of {}",
				save_path.display()
			)),
		}
	}
}
