use std::fmt::Display;
use std::path::PathBuf;
use log::debug;
use interfaces::models::Datasets;

mod yolo_1_1;

#[derive(Debug)]
pub enum Formatter {
	Yolo1_1,
	Coco1_0,
	Custom(String)
}

impl Display for Formatter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Formatter::Yolo1_1 => write!(f, "Yolo 1.1"),
			Formatter::Coco1_0 => write!(f, "Coco 1.0"),
			Formatter::Custom(name) => write!(f, "Custom({})", name)
		}
	}
}

impl Formatter {
	pub fn read(&self, path: &PathBuf, name: String, version: u32) -> anyhow::Result<Datasets> {
		debug!("Reading datasets format '{}' at '{}'", &path.canonicalize()?.to_str().unwrap_or("<Unknown path>"), &self);
		match self {
			Formatter::Yolo1_1 => yolo_1_1::read(&path, name, version),
			Formatter::Coco1_0 => todo!("Coco 1.0 format is not supported yet"),
			Formatter::Custom(name) => todo!("Custom format are not supported yet")
		}
	}

	pub fn write(&self, path: &PathBuf, store_path: &PathBuf, datasets: &Datasets) -> anyhow::Result<()> {
		debug!("Reading datasets format '{}' at '{}'", &path.canonicalize()?.to_str().unwrap_or("<Unknown path>"), &self);
		match self {
			Formatter::Yolo1_1 => yolo_1_1::write(store_path, path, datasets),
			Formatter::Coco1_0 => todo!("Coco 1.0 format is not supported yet"),
			Formatter::Custom(name) => todo!("Custom format are not supported yet")
		}
	}
}