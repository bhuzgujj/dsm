use std::fmt::Display;

use anyhow::Error;
use serde::{Deserialize, Serialize};

const PREFIX: &str = "Custom(";
const SUFIX: &str = ")";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataFormat {
	Yolo1_1,
	Coco1_0,
	Custom(String),
}

impl Display for DataFormat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DataFormat::Yolo1_1 => f.write_str("Yolo 1.1"),
			DataFormat::Coco1_0 => f.write_str("Coco 1.0"),
			DataFormat::Custom(c) => f.write_str(format!("{PREFIX}{c}{SUFIX}").as_str()),
		}
	}
}

impl TryFrom<String> for DataFormat {
	type Error = anyhow::Error;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		let trimmed = value.trim();
		match trimmed {
			"Yolo 1.1" => Ok(DataFormat::Yolo1_1),
			"Coco 1.0" => Ok(DataFormat::Coco1_0),
			_ => {
				if trimmed.starts_with(PREFIX) && trimmed.ends_with(SUFIX) {
					let start = PREFIX.len();
					let end = trimmed.len() - SUFIX.len();
					Ok(DataFormat::Custom(trimmed[start..end].to_string()))
				} else {
					Err(Error::msg(format!("Invalid formats {value}")))
				}
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::models::DataFormat;

	#[test]
	fn try_from_ok() {
		for case in [
			DataFormat::Yolo1_1,
			DataFormat::Coco1_0,
			DataFormat::Custom(String::from("value")),
		] {
			let stringified = case.to_string();
			let parsed = DataFormat::try_from(stringified);
			assert!(parsed.is_ok());
			let parsed_unwrapped = parsed.unwrap();
			assert_eq!(parsed_unwrapped, case);
		}
	}
}
