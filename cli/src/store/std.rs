use std::fs::{copy, create_dir_all};
use crate::format::Format;
use clap::Args;
use interfaces::models::Settings;
use serializer::Formatter;
use std::path::PathBuf;
use storage::Storage;

/// Parse the datasets in a known standard format
#[derive(Args, Debug)]
pub struct Std {
	/// In which format the datasets will be read as
	formats: Format,

	/// Root directory of the datasets
	path: String,

	/// Datasets registered name
	#[clap(long)]
	name: Option<String>,

	/// Datasets registered version
	#[clap(short, long, default_value_t = 1)]
	version: u32,
}

impl Std {
	pub fn execute(&self, settings: &Settings) -> anyhow::Result<()> {
		let path = PathBuf::from(self.path.clone());
		let name = self.name.clone().unwrap_or(path.file_name().expect("Could not get the directory name").to_string_lossy().into());
		let formatter: Formatter = self.formats.clone().into();
		let mut dataset = formatter.read(&path, name.clone(), self.version)?;
		let version: u32 = dataset.get_version();
		let image_store_directory = settings.get_image_store();
		let dataset_dir = PathBuf::from(name.clone());
		for (set_name, entries) in dataset.get_mut_entries() {
			let subset = format!("{}-v{}", &set_name, version);
			let rel_copy_destination = dataset_dir.join(&subset);
			let copy_destination = image_store_directory.join(&rel_copy_destination);
			if copy_destination.exists() {
				return Err(anyhow::anyhow!(format!("{} already exists", copy_destination.to_string_lossy())));
			}
			dbg!(&rel_copy_destination);
			create_dir_all(&copy_destination)?;
			for entry in entries {
				let image_name = entry.get_image_path().file_name()
					.expect("Could not get image name")
					.to_string_lossy()
					.to_string();
				let image_destination = copy_destination.join(&image_name);
				copy(&entry.get_image_path(), &image_destination)?;
				entry.set_image_path(rel_copy_destination.join(image_name));
			}
		}
		Storage::Local(settings.get_dataset_database()).store(&dataset)
	}
}

