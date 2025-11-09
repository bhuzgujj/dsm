use bhomz::log_err;
use interfaces::models::datasets::Dataset;
use interfaces::models::interpreter::Interpreter;
use interfaces::models::DataFormat;
use log::debug;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;

mod coco_1_0;
mod yolo_1_1;

#[derive(Debug, Clone)]
pub enum Serializer {
	Yolo1_1,
	Coco1_0,
	Custom(String),
}

impl Display for Serializer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Serializer::Yolo1_1 => write!(f, "Yolo 1.1"),
			Serializer::Coco1_0 => write!(f, "Coco 1.0"),
			Serializer::Custom(name) => write!(f, "Custom({name})"),
		}
	}
}

impl Serializer {
	pub fn read(
		&self,
		path: &Path,
		name: Option<String>,
		version: String,
		interpreters: &HashMap<String, Interpreter>,
	) -> anyhow::Result<Dataset> {
		debug!(
			"Reading files format '{}' at '{}'",
			&path.canonicalize()?.to_str().unwrap_or("<Unknown path>"),
			&self
		);
		match &self {
			Serializer::Yolo1_1 => yolo_1_1::read(path, name, version, self.clone().into()),
			Serializer::Coco1_0 => coco_1_0::read(path, name, version, self.clone().into()),
			Serializer::Custom(interpreter) => {
				if let Some(interpreter) = interpreters.get(interpreter) {
					interpreter.read(path, name, version)
				} else {
					log_err!("Unknown interpreter: {}", interpreter)
				}
			},
		}
	}

	pub fn write(
		&self,
		output: &Path,
		datasets: &Dataset,
		interpreters: &HashMap<String, Interpreter>,
	) -> anyhow::Result<()> {
		match self {
			Serializer::Yolo1_1 => yolo_1_1::write(output, datasets),
			Serializer::Coco1_0 => coco_1_0::write(output, datasets),
			Serializer::Custom(interpreter) => {
				if let Some(interpreter) = interpreters.get(interpreter) {
					interpreter.write(output, datasets)
				} else {
					log_err!("Unknown interpreter: {interpreter}")
				}
			},
		}
	}

	pub fn to_data_form(&self) -> DataFormat {
		match self {
			Serializer::Yolo1_1 => DataFormat::Yolo1_1,
			Serializer::Coco1_0 => DataFormat::Coco1_0,
			Serializer::Custom(custom) => DataFormat::Custom(custom.clone()),
		}
	}
}

impl From<Serializer> for DataFormat {
	fn from(form: Serializer) -> Self {
		match form {
			Serializer::Yolo1_1 => DataFormat::Yolo1_1,
			Serializer::Coco1_0 => DataFormat::Coco1_0,
			Serializer::Custom(f) => DataFormat::Custom(f.clone()),
		}
	}
}

impl From<DataFormat> for Serializer {
	fn from(form: DataFormat) -> Self {
		match form {
			DataFormat::Yolo1_1 => Serializer::Yolo1_1,
			DataFormat::Coco1_0 => Serializer::Coco1_0,
			DataFormat::Custom(f) => Serializer::Custom(f.clone()),
		}
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	#[test]
	fn test_case_covered_for_all() {
		let mut parsers = HashSet::new();
		if let Ok(src_dir) = std::fs::read_dir("src") {
			for file in src_dir {
				if let Ok(file) = file {
					if file.path().is_dir() {
						parsers.insert(file.file_name());
					}
				}
			}
		}
		if let Ok(resource_dir) = std::fs::read_dir("src") {
			for file in resource_dir {
				if let Ok(file) = file {
					if file.path().is_dir() {
						let file_name = file.file_name();
						assert!(&parsers.contains(&file_name));
						parsers.remove(&file_name);
					}
				}
			}
		}
		assert_eq!(parsers.len(), 0);
	}
}

#[cfg(test)]
macro_rules! test_metadata {
	($dataset_name:ident) => {
		const BASE_DIR: &str = concat!("./resources/", stringify!($dataset_name));

		#[test]
		fn rectangle_only_correct_read_metadata() {
			let base_path = std::path::PathBuf::from(BASE_DIR).join("rectangle-only_correct");
			let result: interfaces::models::datasets::Dataset = read(
				base_path.join("input").as_path(),
				None,
				String::from("1"),
				FORMATTER,
			)
			.unwrap();
			let expected = std::fs::read_to_string(base_path.join("expected.json")).unwrap();
			let expected_dataset =
				serde_json::from_str::<interfaces::models::datasets::Dataset>(&expected).unwrap();
			assert_eq!(expected_dataset.entries(), result.entries());
			assert_eq!(expected_dataset.metadata(), result.metadata());
		}
	};
}

#[cfg(test)]
mod yolo_1_0_test {
	use crate::yolo_1_1::*;
	use std::env::temp_dir;
	use std::fs::{create_dir_all, remove_dir_all};

	test_metadata!(yolo_1_1);

	#[test]
	fn rectangle_only_correct_entry_write() {
		let tmp_dir =
			temp_dir().join(".dsm.unit-test.yolo_1_0_test.rectangle_only_correct_entry_write");
		if tmp_dir.exists() {
			remove_dir_all(&tmp_dir).unwrap();
		}
		create_dir_all(tmp_dir.clone()).unwrap();
		let base_path = std::path::PathBuf::from(BASE_DIR).join("rectangle-only_correct");
		let expected = std::fs::read_to_string(base_path.join("expected.json")).unwrap();
		let expected_dataset =
			serde_json::from_str::<interfaces::models::datasets::Dataset>(&expected).unwrap();
		write(tmp_dir.as_path(), &expected_dataset).unwrap();
		let result = std::fs::read_dir(tmp_dir.join("obj_train_data").as_path());
		assert!(&result.is_ok());
		for file in result.unwrap() {
			if let Ok(file) = file {
				let current_file = base_path
					.join("input")
					.join("obj_train_data")
					.join(file.file_name().to_str().unwrap());
				if file.path().is_file() {
					if file.path().extension().is_some_and(|ext| ext == "txt") {
						let expected_annotation = std::fs::read_to_string(file.path()).unwrap();
						let current_annotation = std::fs::read_to_string(current_file).unwrap();
						assert_eq!(expected_annotation.trim(), current_annotation.trim());
					} else {
						assert!(
							current_file.exists(),
							"'{}' should exist",
							current_file.display()
						);
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod coco_1_0_test {
	use crate::coco_1_0::*;

	test_metadata!(coco_1_0);
}
