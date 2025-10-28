use interfaces::models::datasets::Dataset;
use interfaces::models::interpreter::Interpreter;
use interfaces::models::DataFormat;
use log::debug;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;
use bhomz::log_err;

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
