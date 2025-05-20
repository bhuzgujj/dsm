use std::fmt::Display;
use std::path::PathBuf;
use log::debug;
use interfaces::models::DsmSets;

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
	pub fn read(&self, path: &PathBuf, name: Option<String>, version: Option<u32>) -> anyhow::Result<Vec<DsmSets>> {
		debug!("Reading files format '{}' at '{}'", &path.canonicalize()?.to_str().unwrap_or("<Unknown path>"), &self);
		match &self {
			DataForm::Yolo1_1 => yolo_1_1::read(path, name, version.unwrap_or(1), self.clone().into()),
			DataForm::Coco1_0 => coco_1_0::read(path, name, version, self.clone().into()),
			DataForm::Custom(_) => todo!("Custom format are not supported yet")
		}
	}

	pub fn write(&self, path: &PathBuf, store_path: &PathBuf, datasets: &DsmSets) -> anyhow::Result<()> {
		debug!("Reading files format '{}' at '{}'", &path.canonicalize()?.to_str().unwrap_or("<Unknown path>"), &self);
		match self {
			DataForm::Yolo1_1 => yolo_1_1::write(store_path, path, datasets),
			DataForm::Coco1_0 => coco_1_0::write(store_path, path, datasets),
			DataForm::Custom(_) => todo!("Custom format are not supported yet")
		}
	}

	pub fn to_data_form(&self) -> interfaces::models::DataForm {
		match self {
			DataForm::Yolo1_1 => interfaces::models::DataForm::Yolo1_1,
			DataForm::Coco1_0 => interfaces::models::DataForm::Coco1_0,
			DataForm::Custom(_) => todo!("Custom format are not supported yet")
		}
	}
}

impl From<DataForm> for interfaces::models::DataForm {
	fn from(val: DataForm) -> Self {
		match val {
			DataForm::Yolo1_1 => interfaces::models::DataForm::Yolo1_1,
			DataForm::Coco1_0 => interfaces::models::DataForm::Coco1_0,
			DataForm::Custom(f) => interfaces::models::DataForm::Custom(f.clone())
		}
	}
}