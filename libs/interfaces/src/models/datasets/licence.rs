use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DsmLicense {
	pub name: String,
	pub url: String,
}

impl DsmLicense {
	pub fn new(name: String, url: String) -> Self {
		Self { name, url }
	}
}