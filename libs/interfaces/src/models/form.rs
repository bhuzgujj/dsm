use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DsmDataForm {
	Yolo1_1,
	Coco1_0,
	Custom(String),
	Unknown
}