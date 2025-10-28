use std::{fs, path::Path, process::Command};
use bhomz::log_err;
use serde::{Deserialize, Serialize};

use crate::{models::datasets::Dataset, paths::write_to_file};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Interpreter {
	process: String,
	args: Vec<String>,
}

impl Interpreter {
	pub fn read(
		&self,
		path: &Path,
		name: Option<String>,
		version: String,
	) -> anyhow::Result<Dataset> {
		let new_name = name.clone().unwrap_or(
			path.file_name()
				.expect("Could not get the directory name")
				.to_string_lossy()
				.into(),
		);
		let result = Command::new(&self.process)
			.args(&self.args)
			.arg("read")
			.arg(path)
			.arg(new_name)
			.arg(version)
			.output()?;

		let output = String::from_utf8(result.stdout)?;
		match serde_json::from_str::<Dataset>(&output) {
			Ok(sets) => Ok(sets),
			Err(_) => log_err!(format!("Error from the custom interpreter: {output}")),
		}
	}

	pub fn write(&self, output: &Path, datasets: &Dataset) -> anyhow::Result<()> {
		fs::create_dir_all(output)?;
		let metadata_path = output.join("dsm_metadata.json");
		let content = serde_json::to_string(datasets)?;
		write_to_file(&metadata_path, content, true, true)?;
		let result = Command::new(&self.process)
			.args(&self.args)
			.arg("write")
			.arg(output)
			.arg(metadata_path)
			.output()?;

		let output = String::from_utf8(result.stdout)?;
		if !output.is_empty() {
			return log_err!(format!("Error from the custom interpreter: {output}"));
		}
		Ok(())
	}
}
