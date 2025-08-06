use serializer::DataForm;

#[derive(Debug, Clone)]
pub enum DatasetFormat {
	Coco1_0,
	Yolo1_1,
	Custom(String),
}

impl From<DatasetFormat> for DataForm {
	fn from(val: DatasetFormat) -> Self {
		match val {
			DatasetFormat::Coco1_0 => DataForm::Coco1_0,
			DatasetFormat::Yolo1_1 => DataForm::Yolo1_1,
			DatasetFormat::Custom(s) => DataForm::Custom(s),
		}
	}
}

impl From<String> for DatasetFormat {
	fn from(val: String) -> Self {
		match val.to_lowercase().as_str() {
			"coco-1-0" => DatasetFormat::Coco1_0,
			"yolo-1-1" => DatasetFormat::Yolo1_1,
			s => DatasetFormat::Custom(s.to_string()),
		}
	}
}
