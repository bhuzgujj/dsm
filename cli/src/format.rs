use serializer::DataForm;

#[derive(Debug, Clone)]
pub enum Format {
	/// Format from Coco version 1.0
	Coco1_0,

	/// Format from Yolo version 1.1
	Yolo1_1,

	Custom(String)
}

impl From<Format> for DataForm {
	fn from(val: Format) -> Self {
		match val { 
			Format::Coco1_0 => DataForm::Coco1_0,
			Format::Yolo1_1 => DataForm::Yolo1_1,
			Format::Custom(s) => DataForm::Custom(s),
		}
	}
}

impl From<String> for Format {
	fn from(val: String) -> Self {
		match val.to_lowercase().as_str() { 
			"coco-1-0" => Format::Coco1_0,
			"yolo-1-1" => Format::Yolo1_1,
			s => Format::Custom(s.to_string()),
		}
	}
}