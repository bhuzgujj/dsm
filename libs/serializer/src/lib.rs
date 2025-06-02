use std::fmt::Display;
use std::path::Path;
use log::debug;
use interfaces::models::{DsmDataForm, DsmSets};

mod yolo_1_1;
mod coco_1_0;

#[derive(Debug, Clone)]
pub enum DataForm {
	Yolo1_1,
	Coco1_0,
	Custom(String)
}

impl Display for DataForm {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DataForm::Yolo1_1 => write!(f, "Yolo 1.1"),
			DataForm::Coco1_0 => write!(f, "Coco 1.0"),
			DataForm::Custom(name) => write!(f, "Custom({})", name)
		}
	}
}

impl DataForm {
	pub fn read(&self, path: &Path, name: Option<String>, version: String) -> anyhow::Result<Vec<DsmSets>> {
		debug!("Reading files format '{}' at '{}'", &path.canonicalize()?.to_str().unwrap_or("<Unknown path>"), &self);
		match &self {
			DataForm::Yolo1_1 => yolo_1_1::read(path, name, version, self.clone().into()),
			DataForm::Coco1_0 => coco_1_0::read(path, name, version, self.clone().into()),
			DataForm::Custom(_) => todo!("Custom format are not supported yet")
		}
	}

	pub fn write(&self, output: &Path, input: &Path, datasets: &DsmSets) -> anyhow::Result<()> {
		debug!("Reading files format '{}' at '{}'", &input.canonicalize()?.to_str().unwrap_or("<Unknown path>"), &self);
		match self {
			DataForm::Yolo1_1 => yolo_1_1::write(input, output, datasets),
			DataForm::Coco1_0 => coco_1_0::write(input, output, datasets),
			DataForm::Custom(_) => todo!("Custom format are not supported yet")
		}
	}

	pub fn to_data_form(&self) -> DsmDataForm {
		match self {
			DataForm::Yolo1_1 => DsmDataForm::Yolo1_1,
			DataForm::Coco1_0 => DsmDataForm::Coco1_0,
			DataForm::Custom(_) => todo!("Custom format are not supported yet")
		}
	}
}

impl From<DataForm> for DsmDataForm {
	fn from(form: DataForm) -> Self {
		match form {
			DataForm::Yolo1_1 => DsmDataForm::Yolo1_1,
			DataForm::Coco1_0 => DsmDataForm::Coco1_0,
			DataForm::Custom(f) => DsmDataForm::Custom(f.clone())
		}
	}
}