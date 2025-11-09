use bhomz::log_err;
use clap::Args;
use interfaces::models::{ClassMapper, Settings};
use serializer::Serializer;
use std::{fs::read_to_string, path::PathBuf};

use crate::format::Format;

/// Remap a dataset from a specified path to another. It will convert from a form to another and map its classes
#[derive(Debug, Args)]
pub struct Untrack {
	/// Path to the dataset
	#[clap(long)]
	input_path: PathBuf,

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
	#[clap(long, verbatim_doc_comment)]
	input_format: String,

	/// Path to the output set
	#[clap(long)]
	output_path: PathBuf,

	/// In which format the dataset will be written as.
	///
	/// If it is not a known standard, it will pick a script in scripts directory.
	///
	/// Standard supported:
	/// - coco-1-0
	/// - yolo-1-1
	///
	/// To use a custom reader, prefix it with custom:<NAME>
	/// The name needs to be in the settings
	#[clap(long, verbatim_doc_comment)]
	output_format: String,

	/// Datasets version (this is for metadata purposes, can be ignored)
	#[clap(short, long, default_value = "1")]
	version: String,

	/// Mapping for the classes if needed
	#[clap(short, long)]
	mapping_file: PathBuf,
}

impl Untrack {
	pub async fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		let _ = settings;
		let iformat: Serializer = Format::try_from(self.input_format.clone())?.into();
		let content = match read_to_string(&self.mapping_file) {
			Ok(content) => content,
			Err(err) => {
				return log_err!(format!(
					"Could not read file '{}': {err}",
					&self.mapping_file.display()
				));
			},
		};
		let mapping: ClassMapper = match toml::from_str(&content) {
			Ok(content) => content,
			Err(err) => {
				return log_err!(format!(
					"Could not deserialize toml mapping file '{}': {err}",
					&self.mapping_file.display()
				));
			},
		};
		let datasets = iformat.read(
			&self.input_path,
			None,
			self.version.clone(),
			settings.interpreters(),
		)?;
		let new_ds = datasets.remap(&mapping)?;
		let oformat: Serializer = Format::try_from(self.input_format.clone())?.into();
		oformat.write(&self.output_path, &new_ds, settings.interpreters())?;
		Ok(())
	}
}
