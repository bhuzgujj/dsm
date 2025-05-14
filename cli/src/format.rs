use clap::ValueEnum;
use serializer::Formatter;

#[derive(ValueEnum, Debug, Clone)]
pub enum Format {
	/// Format from Coco version 1.0
	Coco_1_0,
	/// Format from Yolo version 1.1
	Yolo_1_1
}

impl Into<Formatter> for Format {
	fn into(self) -> Formatter {
		match self { 
			Format::Coco_1_0 => Formatter::Coco1_0,
			Format::Yolo_1_1 => Formatter::Yolo1_1
		}
	}
}