use crate::format::Format;
use bhomz::log_err;
use clap::Args;
use interfaces::models::{MergedSet, Settings};
use serializer::Serializer;
use std::path::PathBuf;
use storage::Storage;

/// Create a direvative dataset from storage in a known format at the specified path location
#[derive(Args, Debug)]
pub struct Generator {
	/// Target location for the resulting direvative
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

	/// Registered name of the dataset
	#[clap(short, long)]
	datasets: String,

	/// Registered version of the dataset
	#[clap(short, long)]
	version: String,
}

impl Generator {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		if self.datasets.is_empty() {
			return log_err!("Require at least one dataset");
		}
		let formatter: Serializer = Format::try_from(self.formats.clone())?.into();
		let storage = Storage::local(settings);
		let datasets = storage
			.read(self.datasets.clone(), self.version.clone())
			.await?;
		let new_set = if let Some(dsm_set) = datasets {
			dsm_set
		} else {
			let merged_set: MergedSet = match storage
				.read(self.datasets.clone(), self.version.clone())
				.await?
			{
				Some(val) => val,
				None => return log_err!("Could not find datasets".to_string()),
			};

			merged_set.to_dataset(formatter.to_data_form())?
		};
		let path = PathBuf::from(&self.path);
		formatter.write(&path, &new_set, settings.interpreters())?;
		Ok(())
	}
}
