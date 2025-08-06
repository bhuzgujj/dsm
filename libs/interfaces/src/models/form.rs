use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DsmDataForm {
    Yolo1_1,
    Coco1_0,
    Custom(String),
    Unknown,
}

impl Display for DsmDataForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DsmDataForm::Yolo1_1 => f.write_str("Yolo 1.1"),
            DsmDataForm::Coco1_0 => f.write_str("Coco 1.0"),
            DsmDataForm::Custom(c) => f.write_str(format!("Custom({})", c).as_str()),
            DsmDataForm::Unknown => f.write_str("<Unknown>"),
        }
    }
}
