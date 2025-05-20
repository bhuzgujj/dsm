use clap::builder::PossibleValue;
use clap::ValueEnum;
use serializer::DataForm;

#[derive(Debug, Clone)]
pub enum Format {
	/// Format from Coco version 1.0
	#[allow(non_camel_case_types)]
	Coco_1_0,

	/// Format from Yolo version 1.1
	#[allow(non_camel_case_types)]
	Yolo_1_1,

	Custom(String)
}

impl From<Format> for DataForm {
	fn from(val: Format) -> Self {
		match val { 
			Format::Coco_1_0 => DataForm::Coco1_0,
			Format::Yolo_1_1 => DataForm::Yolo1_1,
			Format::Custom(s) => DataForm::Custom(s),
		}
	}
}

impl ValueEnum for Format {
	fn value_variants<'a>() -> &'a [Self] {
		&[Format::Coco_1_0, Format::Yolo_1_1]
	}

	fn from_str(input: &str, _ignore_case: bool) -> Result<Self, String> {
		match input {
			"coco-1-0" => Ok(Format::Coco_1_0),
			"yolo-1-1" => Ok(Format::Yolo_1_1),
			_ => Ok(Format::Custom(input.into()))
		}
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Format::Coco_1_0 => Some(PossibleValue::new("coco-1-1")),
			Format::Yolo_1_1 => Some(PossibleValue::new("yolo-1-1")),
			Format::Custom(_) => Some(PossibleValue::new("<PATTH-TO-SCRIPT>"))
		}
	}
}