use std::fmt::Display;

use anyhow::Error;
use serde::{Deserialize, Serialize};

const PREFIX: &str = "Custom(";
const SUFIX: &str = ")";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DsmDataForm {
	Yolo1_1,
	Coco1_0,
	Custom(String),
}

impl Display for DsmDataForm {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DsmDataForm::Yolo1_1 => f.write_str("Yolo 1.1"),
			DsmDataForm::Coco1_0 => f.write_str("Coco 1.0"),
			DsmDataForm::Custom(c) => f.write_str(format!("{PREFIX}{c}{SUFIX}").as_str()),
		}
	}
}

impl TryFrom<String> for DsmDataForm {
	type Error = anyhow::Error;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		let trimmed = value.trim();
		match trimmed {
			"Yolo 1.1" => Ok(DsmDataForm::Yolo1_1),
			"Coco 1.0" => Ok(DsmDataForm::Coco1_0),
			_ => {
				if trimmed.starts_with(PREFIX) && trimmed.ends_with(SUFIX) {
					let start = PREFIX.len();
					let end = trimmed.len() - SUFIX.len();
					Ok(DsmDataForm::Custom(trimmed[start..end].to_string()))
				} else {
					Err(Error::msg(format!("Invalid formats {value}")))
				}
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::models::DsmDataForm;

	#[test]
	fn try_from_ok() {
		for case in [
			DsmDataForm::Yolo1_1,
			DsmDataForm::Coco1_0,
			DsmDataForm::Custom(String::from("value")),
		] {
			let stringified = case.to_string();
			let parsed = DsmDataForm::try_from(stringified);
			assert!(parsed.is_ok());
			let parsed_unwrapped = parsed.unwrap();
			assert_eq!(parsed_unwrapped, case);
		}
	}
}
