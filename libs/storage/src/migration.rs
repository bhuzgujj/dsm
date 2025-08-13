use crate::{local::FORMAT_FILE_NAME, Storage};
use colored::Colorize;
use interfaces::{
	models::{DataFormat, Settings},
	paths::read_from_file,
};
use log::warn;
use serializer::Serializer;

pub async fn migrate(settings: &Settings) -> anyhow::Result<()> {
	let store_path = settings.store_path();
	let storage = Storage::local(settings);
	for dataset_directory in store_path.read_dir()? {
		let dataset_dir = dataset_directory?;
		let name = dataset_dir.file_name().to_str().unwrap().to_string();
		for version_directory in dataset_dir.path().read_dir()? {
			let version_dir = version_directory?;
			match version_dir.file_name().to_str() {
				Some(version_dir_name) => {
					let version = version_dir_name.to_string();
					let content = read_from_file(&version_dir.path().join(FORMAT_FILE_NAME))?;
					let format = DataFormat::try_from(content)?;
					let formatter = Serializer::from(format);
					let dsm = formatter.read(
						&version_dir.path(),
						Some(name.clone()),
						version,
						settings.interpreters(),
					)?;
					storage.ledge(&dsm).await?;
				},
				None => {
					let msg = format!(
						"Could not read the name of the directory at '{}'",
						version_dir.path().display()
					);
					warn!("{msg}");
					println!("{}", msg.yellow());
					continue;
				},
			}
		}
	}
	Ok(())
}
