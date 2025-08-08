use interfaces::{
	models::{DsmDataForm, Settings},
	paths::read_from_file,
};
use serializer::DataForm;
use storage::{Storage, FORMAT_FILE_NAME};

pub async fn migrate(settings: &Settings) -> anyhow::Result<()> {
	let store_path = settings.store_path();
	let storage = Storage::local(settings);
	for dataset_directory in store_path.read_dir()? {
		let dataset_dir = dataset_directory?;
		let name = dataset_dir.file_name().to_str().unwrap().to_string();
		for version_directory in dataset_dir.path().read_dir()? {
			let version_dir = version_directory?;
			let version = version_dir.file_name().to_str().unwrap().to_string();
			let content = read_from_file(&version_dir.path().join(FORMAT_FILE_NAME))?;
			let format = DsmDataForm::try_from(content)?;
			let formatter = DataForm::from(format);
			let dsm = formatter.read(
				&version_dir.path(),
				Some(name.clone()),
				version,
				settings.interpreters(),
			)?;
			storage.ledge(&dsm).await?;
		}
	}
	Ok(())
}
