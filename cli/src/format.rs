use clap::ValueEnum;
use serializer::DataForm;

#[derive(ValueEnum, Debug, Clone)]
pub enum Format {
	/// Format from Coco version 1.0
	#[allow(non_camel_case_types)]
	Coco_1_0,

	/// Format from Yolo version 1.1
	#[allow(non_camel_case_types)]
	Yolo_1_1
}

impl Into<DataForm> for Format {
	fn into(self) -> DataForm {
		match self { 
			Format::Coco_1_0 => DataForm::Coco1_0,
			Format::Yolo_1_1 => DataForm::Yolo1_1
		}
	}
}