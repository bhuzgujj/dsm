use interfaces::log_err;
use serializer::DataForm;

#[derive(Debug, Clone)]
pub enum Format {
    /// Format from Coco version 1.0
    Coco1_0,

    /// Format from Yolo version 1.1
    Yolo1_1,

    Custom(String),
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

impl TryFrom<String> for Format {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "coco-1-0" => Ok(Format::Coco1_0),
            "yolo-1-1" => Ok(Format::Yolo1_1),
            s => {
                let splits: Vec<&str> = s.split(":").collect();
                if splits.len() > 1 {
                    Ok(Format::Custom(splits[1..].join(":").to_string()))
                } else {
                    log_err!("Unknown standard: {s}")
                }
            }
        }
    }
}
